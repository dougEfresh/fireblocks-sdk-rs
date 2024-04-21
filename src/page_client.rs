use crate::{Client, Epoch};

#[derive(Clone)]
pub struct PagedClient {
  pub client: Client,
}

#[derive(Clone)]
pub struct TransactionStream {
  client: Client,
  after: Epoch,
}
