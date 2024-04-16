use bigdecimal::BigDecimal;

use crate::client::Client;
use crate::types::connect::{WalletApprove, WalletConnectRequest, WalletConnectResponse};
use crate::{
  types::{
    address::{Address, AddressContainer, CreateAddressResponse},
    asset::SupportedAsset,
    connect::PagedWalletConnectResponse,
    fee::EstimateFee,
    staking::{StakingPosition, StakingPositionsSummary},
    transaction::{CreateTransactionResponse, Transaction, TransactionArguments, TransactionListOptions},
    vault::{Account, CreateAccount, VaultAccounts},
    wallet::{WalletContainer, WalletCreate, WalletCreateAsset, WalletCreateAssetResponse},
    PaginatedAssetWallet,
  },
  PagingVaultRequest, Result,
};

impl Client {
  #[tracing::instrument(skip(self))]
  pub async fn create_address(&self, vault_id: i32, asset_id: &str) -> Result<CreateAddressResponse> {
    let p = format!("vault/accounts/{vault_id}/{asset_id}");
    let (u, _) = self.build_uri(&p, None)?;
    self.post(u).await
  }

  #[tracing::instrument(skip(self))]
  pub async fn addresses(&self, vault_id: i32, asset_id: &str) -> Result<Vec<Address>> {
    let p = format!("vault/accounts/{vault_id}/{asset_id}/addresses");
    let (u, _) = self.build_uri(&p, None)?;
    self.get(u).await
  }

  #[tracing::instrument(skip(self))]
  pub async fn addresses_paginated(
    &self,
    vault_id: i32,
    asset_id: &str,
    paging: Option<&PagingVaultRequest>,
  ) -> Result<AddressContainer> {
    let p = format!("vault/accounts/{vault_id}/{asset_id}/addresses_paginated");
    let (u, _) = self.build_uri(&p, paging)?;
    self.get(u).await
  }

  #[tracing::instrument(skip(self))]
  pub async fn vault(&self, vault_id: i32) -> Result<Account> {
    let p = format!("vault/accounts/{vault_id}");
    let (u, _) = self.build_uri(&p, None)?;
    self.get(u).await
  }

  #[tracing::instrument(skip(self))]
  pub async fn vaults(
    &self,
    page: Option<&PagingVaultRequest>,
    min_threshold: Option<&BigDecimal>,
  ) -> Result<VaultAccounts> {
    let (mut u, _) = self.build_uri("vault/accounts_paged", page)?;
    if let Some(min) = min_threshold {
      u.query_pairs_mut().append_pair("minAmountThreshold", &format!("{min}"));
    }
    self.get(u).await
  }

  #[tracing::instrument(skip(self))]
  pub async fn create_vault(&self, account: &CreateAccount) -> Result<Account> {
    let (u, _) = self.build_uri("vault/accounts", None)?;
    self.post_body(u, account).await
  }

  #[tracing::instrument(skip(self))]
  pub async fn assets(
    &self,
    page: Option<&PagingVaultRequest>,
    greater_than: Option<f64>,
  ) -> Result<PaginatedAssetWallet> {
    let (mut u, _) = self.build_uri("vault/asset_wallets?limit=1000", page)?;
    if let Some(greater) = greater_than {
      u.query_pairs_mut().append_pair("totalAmountLargerThan", &format!("{greater}"));
    }
    self.get(u).await
  }

  #[tracing::instrument(skip(self))]
  pub async fn external_wallets(&self) -> Result<Vec<WalletContainer>> {
    let (u, _) = self.build_uri("external_wallets", None)?;
    self.get(u).await
  }

  #[tracing::instrument(skip(self))]
  pub async fn external_wallet(&self, id: &str) -> Result<WalletContainer> {
    let (u, _) = self.build_uri(&format!("external_wallets/{id}"), None)?;
    self.get(u).await
  }

  #[tracing::instrument(skip(self))]
  pub async fn external_wallet_asset(&self, id: &str, asset: &str, address: &str) -> Result<WalletCreateAssetResponse> {
    let (u, _) = self.build_uri(&format!("external_wallets/{id}/{asset}"), None)?;
    let w = WalletCreateAsset { address: String::from(address), tag: "fireblocks-sdk-rs".to_string() };
    self.post_body(u, w).await
  }

  #[tracing::instrument(skip(self))]
  pub async fn external_wallet_create(&self, name: &str) -> Result<WalletContainer> {
    let (u, _) = self.build_uri("external_wallets", None)?;
    let w = WalletCreate { name: String::from(name) };
    self.post_body(u, w).await
  }

  #[tracing::instrument(skip(self))]
  pub async fn external_wallet_delete(&self, id: &str) -> Result<()> {
    let (u, _) = self.build_uri(&format!("external_wallets/{id}"), None)?;
    self.delete(u).await
  }

  #[tracing::instrument(skip(self))]
  pub async fn contracts(&self) -> Result<Vec<WalletContainer>> {
    let (u, _) = self.build_uri("contracts", None)?;
    self.get(u).await
  }

  #[tracing::instrument(skip(self))]
  pub async fn contract(&self, id: &str) -> Result<WalletContainer> {
    let (u, _) = self.build_uri(&format!("contracts/{id}"), None)?;
    self.get(u).await
  }

  #[tracing::instrument(skip(self))]
  pub async fn contract_asset(&self, id: &str, asset: &str, address: &str) -> Result<WalletCreateAssetResponse> {
    let (u, _) = self.build_uri(&format!("contracts/{id}/{asset}"), None)?;
    let w = WalletCreateAsset { address: String::from(address), tag: "fireblocks-sdk-rs".to_string() };
    self.post_body(u, w).await
  }

  #[tracing::instrument(skip(self))]
  pub async fn contract_create(&self, name: &str) -> Result<WalletContainer> {
    let (u, _) = self.build_uri("contracts", None)?;
    let w = WalletCreate { name: String::from(name) };
    self.post_body(u, w).await
  }

  #[tracing::instrument(skip(self))]
  pub async fn contract_delete(&self, id: &str) -> Result<()> {
    let (u, _) = self.build_uri(&format!("contracts/{id}"), None)?;
    self.delete(u).await
  }

  #[tracing::instrument(skip(self))]
  pub async fn internal_wallets(&self) -> Result<Vec<WalletContainer>> {
    let (u, _) = self.build_uri("internal_wallets", None)?;
    self.get(u).await
  }

  #[tracing::instrument(skip(self))]
  pub async fn supported_assets(&self) -> Result<Vec<SupportedAsset>> {
    let (u, _) = self.build_uri("supported_assets", None)?;
    self.get(u).await
  }

  #[tracing::instrument(skip(self))]
  pub async fn staking_chains(&self) -> Result<Vec<String>> {
    let (u, _) = self.build_uri("staking/chains", None)?;
    self.get(u).await
  }

  #[tracing::instrument(skip(self))]
  pub async fn staking_positions(&self) -> Result<Vec<StakingPosition>> {
    let (u, _) = self.build_uri("staking/positions", None)?;
    self.get(u).await
  }

  #[tracing::instrument(skip(self))]
  pub async fn staking_positions_summary(&self) -> Result<StakingPositionsSummary> {
    let (u, _) = self.build_uri("staking/positions/summary", None)?;
    self.get(u).await
  }

  #[tracing::instrument(skip(self))]
  pub async fn estimate_fee(&self, asset: &str) -> Result<EstimateFee> {
    let (u, _) = self.build_uri(&format!("estimate_network_fee?assetId={asset}"), None)?;
    self.get(u).await
  }

  #[tracing::instrument(skip(self))]
  pub async fn transactions(&self, options: &TransactionListOptions) -> Result<Vec<Transaction>> {
    let (mut u, _) = self.build_uri("transactions", None)?;
    u.query_pairs_mut().extend_pairs(options.params());
    self.get(u).await
  }

  #[tracing::instrument(skip(self))]
  pub async fn create_transaction(&self, args: &TransactionArguments) -> Result<CreateTransactionResponse> {
    let (u, _) = self.build_uri("transactions", None)?;
    self.post_body(u, args).await
  }

  #[tracing::instrument(skip(self))]
  pub async fn get_transaction(&self, id: &str) -> Result<Transaction> {
    let (u, _) = self.build_uri(&format!("transactions/{id}"), None)?;
    self.get(u).await
  }

  #[tracing::instrument(skip(self))]
  pub async fn wallet_connections(&self) -> Result<PagedWalletConnectResponse> {
    let u = self.build_uri("connections", None)?.0;
    self.get(u).await
  }

  #[tracing::instrument(skip(self))]
  pub async fn wallet_connect(&self, request: &WalletConnectRequest) -> Result<WalletConnectResponse> {
    let u = self.build_uri("connections/wc", None)?.0;
    self.post_body(u, request).await
  }

  #[tracing::instrument(skip(self))]
  pub async fn wallet_connection_delete(&self, id: &str) -> Result<()> {
    let u = self.build_uri(&format!("connections/wc/{id}"), None)?.0;
    self.delete(u).await
  }

  #[tracing::instrument(skip(self))]
  pub async fn wallet_connection_approve(&self, id: &str, approve: bool) -> Result<()> {
    let u = self.build_uri(&format!("connections/wc/{id}"), None)?.0;
    self.put(u, WalletApprove { approve }).await
  }
}
