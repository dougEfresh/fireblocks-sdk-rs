use crate::types::{
  Account, Address, AddressContainer, CreateAccount, CreateAddressResponse, PaginatedAssetWallet, VaultAccounts,
  VaultRenameResponse,
};
use crate::Client;
use crate::Result;
use serde_derive::Serialize;
use std::borrow::Borrow;
use std::fmt::{Debug, Display};

impl Client {
  /// Create an asset (address) for a vault account
  ///
  /// * [`crate::Asset`]
  /// * [createVaultAccountAsset](https://docs.fireblocks.com/api/swagger-ui/#/Vaults/createVaultAccountAsset)
  #[tracing::instrument(level = "debug", skip(self))]
  pub async fn create_address<T>(&self, vault_id: i32, asset_id: T) -> Result<CreateAddressResponse>
  where
    T: AsRef<str> + Display + Debug,
  {
    let p = format!("vault/accounts/{vault_id}/{asset_id}");
    let (u, _) = self.build_url(&p)?;
    self.post(u, None as Option<&()>).await
  }

  #[tracing::instrument(level = "debug", skip(self))]
  pub async fn addresses<T>(&self, vault_id: i32, asset_id: T) -> Result<Vec<Address>>
  where
    T: AsRef<str> + Display + Debug,
  {
    let p = format!("vault/accounts/{vault_id}/{asset_id}/addresses");
    let u = self.build_url(&p)?.0;
    self.get(u).await
  }

  #[tracing::instrument(level = "debug", skip(self, paging))]
  pub async fn addresses_paginated<T, I, K, V>(&self, vault_id: i32, asset_id: T, paging: I) -> Result<AddressContainer>
  where
    T: AsRef<str> + Display + Debug,
    I: IntoIterator,
    I::Item: Borrow<(K, V)>,
    K: AsRef<str>,
    V: AsRef<str>,
  {
    let p = format!("vault/accounts/{vault_id}/{asset_id}/addresses_paginated");
    let u = self.build_url_params(&p, Some(paging))?.0;
    self.get(u).await
  }

  #[tracing::instrument(level = "debug", skip(self))]
  pub async fn vault(&self, vault_id: i32) -> Result<Account> {
    let p = format!("vault/accounts/{vault_id}");
    let u = self.build_url(&p)?.0;
    self.get(u).await
  }

  #[tracing::instrument(level = "debug", skip(self, page))]
  pub async fn vaults<I, K, V>(&self, page: I) -> Result<VaultAccounts>
  where
    I: IntoIterator,
    I::Item: Borrow<(K, V)>,
    K: AsRef<str>,
    V: AsRef<str>,
  {
    let u = self.build_url_params("vault/accounts_paged", Some(page))?.0;
    self.get(u).await
  }

  #[tracing::instrument(level = "debug", skip(self))]
  pub async fn create_vault(&self, account: &CreateAccount) -> Result<Account> {
    let u = self.build_url("vault/accounts")?.0;
    self.post(u, Some(account)).await
  }

  #[tracing::instrument(level = "debug", skip(self))]
  pub async fn rename_vault(&self, vault_id: i32, name: &str) -> Result<VaultRenameResponse> {
    #[derive(Debug, Serialize)]
    struct Rename {
      name: String,
    }
    let u = self.build_url(&format!("vault/accounts/{vault_id}"))?.0;
    let name_req = &Rename { name: String::from(name) };
    self.put(u, Some(name_req)).await
  }

  #[tracing::instrument(level = "debug", skip(self, page))]
  pub async fn assets<I, K, V>(&self, page: I) -> Result<PaginatedAssetWallet>
  where
    I: IntoIterator,
    I::Item: Borrow<(K, V)>,
    K: AsRef<str>,
    V: AsRef<str>,
  {
    let u = self.build_url_params("vault/asset_wallets", Some(page))?.0;
    self.get(u).await
  }
}
