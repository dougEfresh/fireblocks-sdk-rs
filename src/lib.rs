#![doc = include_str!("../README.md")]
use {
    chrono::{DateTime, Utc},
    std::fmt::Display,
};
mod assets;
mod client;
pub mod error;
pub(crate) mod jwt;
mod log;
#[cfg(feature = "page")]
mod paged_client;
mod wallet;
pub use wallet::WalletContainer;

#[cfg(feature = "page")]
pub use paged_client::{PagedClient, TransactionStream, VaultStream};
pub use {
    crate::error::*,
    apis::{
        configuration::{ApiKey, Configuration},
        ApiClient,
    },
    assets::{
        Asset, ASSET_BTC, ASSET_BTC_TEST, ASSET_ETH, ASSET_ETH_TEST, ASSET_SOL, ASSET_SOL_TEST,
    },
    client::{Client, ClientBuilder},
};

pub const FIREBLOCKS_API: &str = "https://api.fireblocks.io/v1";
pub const FIREBLOCKS_SANDBOX_API: &str = "https://sandbox-api.fireblocks.io/v1";
pub type Epoch = DateTime<Utc>;
pub type Result<T> = std::result::Result<T, FireblocksError>;
pub type QueryParams = Vec<(String, String)>;

#[derive(Debug, Clone)]
pub enum WalletType {
    Internal,
    External,
    Contract,
}

impl Display for WalletType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Self::Internal => "Internal",
                Self::External => "External",
                Self::Contract => "Contract",
            }
        )
    }
}

#[allow(clippy::all, clippy::pedantic, clippy::nursery)]
pub mod apis;
#[allow(clippy::all, clippy::pedantic, clippy::nursery)]
pub mod models;

//#[cfg(test)]
// mod tests {
//  use std::sync::{Arc, Once, OnceLock};
//  use std::time::Duration;
//
//  use crate::assets::{ASSET_BTC_TEST, ASSET_SOL_TEST};
//  use crate::paged_client::{PagedClient, TransactionStream};
//  use crate::types::*;
//  use crate::{Client, ClientBuilder, ASSET_ETH, ASSET_ETH_TEST, ASSET_SOL};
//  use bigdecimal::BigDecimal;
//  use chrono::{TimeZone, Utc};
//  use anyhow::eyre::format_err;
//  use tokio::time;
//  use tokio_stream::StreamExt;
//  use tracing::warn;
//  use tracing_subscriber::fmt::format::FmtSpan;
//  use tracing_subscriber::EnvFilter;
//

//  #[rstest::rstest]
//  #[tokio::test]
//  async fn test_vaults(config: Config) -> anyhow::Result<()> {
//    if !config.is_ok() {
//      return Ok(());
//    }
//    let params = PagingVaultRequestBuilder::new().build()?;
//    let (results, id) = config.client().vaults(params).await?;
//    assert!(!id.is_empty());
//    assert!(!results.accounts.is_empty());
//
//    let params =
// PagingVaultRequestBuilder::new().min_threshold(&BigDecimal::from_str("
// 1000000.00")?).build()?;    let (results, id) =
// config.client().vaults(params).await?;    assert!(!id.is_empty());
//    assert!(results.accounts.is_empty());
//
//    let params = PagingVaultRequestBuilder::new().limit(1).build()?;
//    let (results, id) = config.client().vaults(params).await?;
//    assert!(!id.is_empty());
//    assert_eq!(1, results.accounts.len());
//
//    let (result, id) = config.client().vault(0).await?;
//    assert!(!id.is_empty());
//    assert_eq!(0, result.id);
//    assert!(!result.assets.is_empty());
//
//    let _ = PagingVaultRequestBuilder::new().before("before").build(); // code
// coverage    Ok(())
//  }
//
//  fn vault_name() -> String {
//    format!("z-test-{}", Utc::now().timestamp_millis())
//  }
//  #[rstest::rstest]
//  #[tokio::test]
//  #[allow(clippy::unwrap_used)]
//  async fn test_transaction_list(config: Config) -> anyhow::Result<()> {
//    let after = Utc::now();
//    let before = Utc::now();
//    // test all options
//    let options = TransactionListBuilder::new()
//      .after(&after)
//      .before(&before)
//      .assets(&[ASSET_BTC_TEST, ASSET_SOL_TEST])
//      .tx_hash("something")
//      .source_id(9)
//      .destination_id(19)
//      .limit(200)
//      .build()?;
//
//    let v = options.iter().find(|(a, _)| *a == "after");
//    assert!(v.is_some());
//
//    let v = options.iter().find(|(a, _)| *a == "before");
//    assert!(v.is_some());
//
//    let v = options.iter().find(|(a, _)| *a == "assets");
//    assert!(v.is_some());
//    assert_eq!("BTC_TEST,SOL_TEST", v.unwrap().1);
//
//    let v = options.iter().find(|(a, _)| *a == "sourceId");
//    assert!(v.is_some());
//    assert_eq!("9", v.unwrap().1);
//
//    let v = options.iter().find(|(a, _)| *a == "destId");
//    assert!(v.is_some());
//    assert_eq!("19", v.unwrap().1);
//
//    let v = options.iter().find(|(a, _)| *a == "limit");
//    assert!(v.is_some());
//    assert_eq!("200", v.unwrap().1);
//
//    let options = TransactionListBuilder::new().assets(&[ASSET_BTC_TEST,
// ASSET_SOL_TEST]).limit(200).build()?;    if !config.is_ok() {
//      return Ok(());
//    }
//    let c = config.client();
//    let transactions = c.transactions(options).await?.0;
//    assert!(!transactions.is_empty());
//    let tx_id = &transactions[0].id;
//    let resp = c.get_transaction(tx_id).await?.0;
//    assert_eq!(resp.id, String::from(tx_id));
//    Ok(())
//  }
//
//  #[rstest::rstest]
//  #[tokio::test]
//  async fn test_assets(config: Config) -> anyhow::Result<()> {
//    if !config.is_ok() {
//      return Ok(());
//    }
//    let c = config.client();
//    let results = c.assets(Vec::<(String, String)>::new()).await?.0;
//    assert!(!results.asset_wallets.is_empty());
//    Ok(())
//  }
//
//  #[rstest::rstest]
//  #[tokio::test]
//  async fn test_create_vaults(config: Config) -> anyhow::Result<()> {
//    if !config.is_ok() {
//      return Ok(());
//    }
//    let vault_name: String = vault_name();
//
//    let account = CreateAccount {
//      auto_fuel: false,
//      customer_ref_id: Some("fireblocks-sdk-rs".to_string()),
//      hidden_on_ui: true,
//      name: vault_name.clone(),
//    };
//    let c = config.client();
//    let (result, id) = c.create_vault(&account).await?;
//    assert!(!id.is_empty());
//    assert_eq!(account.name, result.name);
//    assert!(result.hidden_on_ui);
//    assert!(result.id > 0);
//
//    let (address_response, id) = c.create_address(result.id,
// ASSET_SOL_TEST).await?;    assert!(!id.is_empty());
//    assert!(!address_response.address.is_empty());
//    assert!(!address_response.id.is_empty());
//    let addr = address_response.address.clone();
//    let (address_response, id) = c.addresses(result.id, "SOL_TEST").await?;
//    assert!(!id.is_empty());
//    assert_eq!(1, address_response.len());
//    assert_eq!(addr, address_response[0].address);
//
//    let page = PagingAddressRequestBuilder::new().limit(10).build()?;
//    let (container, id) = c.addresses_paginated(result.id, "SOL_TEST",
// page).await?;    assert!(!id.is_empty());
//    assert_eq!(1, container.addresses.len());
//
//    let rename = format!("{vault_name}-rename");
//    c.rename_vault(result.id, &rename).await?;
//
//    PagingAddressRequestBuilder::new().limit(10).after("after").before("
// before").build()?; // code coverage    c.vault_hide(result.id, false).await?;
//    c.vault_hide(result.id, true).await?;
//    Ok(())
//  }
//
//  #[rstest::rstest]
//  #[tokio::test]
//  async fn test_wallet_contract(config: Config) -> anyhow::Result<()> {
//    if !config.is_ok() {
//      return Ok(());
//    }
//    let c = config.client();
//    let name = format!("c-{}", vault_name());
//    let (contract_response, id) = c.contract_create(&name).await?;
//    assert!(!id.is_empty());
//    assert_eq!(contract_response.name, name);
//    assert!(!contract_response.id.is_empty());
//
//    let (addr_response, _) =
//      c.contract_asset(&contract_response.id, ASSET_ETH_TEST,
// "0x9bb4d44e6963260a1850926e8f6beb8d5803836f").await?;    assert_eq!
// (addr_response.id, ASSET_ETH_TEST);    c.contract(&contract_response.id).
// await?;    c.contract_delete(&id).await?;
//    c.contracts().await?;
//    Ok(())
//  }
//
//  #[rstest::rstest]
//  #[tokio::test]
//  async fn test_external_wallet(config: Config) -> anyhow::Result<()> {
//    if !config.is_ok() {
//      return Ok(());
//    }
//    let name = vault_name();
//    let c = config.client();
//    let (contract_response, id) = c.external_wallet_create(&name).await?;
//    assert!(!id.is_empty());
//    assert_eq!(contract_response.name, name);
//    assert!(!contract_response.id.is_empty());
//
//    let addr_response = c
//      .external_wallet_asset(&contract_response.id, ASSET_ETH_TEST,
// "0x9bb4d44e6963260a1850926e8f6beb8d5803836f")      .await?
//      .0;
//    assert_eq!(addr_response.id, ASSET_ETH_TEST);
//
//    let wallets = c.external_wallets().await?.0;
//    assert!(!wallets.is_empty());
//    c.external_wallet(&contract_response.id).await?;
//    c.external_wallet_delete(&contract_response.id).await?;
//    let found = c.internal_wallets().await?.0.into_iter().find(|w| w.id ==
// contract_response.id);    assert!(found.is_none());
//    Ok(())
//  }
//
//  #[rstest::rstest]
//  #[tokio::test]
//  async fn test_internal_wallet(config: Config) -> anyhow::Result<()> {
//    if !config.is_ok() {
//      return Ok(());
//    }
//    let name = vault_name();
//    let c = config.client();
//    let w = c.internal_wallets().await?.0;
//    for mine in &w {
//      if mine.name.starts_with("z-test-") {
//        c.internal_wallet_delete(&mine.id).await?;
//      }
//    }
//    let w = c.external_wallets().await?.0;
//    for mine in &w {
//      if mine.name.starts_with("z-test-") {
//        c.external_wallet_delete(&mine.id).await?;
//      }
//    }
//    let (wallet, id) = c.internal_wallet_create(&name).await?;
//    assert!(!id.is_empty());
//    assert_eq!(wallet.name, name);
//    assert!(!wallet.id.is_empty());
//
//    let addr_response =
//      c.internal_wallet_asset(&wallet.id, ASSET_ETH_TEST,
// "0x9bb4d44e6963260a1850926e8f6beb8d5803836f").await?.0;    assert_eq!
// (addr_response.id, ASSET_ETH_TEST);
//
//    let wallets = c.internal_wallets().await?.0;
//    assert!(!wallets.is_empty());
//    c.internal_wallet(&wallet.id).await?;
//    c.internal_wallet_delete(&wallet.id).await?;
//    let found = c.internal_wallets().await?.0.into_iter().find(|w| w.id ==
// wallet.id);    assert!(found.is_none());
//    Ok(())
//  }
//
//  #[rstest::rstest]
//  #[tokio::test]
//  async fn test_create_transaction_whitelist(config: Config) ->
// anyhow::Result<()> {    if !config.is_ok() {
//      return Ok(());
//    }
//    if !config.create_tx {
//      warn!("not testing create transaction");
//      return Ok(());
//    }
//    let c = config.client();
//    let wallet = c.internal_wallets().await?.0.into_iter().find(|w| w.name ==
// "test-whitelist");    let id: String = match wallet {
//      None => {
//        let id = c.internal_wallet_create("test-whitelist").await?.0.id;
//        c.internal_wallet_asset(&id, ASSET_SOL_TEST,
// "E4SfgGV2v9GLYsEkCQhrrnFbBcYmAiUZZbJ7swKGzZHJ").await?;        id
//      },
//      Some(w) => w.id,
//    };
//    let tx = c
//      .create_transaction_peer(
//        0,
//        &id,
//        PeerType::INTERNAL_WALLET,
//        ASSET_SOL_TEST,
//        BigDecimal::from_str("0.00001")?,
//        None,
//      )
//      .await?
//      .0;
//    c.poll_transaction(&tx.id, Duration::from_secs(50),
// Duration::from_secs(5), |t: &Transaction| {      tracing::info!("{:#?}",
// t.status);    })
//    .await?;
//    Ok(())
//  }
//
//  #[rstest::rstest]
//  #[tokio::test]
//  async fn test_create_transaction(config: Config) -> anyhow::Result<()> {
//    if !config.is_ok() {
//      return Ok(());
//    }
//    if !config.create_tx {
//      warn!("not testing create transaction");
//      return Ok(());
//    }
//
//    let c = config.client();
//
//    let tx = c.create_transaction_vault(0, 1, ASSET_SOL_TEST,
// BigDecimal::from_str("0.001")?, None).await?.0;    assert_eq!(tx.status,
// TransactionStatus::SUBMITTED);    c.poll_transaction(&tx.id,
// Duration::from_secs(10), Duration::from_secs(5), |t: &Transaction| {
//      tracing::info!("{:#?}", t.status);
//    })
//    .await?;
//
//    c.create_transaction_external(
//      0,
//      "8q1DVf1j5bGCLkQBSrdwQkeJgKUdWjce8W4yab4S7hKR",
//      ASSET_SOL_TEST,
//      BigDecimal::from_str("0.0001")?,
//      None,
//    )
//    .await?;
//
//    let args = &TransactionArguments {
//      asset_id: "SOL_TEST".to_string(),
//      operation: TransactionOperation::TRANSFER,
//      source: TransferPeerPath { id: Some("0".to_string()),
// ..Default::default() },      destination: Some(DestinationTransferPeerPath {
// id: "4".to_string(), ..Default::default() }),      amount:
// "0.001".to_string(),      gas_price: None,
//      gas_limit: None,
//      note: "created by fireblocks-sdk for rust".to_string(),
//    };
//
//    c.estimate_fee_transaction(args).await?;
//    Ok(())
//  }
//
//  #[rstest::rstest]
//  #[tokio::test]
//  async fn test_estimate_fees(config: Config) -> anyhow::Result<()> {
//    if !config.is_ok() {
//      return Ok(());
//    }
//    let (result, id) = config.client().estimate_fee("BTC_TEST").await?;
//    assert!(!id.is_empty());
//    assert!(result.low.gas_price.is_none());
//    assert!(result.medium.gas_price.is_none());
//    assert!(result.high.gas_price.is_none());
//
//    assert!(result.low.network_fee.is_none());
//    assert!(result.medium.network_fee.is_none());
//    assert!(result.high.network_fee.is_none());
//
//    assert!(result.low.fee_per_byte.is_some());
//    assert!(result.medium.fee_per_byte.is_some());
//    assert!(result.high.fee_per_byte.is_some());
//    Ok(())
//  }
//
//  #[test]
//  fn test_handle_not_present() -> anyhow::Result<()> {
//    let data = r#"{ "type": "VAULT_ACCOUNT","name": "jupiter","subType":
// ""}"#;    let result: TransferPeerPath = serde_json::from_str(data)?;
//    assert!(result.id.is_none());
//    Ok(())
//  }
//
//  #[rstest::rstest]
//  #[tokio::test]
//  async fn test_staking(config: Config) -> anyhow::Result<()> {
//    if !config.is_ok() {
//      return Ok(());
//    }
//    let c = config.client();
//    let chains = c.staking_chains().await?.0;
//    assert!(!chains.is_empty());
//    c.staking_positions().await?;
//    c.staking_positions_summary().await?;
//    let providers = c.staking_providers().await?.0;
//    assert!(!providers.is_empty());
//    for p in providers {
//      c.staking_accept_terms(&p.id).await?;
//    }
//
//    for chain in [ASSET_SOL, ASSET_SOL_TEST, ASSET_ETH, ASSET_ETH_TEST] {
//      c.staking_chain_info(&chain).await?;
//    }
//    Ok(())
//  }
//
//  #[rstest::rstest]
//  #[tokio::test]
//  async fn test_paged_vaults(config: Config) -> anyhow::Result<()> {
//    if !config.is_ok() {
//      return Ok(());
//    }
//    let c = config.client();
//    let pc = PagedClient::new(Arc::new(c));
//    let mut vs = pc.vaults(100);
//
//    while let Ok(Some(result)) = vs.try_next().await {
//      tracing::info!("accounts {}", result.0.accounts.len());
//      time::sleep(Duration::from_millis(200)).await;
//    }
//    Ok(())
//  }
//

//  #[rstest::rstest]
//  #[tokio::test]
//  async fn test_hooks(config: Config) -> anyhow::Result<()> {
//    if !config.is_ok() {
//      return Ok(());
//    }
//    let c = config.client();
//    let result = c.hooks_resend().await;
//    if let Err(e) = result {
//      assert!(e.to_string().contains("Internal Fireblocks Error"), "{}",
// e.to_string());    }
//    match c.hooks_resend_tx("e01b1c68-2d26-45dc-bb02-4cc9152295e1", true,
// true).await {      Err(e) => {
//        assert!(e.to_string().contains("Internal Fireblocks Error"), "{}",
// e.to_string());      },
//      Ok(result) => {
//        assert!(result.0.success);
//      },
//    }
//    Ok(())
//  }
//
//
//  #[rstest::rstest]
//  #[test]
//  fn check_ci(config: Config) -> anyhow::Result<()> {
//    match std::env::var("CI") {
//      Err(_) => Ok(()),
//      Ok(_) => match config.client {
//        Some(_) => Ok(()),
//        None => Err(format_err!("client is not configured and you are running
// in CI")),      },
//    }
//  }
//}
