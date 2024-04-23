use crate::types::{CreateTransactionResponse, Transaction, TransactionArguments};
use crate::Client;
use std::borrow::Borrow;

impl Client {
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

  #[tracing::instrument(level = "debug", skip(self))]
  pub async fn create_transaction(&self, args: &TransactionArguments) -> crate::Result<CreateTransactionResponse> {
    let u = self.build_url("transactions")?.0;
    self.post(u, Some(args)).await
  }

  #[tracing::instrument(level = "debug", skip(self))]
  pub async fn get_transaction(&self, id: &str) -> crate::Result<Transaction> {
    let u = self.build_url(&format!("transactions/{id}"))?.0;
    self.get(u).await
  }
}
