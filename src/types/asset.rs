use bigdecimal::BigDecimal;
use serde_derive::{Deserialize, Serialize};

use crate::types::deserialize_str_u64;

pub enum Asset {
  ETH(SupportedAsset),
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct SupportedAsset {
  pub id: String,
  pub name: String,
  #[serde(rename = "type")]
  pub asset_type: String,
  pub contract_address: String,
  pub native_asset: String,
  pub decimals: i32,
}

impl SupportedAsset {
  pub fn name(&self) -> &str {
    &self.name
  }

  pub fn asset_type(&self) -> &str {
    &self.asset_type
  }

  pub fn contract_address(&self) -> &str {
    &self.contract_address
  }

  pub fn native_asset(&self) -> &str {
    &self.native_asset
  }

  pub fn decimals(&self) -> i32 {
    self.decimals
  }

  pub fn id(&self) -> &str {
    &self.id
  }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct AssetResponse {
  #[serde(deserialize_with = "deserialize_str_u64", default)]
  pub vault_id: u64,
  pub asset_id: String,
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
#[allow(dead_code)]
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
