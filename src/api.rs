use std::borrow::Borrow;
use std::fmt::{Debug, Display};

use crate::client::Client;
use crate::types::connect::{WalletApprove, WalletConnectRequest, WalletConnectResponse};
use crate::{
  types::{
    address::{Address, AddressContainer, CreateAddressResponse},
    asset::SupportedAsset,
    connect::PagedWalletConnectResponse,
    fee::EstimateFee,
    staking::{StakingPosition, StakingPositionsSummary},
    transaction::{CreateTransactionResponse, Transaction, TransactionArguments},
    vault::{Account, CreateAccount, VaultAccounts},
    wallet::{WalletContainer, WalletCreate, WalletCreateAsset, WalletCreateAssetResponse},
    PaginatedAssetWallet,
  },
  Result,
};

impl Client {
  /// Create an asset (address) for a vault account
  ///
  /// * [`crate::assets`]
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

  #[tracing::instrument(level = "debug", skip(self))]
  pub async fn external_wallets(&self) -> Result<Vec<WalletContainer>> {
    let u = self.build_url("external_wallets")?.0;
    self.get(u).await
  }

  #[tracing::instrument(level = "debug", skip(self))]
  pub async fn external_wallet(&self, id: &str) -> Result<WalletContainer> {
    let u = self.build_url(&format!("external_wallets/{id}"))?.0;
    self.get(u).await
  }

  #[tracing::instrument(level = "debug", skip(self))]
  pub async fn external_wallet_asset(&self, id: &str, asset: &str, address: &str) -> Result<WalletCreateAssetResponse> {
    let u = self.build_url(&format!("external_wallets/{id}/{asset}"))?.0;
    let w = WalletCreateAsset { address: String::from(address), tag: "fireblocks-sdk-rs".to_string() };
    self.post(u, Some(&w)).await
  }

  #[tracing::instrument(level = "debug", skip(self))]
  pub async fn external_wallet_create(&self, name: &str) -> Result<WalletContainer> {
    let u = self.build_url("external_wallets")?.0;
    let w = WalletCreate { name: String::from(name) };
    self.post(u, Some(&w)).await
  }

  #[tracing::instrument(level = "debug", skip(self))]
  pub async fn external_wallet_delete(&self, id: &str) -> Result<()> {
    let u = self.build_url(&format!("external_wallets/{id}"))?.0;
    self.delete(u).await
  }

  #[tracing::instrument(level = "debug", skip(self))]
  pub async fn contracts(&self) -> Result<Vec<WalletContainer>> {
    let u = self.build_url("contracts")?.0;
    self.get(u).await
  }

  #[tracing::instrument(level = "debug", skip(self))]
  pub async fn contract(&self, id: &str) -> Result<WalletContainer> {
    let u = self.build_url(&format!("contracts/{id}"))?.0;
    self.get(u).await
  }

  #[tracing::instrument(level = "debug", skip(self))]
  pub async fn contract_asset(&self, id: &str, asset: &str, address: &str) -> Result<WalletCreateAssetResponse> {
    let u = self.build_url(&format!("contracts/{id}/{asset}"))?.0;
    let w = WalletCreateAsset { address: String::from(address), tag: "fireblocks-sdk-rs".to_string() };
    self.post(u, Some(&w)).await
  }

  #[tracing::instrument(level = "debug", skip(self))]
  pub async fn contract_create(&self, name: &str) -> Result<WalletContainer> {
    let u = self.build_url("contracts")?.0;
    let w = WalletCreate { name: String::from(name) };
    self.post(u, Some(&w)).await
  }

  #[tracing::instrument(level = "debug", skip(self))]
  pub async fn contract_delete(&self, id: &str) -> Result<()> {
    let u = self.build_url(&format!("contracts/{id}"))?.0;
    self.delete(u).await
  }

  #[tracing::instrument(level = "debug", skip(self))]
  pub async fn internal_wallets(&self) -> Result<Vec<WalletContainer>> {
    let u = self.build_url("internal_wallets")?.0;
    self.get(u).await
  }

  #[tracing::instrument(level = "debug", skip(self))]
  pub async fn supported_assets(&self) -> Result<Vec<SupportedAsset>> {
    let u = self.build_url("supported_assets")?.0;
    self.get(u).await
  }

  #[tracing::instrument(level = "debug", skip(self))]
  pub async fn staking_chains(&self) -> Result<Vec<String>> {
    let u = self.build_url("staking/chains")?.0;
    self.get(u).await
  }

  #[tracing::instrument(level = "debug", skip(self))]
  pub async fn staking_positions(&self) -> Result<Vec<StakingPosition>> {
    let u = self.build_url("staking/positions")?.0;
    self.get(u).await
  }

  #[tracing::instrument(level = "debug", skip(self))]
  pub async fn staking_positions_summary(&self) -> Result<StakingPositionsSummary> {
    let u = self.build_url("staking/positions/summary")?.0;
    self.get(u).await
  }

  #[tracing::instrument(level = "debug", skip(self))]
  pub async fn estimate_fee(&self, asset: &str) -> Result<EstimateFee> {
    let u = self.build_url(&format!("estimate_network_fee?assetId={asset}"))?.0;
    self.get(u).await
  }

  #[tracing::instrument(level = "debug", skip(self, options))]
  pub async fn transactions<I, K, V>(&self, options: I) -> Result<Vec<Transaction>>
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
  pub async fn create_transaction(&self, args: &TransactionArguments) -> Result<CreateTransactionResponse> {
    let u = self.build_url("transactions")?.0;
    self.post(u, Some(args)).await
  }

  #[tracing::instrument(level = "debug", skip(self))]
  pub async fn get_transaction(&self, id: &str) -> Result<Transaction> {
    let u = self.build_url(&format!("transactions/{id}"))?.0;
    self.get(u).await
  }

  #[tracing::instrument(level = "debug", skip(self))]
  pub async fn wallet_connections(&self) -> Result<PagedWalletConnectResponse> {
    let u = self.build_url("connections")?.0;
    self.get(u).await
  }

  #[tracing::instrument(level = "debug", skip(self))]
  pub async fn wallet_connect(&self, request: &WalletConnectRequest) -> Result<WalletConnectResponse> {
    let u = self.build_url("connections/wc")?.0;
    self.post(u, Some(request)).await
  }

  #[tracing::instrument(level = "debug", skip(self))]
  pub async fn wallet_connection_delete(&self, id: &str) -> Result<()> {
    let u = self.build_url(&format!("connections/wc/{id}"))?.0;
    self.delete(u).await
  }

  #[tracing::instrument(level = "debug", skip(self))]
  pub async fn wallet_connection_approve(&self, id: &str, approve: bool) -> Result<()> {
    let u = self.build_url(&format!("connections/wc/{id}"))?.0;
    self.put(u, Some(&WalletApprove { approve })).await
  }
}
