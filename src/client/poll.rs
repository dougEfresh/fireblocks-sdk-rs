use {
    super::Client,
    crate::models::{TransactionResponse, TransactionStatus},
    std::{ops::Add, time::Duration},
    tracing::debug,
};

impl Client {
    /// Pool transaction until
    /// * [`TransactionStatus::Failed`]
    /// * [`TransactionStatus::Completed`]
    /// * [`TransactionStatus::Blocked`]
    /// * [`TransactionStatus::Rejected`]
    /// * [`TransactionStatus::Cancelling`]
    /// * [`TransactionStatus::Cancelled`]
    ///
    /// [getTransaction](https://docs.fireblocks.com/api/swagger-ui/#/Transactions/getTransaction)
    #[tracing::instrument(level = "debug", skip(self, callback))]
    pub async fn poll_transaction(
        &self,
        id: &str,
        timeout: std::time::Duration,
        interval: std::time::Duration,
        callback: impl Fn(&TransactionResponse) + Send + Sync,
    ) -> crate::Result<TransactionResponse> {
        let mut total_time = Duration::from_millis(0);
        loop {
            if let Ok(result) = self.get_transaction(id).await {
                let status = &result.status;
                debug!("status {:#?}", status);
                #[allow(clippy::match_same_arms)]
                match status {
                    TransactionStatus::Blocked => break,
                    TransactionStatus::Cancelled => break,
                    TransactionStatus::Cancelling => break,
                    TransactionStatus::Completed => break,
                    TransactionStatus::Confirming => break,
                    TransactionStatus::Failed => break,
                    TransactionStatus::Rejected => break,
                    _ => {
                        callback(&result);
                    }
                }
            }
            tokio::time::sleep(interval).await;
            total_time = total_time.add(interval);
            if total_time > timeout {
                break;
            }
        }
        self.get_transaction(id).await
    }
}
