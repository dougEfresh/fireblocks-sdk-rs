use std::fmt::Debug;
use std::pin::Pin;
use std::task::{Context, Poll};
use futures::Stream;
use crate::{Client, Epoch};
use crate::types::{Transaction, TransactionListOptions};

#[derive(Clone)]
struct PagedClient {
  client: Client
}

#[derive(Clone)]
pub struct  TransactionStream
{
  client: Client,
  after: Epoch,
  before: Epoch,
}

impl Stream for TransactionStream
{
  type Item = Vec<Transaction>;

  fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
    self.client.transactions(&TransactionListOptions::new().after()
  })

    if self.has_more_pages() {
      // Make the API call to fetch the next page of results asynchronously
      match self.fetch_next_page() {
        Ok(records) => Poll::Ready(Some(records)),
        Err(_) => {
          // Handle the error case
          Poll::Ready(None)
        }
      }
    } else {
      // No more pages, end the stream
      Poll::Ready(None)
    }

}

impl PagedClient {

  pub async fn paged_transactions(&self) {

  }
}