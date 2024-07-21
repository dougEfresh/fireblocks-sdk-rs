use crate::types::{
  CreateTransactionResponse, DestinationTransferPeerPath, EstimateFee, OneTimeAddress, PeerType, Transaction,
  TransactionArguments, TransactionOperation, TransactionStatus, TransferPeerPath,
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
  /// See
  /// * [getTransactions](https://docs.fireblocks.com/api/swagger-ui/#/Transactions/getTransactions)
  /// * [`crate::types::transaction::TransactionListBuilder`]
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

  /// Create a vault-to-peer destination transaction (e.g. INTERNAL_WALLET)
  /// create_transaction_whitelist(0, &id, PeerType::INTERNAL_WALLET, "SOL_TEST", BigDecimal::from_str("0.00001")?, None).await?
  ///
  /// [createTransaction](https://docs.fireblocks.com/api/swagger-ui/#/Transactions/createTransaction)
  #[tracing::instrument(level = "debug", skip(self))]
  pub async fn create_transaction_peer<T>(
    &self,
    source_vault: i32,
    destination_wallet_id: &str,
    peer_type: PeerType,
    asset_id: T,
    amount: BigDecimal,
    note: Option<&str>,
  ) -> crate::Result<CreateTransactionResponse>
  where
    T: AsRef<str> + Debug + Display,
  {
    let dest = DestinationTransferPeerPath {
      peer_type,
      id: String::from(destination_wallet_id),
      wallet_id: Some(String::from(destination_wallet_id)),
      virtual_id: None,
      virtual_type: None,
      one_time_address: None,
    };
    let args = &TransactionArguments {
      asset_id: format!("{asset_id}"),
      operation: TransactionOperation::TRANSFER,
      source: TransferPeerPath { id: Some(source_vault.to_string()), ..Default::default() },
      destination: Some(dest),
      amount: amount.to_string(),
      gas_price: None,
      gas_limit: None,
      note: note.unwrap_or("created by fireblocks-sdk for rust").to_string(),
    };
    self.create_transaction(args).await
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
      source: TransferPeerPath { id: Some(source_vault.to_string()), ..Default::default() },
      destination: Some(DestinationTransferPeerPath { id: destination_vault.to_string(), ..Default::default() }),
      amount: amount.to_string(),
      gas_price: None,
      gas_limit: None,
      note: note.unwrap_or("created by fireblocks-sdk for rust").to_string(),
    };
    self.create_transaction(args).await
  }

  /// Create a transaction to external wallet
  ///
  /// [createTransaction](https://docs.fireblocks.com/api/swagger-ui/#/Transactions/createTransaction)
  #[tracing::instrument(level = "debug", skip(self))]
  pub async fn create_transaction_external<A, D>(
    &self,
    source_vault: i32,
    destination: D,
    asset_id: A,
    amount: BigDecimal,
    note: Option<&str>,
  ) -> crate::Result<CreateTransactionResponse>
  where
    A: AsRef<str> + Debug + Display,
    D: AsRef<str> + Debug + Display,
  {
    let args = &TransactionArguments {
      asset_id: format!("{asset_id}"),
      operation: TransactionOperation::TRANSFER,
      source: TransferPeerPath {
        id: Some(source_vault.to_string()),
        peer_type: PeerType::VAULT_ACCOUNT,
        ..Default::default()
      },
      destination: Some(DestinationTransferPeerPath {
        peer_type: PeerType::ONE_TIME_ADDRESS,
        one_time_address: Some(OneTimeAddress { address: destination.to_string(), tag: None }),
        ..Default::default()
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
    let u = self.build_url(format!("transactions/{id}"))?.0;
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
  #[tracing::instrument(level = "debug", skip(self, callback))]
  pub async fn poll_transaction(
    &self,
    id: &str,
    timeout: time::Duration,
    interval: time::Duration,
    callback: impl Fn(&Transaction) + Send + Sync,
  ) -> crate::Result<Transaction> {
    let u = self.build_url(format!("transactions/{id}"))?.0;
    let mut total_time = time::Duration::from_millis(0);
    loop {
      if let Ok(result) = self.get::<Transaction>(u.clone()).await {
        let status = result.0.status.clone();
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
          _ => {
            callback(&result.0);
          },
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

  #[tracing::instrument(level = "debug", skip(self))]
  pub async fn estimate_fee(&self, asset: &str) -> crate::Result<EstimateFee> {
    let u = self.build_url(format!("estimate_network_fee?assetId={asset}"))?.0;
    self.get(u).await
  }

  #[tracing::instrument(level = "debug", skip(self))]
  pub async fn estimate_fee_transaction(&self, t: &TransactionArguments) -> crate::Result<EstimateFee> {
    let u = self.build_url("transactions/estimate_fee")?.0;
    self.post(u, Some(t)).await
  }
}
