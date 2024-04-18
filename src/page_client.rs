use crate::types::{Transaction, TransactionListBuilder};
use crate::{Client, Epoch};
use async_stream::try_stream;
use futures_core::stream::Stream;

#[derive(Clone)]
pub struct PagedClient {
  pub client: Client,
}

#[derive(Clone)]
pub struct TransactionStream {
  client: Client,
  after: Epoch,
}

impl PagedClient {
  pub fn paged_transactions(&self) -> impl Stream<Item = Transaction> {
    try_stream! {
      let after = chrono::DateTime::UNIX_EPOCH.clone();
      let opts = TransactionListBuilder::new().after(&after).build();
      for t in self.client.transactions(opts).await?.1 {
        yield t;
      }
    }
  }
}
