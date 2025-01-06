use {
    crate::{
        apis::{
            transactions_api::{GetTransactionsError, GetTransactionsParams},
            vaults_api::{GetPagedVaultAccountsError, GetPagedVaultAccountsParams},
            ResponseContent,
        },
        models::{self, ErrorSchema, TransactionResponse, VaultAccountsPagedResponse},
        Client, Epoch, FireblocksError, ParamError, QueryParams,
    },
    anyhow::anyhow,
    chrono::{offset::LocalResult, TimeZone, Utc},
    futures::{future::BoxFuture, stream::FuturesUnordered, FutureExt, Stream, StreamExt},
    std::{
        pin::Pin,
        sync::Arc,
        task::{Context, Poll},
    },
};

#[derive(Clone)]
pub struct PagedClient {
    pub client: Arc<Client>,
}

type VaultResult =
    std::result::Result<VaultAccountsPagedResponse, crate::apis::Error<GetPagedVaultAccountsError>>;

type TransactionResult = Result<Vec<TransactionResponse>, crate::apis::Error<GetTransactionsError>>;

pub struct VaultStream {
    client: Arc<Client>,
    batch: u16,
    after: String,
    init: bool,
    fut: FuturesUnordered<BoxFuture<'static, VaultResult>>,
}

/// Stream all vault accounts in batches
impl VaultStream {
    fn new(client: Arc<Client>, batch: u16) -> Self {
        Self {
            client,
            batch,
            init: false,
            after: String::new(),
            fut: FuturesUnordered::new(),
        }
    }

    fn build_params(&self) -> GetPagedVaultAccountsParams {
        GetPagedVaultAccountsParams::builder()
            .after(self.after.clone())
            .limit(self.batch.into())
            .build()
    }
}

/// Stream transactions from a vault account
pub struct TransactionStream {
    client: Arc<Client>,
    batch: u16,
    init: bool, // has the stream started?
    vault_id: i32,
    after: Epoch,
    is_source: bool, // are we streaming from source vault account or destination
    fut: FuturesUnordered<BoxFuture<'static, TransactionResult>>,
}

const fn epoch(ts: &Epoch) -> i64 {
    ts.timestamp_millis()
}

impl TransactionStream {
    fn from_source(client: Arc<Client>, batch: u16, vault_id: i32, after: Epoch) -> Self {
        Self {
            client,
            batch,
            init: false,
            vault_id,
            after,
            fut: FuturesUnordered::new(),
            is_source: true,
        }
    }

    fn from_dest(client: Arc<Client>, batch: u16, vault_id: i32, after: Epoch) -> Self {
        Self {
            client,
            batch,
            init: false,
            vault_id,
            after,
            fut: FuturesUnordered::new(),
            is_source: false,
        }
    }

    fn build_params(&self) -> GetTransactionsParams {
        let builder = GetTransactionsParams::builder()
            .limit(self.batch.into())
            .order_by("createdAt".to_owned())
            .after(epoch(&self.after).to_string())
            .sort("ASC".to_owned());
        if self.is_source {
            return builder.source_id(self.vault_id.to_string()).build();
        }
        builder.dest_id(self.vault_id.to_string()).build()
    }
}

impl Stream for VaultStream {
    type Item = VaultResult;

    #[allow(clippy::cognitive_complexity)]
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if !self.init {
            tracing::debug!("init future");
            self.init = true;
            let client = self.client.clone();
            let params = self.build_params();
            let fut =
                async move { client.vaults_api().get_paged_vault_accounts(params).await }.boxed();
            self.fut.push(fut);
            cx.waker().wake_by_ref();
            return Poll::Pending;
        }

        // Try to resolve any existing futures first
        tracing::trace!("check future poll");
        match self.fut.poll_next_unpin(cx) {
            Poll::Ready(opt) => {
                if let Some(result) = opt {
                    match result {
                        Ok(ref va) => {
                            match &va.paging {
                                None => self.after = String::new(),
                                Some(p) => self.after = p.after.clone().unwrap_or_default(),
                            };
                        }
                        Err(e) => {
                            return Poll::Ready(Some(Err(e)));
                        }
                    }
                    return Poll::Ready(Some(result));
                }
            }
            Poll::Pending => {
                tracing::trace!("still pending");
                cx.waker().wake_by_ref();
                return Poll::Pending;
            }
        };

        tracing::trace!("checking after {:#?}", self.after);
        // If there are no more pages to fetch and no pending futures, end the stream
        if self.after.is_empty() {
            return Poll::Ready(None);
        }

        let client = self.client.clone();
        let params = self.build_params();
        let fut = async move { client.vaults_api().get_paged_vault_accounts(params).await }.boxed();
        self.fut.push(fut);
        cx.waker().wake_by_ref();
        Poll::Pending
    }
}

impl Stream for TransactionStream {
    type Item = TransactionResult;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if !self.init {
            tracing::debug!("init tracing stream");
            self.init = true;
            let client = self.client.clone();
            let params = self.build_params();
            let fut =
                async move { client.transactions_api().get_transactions(params).await }.boxed();
            self.fut.push(fut);
            cx.waker().wake_by_ref();
            return Poll::Pending;
        }

        match self.fut.poll_next_unpin(cx) {
            Poll::Ready(opt) => {
                if let Some(result) = opt {
                    match result {
                        Ok(ref va) => {
                            if va.is_empty() {
                                return Poll::Ready(None);
                            }
                            if let Some(last) = va.last() {
                                tracing::trace!(
                                    "1st after {:#?} last after {:#?}",
                                    va[0].created_at,
                                    last.created_at
                                );
                                if let Some(millis) = last.created_at {
                                    let ts = match Utc.timestamp_millis_opt(millis as i64) {
                                        LocalResult::Single(dt) => dt,
                                        _ => {
                                            let entity: GetTransactionsError =
                                                GetTransactionsError::DefaultResponse(
                                                    ErrorSchema {
                                                        message: Some(format!(
                                                            "invalid timestamp {millis}"
                                                        )),
                                                        code: Some(-1.0),
                                                    },
                                                );
                                            let e = crate::apis::Error::ResponseError(
                                                ResponseContent {
                                                    status: reqwest::StatusCode::GONE,
                                                    content: String::new(),
                                                    entity: Some(entity),
                                                },
                                            );
                                            return Poll::Ready(Some(Err(e)));
                                        }
                                    };
                                    self.after = ts;
                                    // last.created_at +
                                    // chrono::Duration::milliseconds(1);
                                }
                            }
                        }
                        Err(e) => {
                            return Poll::Ready(Some(Err(e)));
                        }
                    }
                    return Poll::Ready(Some(result));
                }
            }
            Poll::Pending => {
                cx.waker().wake_by_ref();
                return Poll::Pending;
            }
        };

        let client = self.client.clone();
        let params = self.build_params();
        let fut = async move { client.transactions_api().get_transactions(params).await }.boxed();
        self.fut.push(fut);
        cx.waker().wake_by_ref();
        Poll::Pending
    }
}

impl PagedClient {
    pub const fn new(client: Arc<Client>) -> Self {
        Self { client }
    }

    /// Stream the vault accounts based on batch size
    ///
    /// ```
    /// use {
    ///     fireblocks_sdk::{Client, PagedClient},
    ///     futures::TryStreamExt,
    ///     std::sync::Arc,
    /// };
    ///
    /// async fn vault_accounts(c: Client) -> color_eyre::Result<()> {
    ///     let pc = PagedClient::new(Arc::new(c));
    ///     let mut vault_stream = pc.vaults(100);
    ///     while let Ok(Some(result)) = vault_stream.try_next().await {
    ///         tracing::info!("accounts {}", result.0.accounts.len());
    ///         tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    ///     }
    ///     Ok(())
    /// }
    /// ```
    /// see [`Client::vaults`]
    pub fn vaults(&self, batch_size: u16) -> VaultStream {
        VaultStream::new(self.client.clone(), batch_size)
    }

    /// Stream all the transactions from source vault account id and after some date
    ///
    /// Default date is 2022-04-06 if None provided
    ///
    /// ```
    /// use {
    ///     fireblocks_sdk::{Client, PagedClient},
    ///     futures::TryStreamExt,
    ///     std::sync::Arc,
    /// };
    ///
    /// async fn transactions_paged(c: Client) -> color_eyre::Result<()> {
    ///     let pc = PagedClient::new(Arc::new(c));
    ///     let mut ts = pc.transactions_from_source(0, 100, None);
    ///     while let Ok(Some(result)) = ts.try_next().await {
    ///         tracing::info!("transactions {}", result.0.len());
    ///     }
    ///     Ok(())
    /// }
    /// ```
    ///
    /// see
    /// * [`Client::transactions`]
    pub fn transactions_from_source(
        &self,
        vault_id: i32,
        batch_size: u16,
        after: Option<Epoch>,
    ) -> TransactionStream {
        let after = after.unwrap_or(Utc.with_ymd_and_hms(2022, 4, 6, 0, 1, 1).unwrap());
        TransactionStream::from_source(self.client.clone(), batch_size, vault_id, after)
    }

    ///  Stream all the transactions from destination vault account id
    ///  See [`self.transactions_from_source`]
    pub fn transactions_from_destination(
        &self,
        vault_id: i32,
        batch_size: u16,
        after: Option<Epoch>,
    ) -> TransactionStream {
        let after = after.unwrap_or(Utc.with_ymd_and_hms(2022, 4, 6, 0, 1, 1).unwrap());
        TransactionStream::from_dest(self.client.clone(), batch_size, vault_id, after)
    }
}
