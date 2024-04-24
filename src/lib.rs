/*!
`fireblocks_sdk` is an async library for fireblocks [api](https://docs.fireblocks.com/api/swagger-ui/#)
*/
use chrono::{DateTime, Utc};
pub mod api;
mod assets;
mod client;
pub mod error;
pub(crate) mod jwt;
mod page_client;
mod transactions;
pub mod types;
mod wallet_connect;

pub use crate::error::FireblocksError;
pub use crate::types::PagingVaultRequestBuilder;
pub use assets::{Asset, ASSET_BTC, ASSET_BTC_TEST, ASSET_ETH, ASSET_ETH_TEST, ASSET_SOL, ASSET_SOL_TEST};
pub use client::{Client, ClientBuilder};

pub const FIREBLOCKS_API: &str = "https://api.fireblocks.io/v1";
pub const FIREBLOCKS_SANDBOX_API: &str = "https://sandbox-api.fireblocks.io/v1";
pub type Epoch = DateTime<Utc>;
pub type Result<T> = std::result::Result<(T, String), FireblocksError>;
pub type QueryParams = Vec<(String, String)>;

#[macro_export]
macro_rules! impl_base_query_params {
  ($struct_name:ident) => {
    impl $struct_name {
      pub fn new() -> Self {
        Self { params: Vec::new(), base: BasePageParams::new() }
      }
      pub fn limit(&mut self, limit: u16) -> &mut Self {
        self.base.limit(limit);
        self
      }

      pub fn before(&mut self, t: &Epoch) -> &mut Self {
        self.base.before(t);
        self
      }

      pub fn after(&mut self, t: &Epoch) -> &mut Self {
        self.base.after(t);
        self
      }

      pub fn build(&self) -> std::result::Result<QueryParams, $crate::error::ParamError> {
        let mut p = Vec::clone(&self.params);
        let b = self.base.build()?;
        p.extend(b);
        Ok(p)
      }
    }
  };
}

#[cfg(test)]
mod tests {
  use std::str::FromStr;
  use std::sync::{Once, OnceLock};
  use std::{env, time::Duration};

  use crate::assets::{ASSET_BTC_TEST, ASSET_SOL_TEST};
  use crate::types::*;
  use crate::{Client, ClientBuilder, ASSET_ETH_TEST};
  use bigdecimal::BigDecimal;
  use chrono::{TimeZone, Utc};
  use color_eyre::eyre::format_err;
  use tokio::time;
  use tracing::warn;
  use tracing_subscriber::fmt::format::FmtSpan;
  use tracing_subscriber::EnvFilter;

  static INIT: Once = Once::new();
  static KEYS: OnceLock<(String, String)> = OnceLock::new();

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
      let _ = KEYS.set((api_key.unwrap(), path.unwrap()));
    });
  }

  #[rstest::fixture]
  fn config() -> Config {
    setup();
    Config::new()
  }

  pub struct Config {
    client: Option<Client>,
    create_tx: bool,
  }

  impl Config {
    fn new() -> Self {
      let keys = KEYS.get();
      let client: Option<Client> = match keys {
        None => None,
        Some((api_key, path)) => {
          let rsa_pem = path.as_bytes().to_vec();
          ClientBuilder::new(api_key, &rsa_pem)
            .use_sandbox()
            .with_sandbox() // code coverage
            .with_user_agent("fireblocks-sdk-rs test")
            .with_timeout(Duration::from_secs(30))
            .with_connect_timeout(Duration::from_secs(5))
            .build()
            .ok()
        },
      };
      let create_tx = env::var("FIREBLOCKS_CREATE_TX").ok().is_some();
      Self { client, create_tx }
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
    let params = PagingVaultRequestBuilder::new().build()?;
    let (results, id) = config.client().vaults(params).await?;
    assert!(!id.is_empty());
    assert!(!results.accounts.is_empty());

    let params = PagingVaultRequestBuilder::new().min_threshold(&BigDecimal::from_str("1000000.00")?).build()?;
    let (results, id) = config.client().vaults(params).await?;
    assert!(!id.is_empty());
    assert!(results.accounts.is_empty());

    let _before = &chrono::offset::Utc::now();
    let params = PagingVaultRequestBuilder::new().limit(1).build()?;
    let (results, id) = config.client().vaults(params).await?;
    assert!(!id.is_empty());
    assert_eq!(1, results.accounts.len());

    let (result, id) = config.client().vault(0).await?;
    assert!(!id.is_empty());
    assert_eq!(0, result.id);
    assert!(!result.assets.is_empty());
    Ok(())
  }

  #[rstest::rstest]
  #[tokio::test]
  async fn test_vault_names(config: Config) -> color_eyre::Result<()> {
    if !config.is_ok() {
      return Ok(());
    }
    let params = PagingVaultRequestBuilder::new().name_prefix("Default").build()?;
    let results = config.client().vaults(params).await?.0;
    assert!(!results.accounts.is_empty());
    assert_eq!(results.accounts[0].name, "Default");

    let params = PagingVaultRequestBuilder::new().name_suffix("Default").build()?;
    let results = config.client().vaults(params).await?.0;
    assert!(!results.accounts.is_empty());
    assert_eq!(results.accounts[0].name, "Default");
    Ok(())
  }

  fn vault_name() -> String {
    format!("z-test-{}", Utc::now().timestamp())
  }

  #[rstest::rstest]
  #[tokio::test]
  #[allow(clippy::unwrap_used)]
  async fn test_supported_assets(config: Config) -> color_eyre::Result<()> {
    if !config.is_ok() {
      return Ok(());
    }
    let c = config.client();

    let assets = c.supported_assets().await?.0;
    assert!(!assets.is_empty());
    let found = assets.iter().find(|a| a.id == ASSET_BTC_TEST);
    assert!(found.is_some());
    Ok(())
  }

  #[rstest::rstest]
  #[tokio::test]
  #[allow(clippy::unwrap_used)]
  async fn test_transaction_list(config: Config) -> color_eyre::Result<()> {
    let after = Utc::now();
    let before = Utc::now();
    // test all options
    let options = TransactionListBuilder::new()
      .after(&after)
      .before(&before)
      .assets(&[ASSET_BTC_TEST, ASSET_SOL_TEST])
      .tx_hash("something")
      .source_id(9)
      .destination_id(19)
      .limit(200)
      .build()?;

    let v = options.iter().find(|(a, _)| *a == "after");
    assert!(v.is_some());

    let v = options.iter().find(|(a, _)| *a == "before");
    assert!(v.is_some());

    let v = options.iter().find(|(a, _)| *a == "assets");
    assert!(v.is_some());
    assert_eq!("BTC_TEST,SOL_TEST", v.unwrap().1);

    let v = options.iter().find(|(a, _)| *a == "sourceId");
    assert!(v.is_some());
    assert_eq!("9", v.unwrap().1);

    let v = options.iter().find(|(a, _)| *a == "destId");
    assert!(v.is_some());
    assert_eq!("19", v.unwrap().1);

    let v = options.iter().find(|(a, _)| *a == "limit");
    assert!(v.is_some());
    assert_eq!("200", v.unwrap().1);

    let options = TransactionListBuilder::new().assets(&[ASSET_BTC_TEST, ASSET_SOL_TEST]).limit(200).build()?;
    if !config.is_ok() {
      return Ok(());
    }
    let c = config.client();
    let transactions = c.transactions(options).await?.0;
    assert!(!transactions.is_empty());
    let tx_id = &transactions[0].id;
    let resp = c.get_transaction(tx_id).await?.0;
    assert_eq!(resp.id, String::from(tx_id));
    Ok(())
  }

  #[rstest::rstest]
  #[tokio::test]
  async fn test_assets(config: Config) -> color_eyre::Result<()> {
    if !config.is_ok() {
      return Ok(());
    }
    let c = config.client();
    let results = c.assets(Vec::<(String, String)>::new()).await?.0;
    assert!(!results.asset_wallets.is_empty());
    Ok(())
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

    let (address_response, id) = config.client().create_address(result.id, ASSET_SOL_TEST).await?;
    assert!(!id.is_empty());
    assert!(!address_response.address.is_empty());
    assert!(!address_response.id.is_empty());
    let addr = address_response.address.clone();
    let (address_response, id) = config.client().addresses(result.id, "SOL_TEST").await?;
    assert!(!id.is_empty());
    assert_eq!(1, address_response.len());
    assert_eq!(addr, address_response[0].address);

    let page = PagingAddressRequestBuilder::new().limit(10).build()?;
    let (container, id) = config.client().addresses_paginated(result.id, "SOL_TEST", page).await?;
    assert!(!id.is_empty());
    assert_eq!(1, container.addresses.len());

    let rename = format!("{vault_name}-rename");
    config.client().rename_vault(result.id, &rename).await?;

    let after = &Utc.with_ymd_and_hms(2023, 4, 6, 0, 1, 1).unwrap();
    let before = &chrono::offset::Utc::now();
    PagingAddressRequestBuilder::new().limit(10).after(after).build()?;
    //config.client().addresses_paginated(0, ASSET_BTC_TEST, page).await?;

    PagingAddressRequestBuilder::new().limit(10).before(before).build()?;
    //config.client().addresses_paginated(0, ASSET_BTC_TEST, page).await?;
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
      .contract_asset(&contract_response.id, "ETH_TEST5", "0x9bb4d44e6963260a1850926e8f6beb8d5803836f")
      .await?;
    assert_eq!(addr_response.id, ASSET_ETH_TEST);

    config.client().contract_delete(&name).await?;
    config.client().contracts().await?;
    Ok(())
  }

  #[rstest::rstest]
  #[tokio::test]
  async fn test_external_wallet(config: Config) -> color_eyre::Result<()> {
    if !config.is_ok() {
      return Ok(());
    }
    let name = vault_name();
    let c = config.client();
    let (contract_response, id) = c.external_wallet_create(&name).await?;
    assert!(!id.is_empty());
    assert_eq!(contract_response.name, name);
    assert!(!contract_response.id.is_empty());

    let addr_response = c
      .external_wallet_asset(&contract_response.id, "ETH_TEST5", "0x9bb4d44e6963260a1850926e8f6beb8d5803836f")
      .await?
      .0;
    assert_eq!(addr_response.id, ASSET_ETH_TEST);

    let wallets = c.external_wallets().await?.0;
    assert!(!wallets.is_empty());
    c.external_wallet(&contract_response.id).await?;
    c.external_wallet_delete(&name).await?;
    Ok(())
  }

  #[rstest::rstest]
  #[tokio::test]
  async fn test_create_transaction(config: Config) -> color_eyre::Result<()> {
    if !config.is_ok() {
      return Ok(());
    }
    if !config.create_tx {
      warn!("not testing create transaction");
      return Ok(());
    }

    let c = config.client();
    let tx = c.create_transaction_vault(0, 1, ASSET_SOL_TEST, BigDecimal::from_str("0.001")?, None).await?.0;
    assert_eq!(tx.status, TransactionStatus::SUBMITTED);
    c.poll_transaction(&tx.id, time::Duration::from_secs(10), Duration::from_secs(5)).await?;

    c.create_transaction_external(
      0,
      "8q1DVf1j5bGCLkQBSrdwQkeJgKUdWjce8W4yab4S7hKR",
      ASSET_SOL_TEST,
      BigDecimal::from_str("0.0001")?,
      None,
    )
    .await?;
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
  async fn test_wallet_connections(config: Config) -> color_eyre::Result<()> {
    if !config.is_ok() {
      return Ok(());
    }
    config.client().wallet_connections().await?;
    Ok(())
  }

  #[rstest::rstest]
  #[tokio::test]
  async fn test_staking(config: Config) -> color_eyre::Result<()> {
    if !config.is_ok() {
      return Ok(());
    }
    let c = config.client();
    let chains = c.staking_chains().await?.0;
    assert!(!chains.is_empty());
    c.staking_positions().await?;
    c.staking_positions_summary().await?;
    Ok(())
  }

  #[rstest::rstest]
  #[tokio::test]
  async fn test_internal_wallets(config: Config) -> color_eyre::Result<()> {
    if !config.is_ok() {
      return Ok(());
    }
    let c = config.client();
    c.internal_wallets().await?;
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
