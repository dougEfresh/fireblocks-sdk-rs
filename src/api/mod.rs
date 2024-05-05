use serde_derive::{Deserialize, Serialize};

mod contracts;
mod external_wallets;
mod hooks;
mod internal_wallets;
mod staking;
mod transactions;
mod vaults;
mod wallet_connect;

#[derive(Debug, Serialize)]
struct WalletCreate {
  name: String,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct WalletCreateAsset {
  pub address: String,
  pub tag: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct Success {
  pub success: bool,
}
