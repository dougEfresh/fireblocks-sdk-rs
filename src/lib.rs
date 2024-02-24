use bigdecimal::BigDecimal;
use types::{address::Address, asset::SupportedAsset, vault::VaultAccounts, wallet::WalletContainer};

use crate::types::{
  address::{AddressContainer, CreateAddressResponse},
  fee::EstimateFee,
  staking::{StakingPosition, StakingPositionsSummary},
  transaction::{CreateTransactionResponse, Transaction, TransactionArguments, TransactionListOptions},
  vault::{Account, CreateAccount},
  wallet::WalletCreateAssetResponse,
  PaginatedAssetWallet, PagingVaultRequest,
};

pub mod api;
pub mod error;
pub mod jwt;
pub mod types;

pub type Result<T> = std::result::Result<(T, String), error::FireblocksError>;

#[trait_variant::make(FireblocksFactory: Send)]
pub trait FireblocksClient {
  async fn create_address(&self, vault_id: i32, asset_id: &str) -> Result<CreateAddressResponse>;
  async fn addresses(&self, vault_id: i32, asset_id: &str) -> Result<Vec<Address>>;
  async fn addresses_paginated(
    &self,
    vault_id: i32,
    asset_id: &str,
    paging: Option<&PagingVaultRequest>,
  ) -> Result<AddressContainer>;

  async fn vault(&self, vault_id: i32) -> Result<Account>;
  async fn vaults(
    &self,
    page: Option<&PagingVaultRequest>,
    min_threshold: Option<&BigDecimal>,
  ) -> Result<VaultAccounts>;
  async fn create_vault(&self, account: &CreateAccount) -> Result<Account>;

  async fn assets(&self, page: Option<&PagingVaultRequest>, greater_than: Option<f64>) -> Result<PaginatedAssetWallet>;

  async fn external_wallets(&self) -> Result<Vec<WalletContainer>>;
  async fn external_wallet(&self, id: &str) -> Result<WalletContainer>;
  async fn external_wallet_asset(&self, id: &str, asset: &str, address: &str) -> Result<WalletCreateAssetResponse>;
  async fn external_wallet_create(&self, name: &str) -> Result<WalletContainer>;
  async fn external_wallet_delete(&self, id: &str) -> Result<()>;

  async fn contracts(&self) -> Result<Vec<WalletContainer>>;
  async fn contract(&self, id: &str) -> Result<WalletContainer>;
  async fn contract_asset(&self, id: &str, asset: &str, address: &str) -> Result<WalletCreateAssetResponse>;
  async fn contract_create(&self, name: &str) -> Result<WalletContainer>;
  async fn contract_delete(&self, id: &str) -> Result<()>;

  async fn internal_wallets(&self) -> Result<Vec<WalletContainer>>;
  async fn supported_assets(&self) -> Result<Vec<SupportedAsset>>;
  async fn staking_chains(&self) -> Result<Vec<String>>;
  async fn staking_positions(&self) -> Result<Vec<StakingPosition>>;
  async fn staking_positions_summary(&self) -> Result<StakingPositionsSummary>;
  async fn estimate_fee(&self, asset: &str) -> Result<EstimateFee>;
  async fn transactions(&self, options: &TransactionListOptions) -> Result<Vec<Transaction>>;
  async fn create_transaction(&self, args: &TransactionArguments) -> Result<CreateTransactionResponse>;
  async fn get_transaction(&self, id: &str) -> Result<Transaction>;
}

#[cfg(test)]
mod tests {
  use std::sync::{Once, OnceLock};
  use std::{str::FromStr, time::Duration};

  use bigdecimal::BigDecimal;
  use chrono::Utc;
  use jsonwebtoken::EncodingKey;
  use log::warn;
  use reqwest::Client;
  use tracing_subscriber::EnvFilter;

  use crate::{
    api::FireblocksHttpClient,
    types::{transaction::TransferPeerPath, vault::CreateAccount, PagingVaultRequest},
    FireblocksFactory,
  };

  static INIT: Once = Once::new();
  static FB: OnceLock<FireblocksHttpClient> = OnceLock::new();

  #[allow(clippy::unwrap_used)]
  fn setup() {
    INIT.call_once(|| {
      color_eyre::install().unwrap();
      let filter = EnvFilter::from_default_env();
      let subscriber = tracing_subscriber::FmtSubscriber::builder().with_env_filter(filter).with_target(true).finish();
      tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
      let env = dotenvy::dotenv();
      if env.is_err() {
        warn!("no .env file");
      }
      let api_key: Option<String> = std::env::var("FIREBLOCKS_API_KEY").ok();
      let path: Option<String> = std::env::var("FIREBLOCKS_SECRET_PATH").ok();
      if api_key.is_none() || path.is_none() {
        return;
      }
      let rsa_pem = std::fs::read(path.unwrap()).unwrap();
      let key = EncodingKey::from_rsa_pem(&rsa_pem[..]).unwrap();
      let signer = crate::jwt::Signer::new(key, &api_key.unwrap());
      let c =
        Client::builder().connect_timeout(Duration::from_secs(5)).timeout(Duration::from_secs(30)).build().unwrap();
      let fb = FireblocksHttpClient::sandbox(signer, c);
      let _ = FB.set(fb);
    });
  }

  #[rstest::fixture]
  fn config() -> Config {
    setup();
    Config::new()
  }

  pub struct Config {
    client: Option<FireblocksHttpClient>,
  }

  impl Config {
    fn new() -> Self {
      Self { client: FB.get().map_or_else(|| None, |c| Some(c.clone())) }
    }

    const fn is_ok(&self) -> bool {
      self.client.is_some()
    }

    #[allow(clippy::unwrap_used)]
    fn client(&self) -> &FireblocksHttpClient {
      self.client.as_ref().unwrap()
    }
  }

  #[rstest::rstest]
  #[tokio::test]
  async fn test_vaults(config: Config) -> color_eyre::Result<()> {
    if !config.is_ok() {
      return Ok(());
    }
    let (results, id) = config.client().vaults(None, None).await?;
    assert!(!id.is_empty());
    assert!(!results.accounts.is_empty());

    let min = BigDecimal::from_str("1000000.00")?;

    let (results, id) = config.client().vaults(None, Some(min).as_ref()).await?;
    assert!(!id.is_empty());
    assert!(results.accounts.is_empty());

    let _before = &chrono::offset::Utc::now();
    let page = PagingVaultRequest { limit: 1, ..Default::default() };
    let (results, id) = config.client().vaults(Some(page).as_ref(), None).await?;
    assert!(!id.is_empty());
    assert_eq!(1, results.accounts.len());

    let (result, id) = config.client().vault(0).await?;
    assert!(!id.is_empty());
    assert_eq!(0, result.id);
    assert!(!result.assets.is_empty());
    Ok(())
  }

  fn vault_name() -> String {
    format!("z-test-{}", Utc::now().timestamp())
  }

  #[rstest::rstest]
  #[tokio::test]
  async fn test_create_vaults(config: Config) -> color_eyre::Result<()> {
    if !config.is_ok() {
      return Ok(());
    }
    let vault_name: String = vault_name();
    let account = CreateAccount {
      auto_fuel: false,
      customer_ref_id: Some("fireblocks-sdk-rs".to_string()),
      hidden_on_ui: true,
      name: vault_name.clone(),
    };

    let (result, id) = config.client().create_vault(&account).await?;
    assert!(!id.is_empty());
    assert_eq!(account.name, result.name);
    assert!(result.hidden_on_ui);
    assert!(result.id > 0);

    let (address_response, id) = config.client().create_address(result.id, "SOL_TEST").await?;
    assert!(!id.is_empty());
    assert!(!address_response.address.is_empty());
    assert!(!address_response.id.is_empty());
    let addr = address_response.address.clone();
    let (address_response, id) = config.client().addresses(result.id, "SOL_TEST").await?;
    assert!(!id.is_empty());
    assert_eq!(1, address_response.len());
    assert_eq!(addr, address_response[0].address);

    let page = PagingVaultRequest { limit: 10, ..Default::default() };
    let (container, id) = config.client().addresses_paginated(result.id, "SOL_TEST", Some(page).as_ref()).await?;
    assert!(!id.is_empty());
    assert_eq!(1, container.addresses.len());
    Ok(())
  }

  #[rstest::rstest]
  #[tokio::test]
  async fn test_wallet_contract(config: Config) -> color_eyre::Result<()> {
    if !config.is_ok() {
      return Ok(());
    }
    let name = format!("c-{}", vault_name());
    let (contract_response, id) = config.client().contract_create(&name).await?;
    assert!(!id.is_empty());
    assert_eq!(contract_response.name, name);
    assert!(!contract_response.id.is_empty());

    let (addr_response, _) = config
      .client()
      .contract_asset(&contract_response.id, "ETH_TEST3", "0x9bb4d44e6963260a1850926e8f6beb8d5803836f")
      .await?;
    assert!(!addr_response.id.is_empty());

    config.client().contract_delete(&name).await?;
    Ok(())
  }

  #[rstest::rstest]
  #[tokio::test]
  async fn test_external_wallet(config: Config) -> color_eyre::Result<()> {
    if !config.is_ok() {
      return Ok(());
    }
    let name = vault_name();
    let (contract_response, id) = config.client().external_wallet_create(&name).await?;
    assert!(!id.is_empty());
    assert_eq!(contract_response.name, name);
    assert!(!contract_response.id.is_empty());

    let (addr_response, _) = config
      .client()
      .external_wallet_asset(&contract_response.id, "ETH_TEST3", "0x9bb4d44e6963260a1850926e8f6beb8d5803836f")
      .await?;
    assert!(!addr_response.id.is_empty());

    config.client().external_wallet_delete(&name).await?;
    Ok(())
  }

  #[rstest::rstest]
  #[tokio::test]
  async fn test_estimate_fees(config: Config) -> color_eyre::Result<()> {
    if !config.is_ok() {
      return Ok(());
    }
    let (result, id) = config.client().estimate_fee("BTC_TEST").await?;
    assert!(!id.is_empty());
    assert!(result.low.gas_price.is_none());
    assert!(result.medium.gas_price.is_none());
    assert!(result.high.gas_price.is_none());

    assert!(result.low.network_fee.is_none());
    assert!(result.medium.network_fee.is_none());
    assert!(result.high.network_fee.is_none());

    assert!(result.low.fee_per_byte.is_some());
    assert!(result.medium.fee_per_byte.is_some());
    assert!(result.high.fee_per_byte.is_some());
    Ok(())
  }

  #[test]
  fn test_handle_not_present() -> color_eyre::Result<()> {
    let data = r#"{ "type": "VAULT_ACCOUNT","name": "jupiter","subType": ""}"#;
    let result: TransferPeerPath = serde_json::from_str(data)?;
    assert!(result.id.is_none());
    Ok(())
  }
}
