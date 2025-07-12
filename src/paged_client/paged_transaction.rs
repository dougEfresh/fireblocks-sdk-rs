use {
    crate::{
        Client,
        Epoch,
        FireblocksError,
        apis::{
            ResponseContent,
            transactions_api::{GetTransactionsError, GetTransactionsParams},
        },
        models::{self, ErrorSchema, TransactionResponse},
    },
    chrono::{TimeZone, Utc, offset::LocalResult},
    futures::{FutureExt, Stream, StreamExt, future::BoxFuture, stream::FuturesUnordered},
    std::{
        pin::Pin,
        sync::Arc,
        task::{Context, Poll},
    },
};

type TransactionResults = crate::Result<Vec<TransactionResponse>>;
const fn epoch(ts: &Epoch) -> i64 {
    ts.timestamp_millis()
}

/// Stream transactions from a vault account
pub struct TransactionStream {
    client: Arc<Client>,
    batch: u16,
    init: bool, // has the stream started?
    vault_id: i32,
    after: Epoch,
    is_source: bool, // are we streaming from source vault account or destination
    fut: FuturesUnordered<BoxFuture<'static, TransactionResults>>,
}

impl TransactionStream {
    pub fn from_source(client: Arc<Client>, batch: u16, vault_id: i32, after: Epoch) -> Self {
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

    pub fn from_dest(client: Arc<Client>, batch: u16, vault_id: i32, after: Epoch) -> Self {
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
            .status(models::TransactionStatus::Completed)
            .sort("ASC".to_owned());
        if self.is_source {
            return builder.source_id(self.vault_id.to_string()).build();
        }
        builder.dest_id(self.vault_id.to_string()).build()
    }
}

impl Stream for TransactionStream {
    type Item = TransactionResults;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if !self.init {
            tracing::debug!("init tracing stream");
            self.init = true;
            let client = self.client.clone();
            let params = self.build_params();
            let fut = async move {
                client
                    .transactions_api()
                    .get_transactions(params)
                    .await
                    .map_err(FireblocksError::FetchTransactionsError)
            }
            .boxed();
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
                                    #[allow(clippy::cast_possible_truncation)]
                                    let LocalResult::Single(ts) =
                                        Utc.timestamp_millis_opt((millis as i64) + 1)
                                    else {
                                        let entity: GetTransactionsError =
                                            GetTransactionsError::DefaultResponse(ErrorSchema {
                                                message: Some(format!(
                                                    "invalid timestamp {millis}"
                                                )),
                                                code: Some(-1.0),
                                            });
                                        let e =
                                            crate::apis::Error::ResponseError(ResponseContent {
                                                status: reqwest::StatusCode::GONE,
                                                content: String::new(),
                                                entity: Some(entity),
                                            });
                                        return Poll::Ready(Some(Err(
                                            FireblocksError::FetchTransactionsError(e),
                                        )));
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
        }

        let client = self.client.clone();
        let params = self.build_params();
        let fut = async move {
            client
                .transactions_api()
                .get_transactions(params)
                .await
                .map_err(FireblocksError::FetchTransactionsError)
        }
        .boxed();
        self.fut.push(fut);
        cx.waker().wake_by_ref();
        Poll::Pending
    }
}
