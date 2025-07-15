use {
    crate::{Client, Epoch},
    chrono::{TimeZone, Utc},
    std::sync::Arc,
};

mod paged_transaction;
mod paged_vault;
pub use {paged_transaction::TransactionStream, paged_vault::VaultStream};

#[derive(Clone)]
pub struct PagedClient {
    pub client: Arc<Client>,
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
    /// async fn vault_accounts(c: Client) -> anyhow::Result<()> {
    ///     let pc = PagedClient::new(Arc::new(c));
    ///     let mut vault_stream = pc.vaults(100);
    ///     while let Ok(Some(result)) = vault_stream.try_next().await {
    ///         tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    ///     }
    ///     Ok(())
    /// }
    /// ```
    /// see [`Client::vaults`]
    pub fn vaults(&self, batch_size: u16) -> VaultStream {
        VaultStream::new(self.client.clone(), batch_size)
    }

    /// Stream all the transactions from source vault account id and after some
    /// date
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
    /// async fn transactions_paged(c: Client) -> anyhow::Result<()> {
    ///     let pc = PagedClient::new(Arc::new(c));
    ///     let mut ts = pc.transactions_from_source(0, 100, None);
    ///     while let Ok(Some(result)) = ts.try_next().await {
    ///         tracing::info!("transactions {}", result.len());
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
        #[allow(clippy::unwrap_used, clippy::or_fun_call)]
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
        #[allow(clippy::unwrap_used, clippy::or_fun_call)]
        let after = after.unwrap_or(Utc.with_ymd_and_hms(2022, 4, 6, 0, 1, 1).unwrap());
        TransactionStream::from_dest(self.client.clone(), batch_size, vault_id, after)
    }
}
