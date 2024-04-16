use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_derive::Deserialize;
use std::fmt::Debug;

use crate::error::FireblocksError;

pub mod api;
mod client;
pub mod error;
pub(crate) mod jwt;
pub mod types;
pub use client::{Client, ClientBuilder};
pub const FIREBLOCKS_API: &str = "https://api.fireblocks.io/v1";
pub const FIREBLOCKS_SANDBOX_API: &str = "https://sandbox-api.fireblocks.io/v1";

pub type Result<T> = std::result::Result<(T, String), FireblocksError>;

#[derive(Debug)]
#[allow(dead_code)]
pub struct PagingVaultRequest {
  pub limit: i32,
  pub before: Option<String>,
  pub after: Option<String>,
  pub name_prefix: Option<String>,
  pub name_suffix: Option<String>,
}

impl Default for PagingVaultRequest {
  fn default() -> Self {
    Self { limit: 500, before: None, after: None, name_prefix: None, name_suffix: None }
  }
}

impl PagingVaultRequest {
  pub fn params(&self) -> Vec<(String, String)> {
    let mut params: Vec<(String, String)> = Vec::new();
    params.push(("limit".to_owned(), self.limit.to_string()));
    self.name_prefix.clone().inspect(|v| params.push(("namePrefix".to_owned(), String::from(v))));
    self.name_suffix.clone().inspect(|v| params.push(("nameSuffix".to_owned(), String::from(v))));
    params
  }
}

#[cfg(test)]
mod tests {
  use std::sync::{Once, OnceLock};
  use std::{env, str::FromStr, time::Duration};

  use crate::types::*;
  use crate::{Client, ClientBuilder, PagingVaultRequest};
  use bigdecimal::BigDecimal;
  use chrono::Utc;
  use color_eyre::eyre::format_err;
  use tracing::{error, warn};
  use tracing_subscriber::fmt::format::FmtSpan;
  use tracing_subscriber::EnvFilter;

  static INIT: Once = Once::new();
  static FB: OnceLock<Client> = OnceLock::new();

  #[allow(clippy::unwrap_used)]
  fn setup() {
    INIT.call_once(|| {
      color_eyre::install().unwrap();
      tracing_subscriber::fmt()
        .with_target(true)
        .with_level(true)
        .with_span_events(FmtSpan::CLOSE)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

      let env = dotenvy::dotenv();
      if env.is_err() {
        warn!("no .env file");
      }

      let api_key: Option<String> = std::env::var("FIREBLOCKS_API_KEY").ok();
      let path: Option<String> = std::env::var("FIREBLOCKS_SECRET").ok();
      if api_key.is_none() || path.is_none() {
        return;
      }
      let path = path.unwrap();
      let rsa_pem = path.as_bytes().to_vec();
      match ClientBuilder::new(api_key.as_ref().unwrap(), &rsa_pem)
        .use_sandbox()
        .with_user_agent("fireblocks-sdk-rs test")
        .with_timeout(Duration::from_secs(30))
        .with_connect_timeout(Duration::from_secs(5))
        .build()
      {
        Ok(fb) => {
          let _ = FB.set(fb);
        },
        Err(e) => {
          error!("failed to create client {e}");
        },
      };
    });
  }

  #[rstest::fixture]
  fn config() -> Config {
    setup();
    Config::new()
  }

  pub struct Config {
    client: Option<Client>,
  }

  impl Config {
    fn new() -> Self {
      Self { client: FB.get().map_or_else(|| None, |c| Some(c.clone())) }
    }

    const fn is_ok(&self) -> bool {
      self.client.is_some()
    }

    #[allow(clippy::unwrap_used)]
    fn client(&self) -> Client {
      self.client.as_ref().unwrap().clone()
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
      .external_wallet_asset(&contract_response.id, "ETH_TEST5", "0x9bb4d44e6963260a1850926e8f6beb8d5803836f")
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

  #[rstest::rstest]
  #[tokio::test]
  async fn test_network(config: Config) -> color_eyre::Result<()> {
    if !config.is_ok() {
      return Ok(());
    }
    config.client().wallet_connections().await?;
    Ok(())
  }

  #[rstest::rstest]
  #[test]
  fn check_ci(config: Config) -> color_eyre::Result<()> {
    match env::var("CI") {
      Err(_) => Ok(()),
      Ok(_) => match config.client {
        Some(_) => Ok(()),
        None => Err(format_err!("client is not configured and you are running in CI")),
      },
    }
  }
}

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct Paging {
  pub before: Option<String>,
  pub after: Option<String>,
}

impl Paging {
  fn epoch(before: &DateTime<Utc>) -> String {
    format!("{}", before.timestamp_millis())
  }

  pub fn set_before(&mut self, before: &DateTime<Utc>) {
    self.before = Some(Self::epoch(before));
  }

  pub fn set_after(&mut self, after: &DateTime<Utc>) {
    self.after = Some(Self::epoch(after));
  }
}
