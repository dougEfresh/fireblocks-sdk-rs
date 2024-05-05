use crate::assets::Asset;
use crate::types::deserialize_str_u64;
use bigdecimal::BigDecimal;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SupportedAsset {
  pub id: Asset,
  pub name: String,
  #[serde(rename = "type")]
  pub asset_type: String,
  pub contract_address: String,
  pub native_asset: String,
  pub decimals: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetResponse {
  #[serde(deserialize_with = "deserialize_str_u64", default)]
  pub vault_id: u64,
  pub asset_id: Asset,
  pub total: BigDecimal,
  pub locked_amount: BigDecimal,
  pub available: BigDecimal,
  pub pending: BigDecimal,
  pub frozen: BigDecimal,
  pub block_height: Option<String>,
  pub block_hash: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountAsset {
  pub id: String,
  pub total: BigDecimal,
  pub available: BigDecimal,
  pub pending: BigDecimal,
  pub frozen: BigDecimal,
  pub locked_amount: BigDecimal,
  pub staked: BigDecimal,

  #[serde(rename = "totalStakedCPU")]
  pub total_staked_cpu: Option<BigDecimal>,
  pub total_staked_network: Option<String>,
  #[serde(rename = "selfStakedCPU")]
  pub self_staked_cpu: Option<BigDecimal>,
  pub self_staked_network: Option<String>,
  #[serde(rename = "pendingRefundCPU")]
  pub pending_refund_cpu: Option<BigDecimal>,
  pub pending_refund_network: Option<String>,

  pub block_height: Option<String>,
  pub block_hash: Option<String>,
}
