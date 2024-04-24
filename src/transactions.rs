use crate::types::{
  CreateTransactionResponse, DestinationTransferPeerPath, PeerType, Transaction, TransactionArguments,
  TransactionOperation, TransactionStatus, TransferPeerPath,
};
use crate::Client;
use bigdecimal::BigDecimal;
use std::borrow::Borrow;
use std::fmt::{Debug, Display};
use std::ops::Add;
use tokio::time;
use tracing::debug;

impl Client {
  /// Query transactions
  ///
  /// [getTransactions](https://docs.fireblocks.com/api/swagger-ui/#/Transactions/getTransactions)
  ///
  #[tracing::instrument(level = "debug", skip(self, options))]
  pub async fn transactions<I, K, V>(&self, options: I) -> crate::Result<Vec<Transaction>>
  where
    I: IntoIterator,
    I::Item: Borrow<(K, V)>,
    K: AsRef<str>,
    V: AsRef<str>,
  {
    let u = self.build_url_params("transactions", Some(options))?.0;
    self.get(u).await
  }

  /// Create a transaction
  ///
  /// [createTransaction](https://docs.fireblocks.com/api/swagger-ui/#/Transactions/createTransaction)
  #[tracing::instrument(level = "debug", skip(self))]
  pub async fn create_transaction(&self, args: &TransactionArguments) -> crate::Result<CreateTransactionResponse> {
    let u = self.build_url("transactions")?.0;
    self.post(u, Some(args)).await
  }

  /// Create a vault-to-vault transaction
  ///
  /// [createTransaction](https://docs.fireblocks.com/api/swagger-ui/#/Transactions/createTransaction)
  #[tracing::instrument(level = "debug", skip(self))]
  pub async fn create_transaction_vault<T>(
    &self,
    source_vault: i32,
    destination_vault: i32,
    asset_id: T,
    amount: BigDecimal,
    note: Option<&str>,
  ) -> crate::Result<CreateTransactionResponse>
  where
    T: AsRef<str> + Debug + Display,
  {
    let args = &TransactionArguments {
      asset_id: format!("{asset_id}"),
      operation: TransactionOperation::TRANSFER,
      source: TransferPeerPath {
        id: Some(source_vault.to_string()),
        peer_type: PeerType::VAULT_ACCOUNT,
        name: String::new(),
        sub_type: None,
        virtual_type: None,
        virtual_id: None,
        wallet_id: None,
      },
      destination: Some(DestinationTransferPeerPath {
        peer_type: PeerType::VAULT_ACCOUNT,
        id: destination_vault.to_string(),
        wallet_id: None,
        virtual_id: None,
        virtual_type: None,
        one_time_address: None,
      }),
      amount: amount.to_string(),
      gas_price: None,
      gas_limit: None,
      note: note.unwrap_or("created by fireblocks-sdk for rust").to_string(),
    };
    self.create_transaction(args).await
  }

  /// Get a transaction by id
  ///
  /// [getTransaction](https://docs.fireblocks.com/api/swagger-ui/#/Transactions/getTransaction)
  #[tracing::instrument(level = "debug", skip(self))]
  pub async fn get_transaction(&self, id: &str) -> crate::Result<Transaction> {
    let u = self.build_url(&format!("transactions/{id}"))?.0;
    self.get(u).await
  }

  /// Pool transaction until
  /// * [`TransactionStatus::FAILED`]
  /// * [`TransactionStatus::COMPLETED`]
  /// * [`TransactionStatus::BROADCASTING`]
  /// * [`TransactionStatus::BLOCKED`]
  /// * [`TransactionStatus::TIMEOUT`]
  /// * [`TransactionStatus::CANCELLED`]
  /// * [`TransactionStatus::CANCELLING`]
  /// * [`TransactionStatus::CONFIRMING`]
  ///
  /// [getTransaction](https://docs.fireblocks.com/api/swagger-ui/#/Transactions/getTransaction)
  #[tracing::instrument(level = "debug", skip(self))]
  pub async fn poll_transaction(
    &self,
    id: &str,
    timeout: time::Duration,
    interval: time::Duration,
  ) -> crate::Result<Transaction> {
    let u = self.build_url(&format!("transactions/{id}"))?.0;
    let mut total_time = time::Duration::from_millis(0);
    loop {
      if let Ok(result) = self.get::<Transaction>(u.clone()).await {
        let status = result.0.status;
        debug!("status {:#?}", status);
        #[allow(clippy::match_same_arms)]
        match status {
          TransactionStatus::BLOCKED => break,
          TransactionStatus::CANCELLING => break,
          TransactionStatus::CANCELLED => break,
          TransactionStatus::COMPLETED => break,
          TransactionStatus::CONFIRMING => break,
          TransactionStatus::FAILED => break,
          TransactionStatus::REJECTED => break,
          TransactionStatus::TIMEOUT => break,
          _ => {},
        }
      }
      time::sleep(interval).await;
      total_time = total_time.add(interval);
      if total_time > timeout {
        break;
      }
    }
    self.get(u.clone()).await
  }
}