use {
    crate::{
        apis::vaults_api::{GetPagedVaultAccountsError, GetPagedVaultAccountsParams},
        models::VaultAccountsPagedResponse,
        Client,
    },
    futures::{future::BoxFuture, stream::FuturesUnordered, FutureExt, Stream, StreamExt},
    std::{
        pin::Pin,
        sync::Arc,
        task::{Context, Poll},
    },
};

type VaultResult =
    std::result::Result<VaultAccountsPagedResponse, crate::apis::Error<GetPagedVaultAccountsError>>;

pub struct VaultStream {
    client: Arc<Client>,
    batch: u16,
    after: String,
    init: bool,
    fut: FuturesUnordered<BoxFuture<'static, VaultResult>>,
}

/// Stream all vault accounts in batches
impl VaultStream {
    pub fn new(client: Arc<Client>, batch: u16) -> Self {
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
