use crate::assets::Asset;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct WalletContainer {
  pub id: String,
  pub name: String,
  pub assets: Vec<ExternalWalletAsset>,
  pub customer_ref_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct ExternalWalletAsset {
  pub id: String,
  pub status: String,
  pub address: String,
  pub tag: Option<String>,
  pub activation_time: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
#[allow(dead_code)]
pub struct WalletCreateAssetResponse {
  pub id: Asset,
}
