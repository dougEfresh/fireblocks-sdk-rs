use crate::types::VaultAccounts;
use crate::{Client, PagingVaultRequestBuilder};

#[derive(Clone)]
pub struct PagedClient {
  pub client: Client,
}

#[derive(Clone)]
pub struct VaultStream {
  client: Client,
  batch: u16,
  init: bool, // has the stream started?
  after: Option<String>,
}

pub trait AsyncIteratorAsyncNext {
  type Item;
  async fn next(&mut self) -> crate::Result<Option<Self::Item>>;
}

impl AsyncIteratorAsyncNext for VaultStream {
  type Item = VaultAccounts;

  async fn next(&mut self) -> crate::Result<Option<Self::Item>> {
    if let Some(ref after) = self.after {
      let params = PagingVaultRequestBuilder::new().limit(self.batch).after(after).build()?;
      let (va, id) = self.client.vaults(params).await?;
      self.after = Option::clone(&va.paging.after);
      return Ok((Some(va), id));
    }
    // this is the 1st attempt
    if !self.init {
      self.init = true;
      let params = PagingVaultRequestBuilder::new().limit(self.batch).build()?;
      let (va, id) = self.client.vaults(params).await?;
      self.after = Option::clone(&va.paging.after);
      return Ok((Some(va), id));
    }
    Ok((None, String::new()))
  }
}

impl PagedClient {
  pub const fn new(client: Client) -> Self {
    Self { client }
  }

  /// Stream the vault accounts based on batch size
  ///
  /// see
  pub fn vaults(&self, batch_size: u16) -> VaultStream {
    VaultStream { batch: batch_size, client: self.client.clone(), init: false, after: None }
  }
}
