use crate::api::{WalletCreate, WalletCreateAsset};
use crate::types::{WalletContainer, WalletCreateAssetResponse};
use crate::Client;
use crate::Result;
use std::fmt::{Debug, Display};

impl Client {
  #[tracing::instrument(level = "debug", skip(self))]
  pub async fn internal_wallets(&self) -> Result<Vec<WalletContainer>> {
    let u = self.build_url("internal_wallets")?.0;
    self.get(u).await
  }

  #[tracing::instrument(level = "debug", skip(self))]
  pub async fn internal_wallet(&self, id: &str) -> Result<WalletContainer> {
    let u = self.build_url(format!("internal_wallets/{id}"))?.0;
    self.get(u).await
  }

  #[tracing::instrument(level = "debug", skip(self))]
  pub async fn internal_wallet_asset<T>(&self, id: &str, asset: T, address: &str) -> Result<WalletCreateAssetResponse>
  where
    T: AsRef<str> + Display + Debug,
  {
    let u = self.build_url(format!("internal_wallets/{id}/{asset}"))?.0;
    let w = WalletCreateAsset { address: String::from(address), tag: "fireblocks-sdk-rs".to_string() };
    self.post(u, Some(&w)).await
  }

  #[tracing::instrument(level = "debug", skip(self))]
  pub async fn internal_wallet_create(&self, name: &str) -> Result<WalletContainer> {
    let u = self.build_url("internal_wallets")?.0;
    let w = WalletCreate { name: String::from(name) };
    self.post(u, Some(&w)).await
  }

  #[tracing::instrument(level = "debug", skip(self))]
  pub async fn internal_wallet_delete(&self, id: &str) -> Result<()> {
    let u = self.build_url(format!("internal_wallets/{id}"))?.0;
    self.delete(u).await
  }
}
