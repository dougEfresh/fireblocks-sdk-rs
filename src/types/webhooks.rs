use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::{
  deserialize_epoch_time, deserialize_option_empty_object, AmountInfo, AuthorizationInfo, BlockInfo, ExtraParameters,
  FeeInfo, RewardInfo, SignedMessage, TransactionOperation, TransferPeerPath,
};

use super::TransactionStatus;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebhookEntry {
  #[serde(rename = "type")]
  pub webhook_type: String,
  pub tenant_id: String,
  pub timestamp: i64,
  pub data: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionDetails {
  pub id: String,
  #[serde(deserialize_with = "deserialize_option_empty_object", default)]
  pub external_tx_id: Option<String>,
  pub status: TransactionStatus,
  #[serde(deserialize_with = "deserialize_option_empty_object", default)]
  pub sub_status: Option<String>,
  #[serde(deserialize_with = "deserialize_option_empty_object", default)]
  pub tx_hash: Option<String>,
  pub operation: TransactionOperation,
  pub note: String,
  pub asset_id: String,
  pub asset_type: AssetType,
  pub source: TransferPeerPath,
  #[serde(deserialize_with = "deserialize_option_empty_object", default)]
  pub source_address: Option<String>,
  pub destination: TransferPeerPath,
  pub destinations: Vec<DestinationsResponse>,
  #[serde(deserialize_with = "deserialize_option_empty_object", default)]
  pub destination_address: Option<String>,
  #[serde(deserialize_with = "deserialize_option_empty_object", default)]
  pub destination_address_description: Option<String>,
  #[serde(deserialize_with = "deserialize_option_empty_object", default)]
  pub destination_tag: Option<String>,
  #[serde(deserialize_with = "deserialize_option_empty_object", default)]
  pub amount_info: Option<AmountInfo>,
  pub treat_as_gross_amount: Option<bool>,
  #[serde(deserialize_with = "deserialize_option_empty_object", default)]
  pub fee_info: Option<FeeInfo>,
  pub fee_currency: String,
  pub network_records: Option<Vec<serde_json::Value>>,
  #[serde(deserialize_with = "deserialize_epoch_time")]
  pub created_at: DateTime<Utc>,
  #[serde(deserialize_with = "deserialize_epoch_time")]
  pub last_updated: DateTime<Utc>,
  #[serde(deserialize_with = "deserialize_option_empty_object", default)]
  pub created_by: Option<String>,
  #[serde(deserialize_with = "deserialize_option_empty_object", default)]
  pub signed_by: Option<Vec<String>>,
  #[serde(deserialize_with = "deserialize_option_empty_object", default)]
  pub rejected_by: Option<String>,
  #[serde(deserialize_with = "deserialize_option_empty_object", default)]
  pub authorization_info: Option<AuthorizationInfo>,
  #[serde(deserialize_with = "deserialize_option_empty_object", default)]
  pub customer_ref_id: Option<String>,
  #[serde(deserialize_with = "deserialize_option_empty_object", default)]
  pub aml_screening_result: Option<AmlScreeningResult>,
  #[serde(deserialize_with = "deserialize_option_empty_object", default)]
  pub replaced_tx_hash: Option<String>,
  #[serde(deserialize_with = "deserialize_option_empty_object", default)]
  pub extra_parameters: Option<ExtraParameters>,
  #[serde(deserialize_with = "deserialize_option_empty_object", default)]
  pub signed_messages: Option<Vec<SignedMessage>>,
  #[serde(deserialize_with = "deserialize_option_empty_object", default)]
  pub num_of_confirmations: Option<i64>,
  #[serde(deserialize_with = "deserialize_option_empty_object", default)]
  pub block_info: Option<BlockInfo>,
  #[serde(deserialize_with = "deserialize_option_empty_object", default)]
  pub index: Option<i64>,
  #[serde(deserialize_with = "deserialize_option_empty_object", default)]
  pub rewards_info: Option<RewardInfo>,
  #[serde(deserialize_with = "deserialize_option_empty_object", default)]
  pub system_messages: Option<Vec<SystemMessage>>,
  #[serde(deserialize_with = "deserialize_option_empty_object", default)]
  pub address_type: Option<String>,

  // Deprecated
  pub requested_amount: Option<BigDecimal>,
  pub amount: Option<BigDecimal>,
  pub net_amount: Option<BigDecimal>,
  #[serde(rename = "amountUSD")]
  pub amount_usd: Option<BigDecimal>,
  pub service_fee: Option<BigDecimal>,
  pub network_fee: Option<BigDecimal>,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
#[allow(clippy::upper_case_acronyms)]
#[cfg_attr(feature = "sql", derive(sqlx::Type))]
#[cfg_attr(feature = "sql", sqlx(type_name = "fb_asset_type", rename_all = "lowercase"))]
pub enum AssetType {
  XLM_ASSET,
  XDB_ASSET,
  TRON_TRC20,
  SOL_ASSET,
  HBAR_ERC20,
  FIAT,
  ERC721,
  ERC20,
  ERC1155,
  BEP20,
  #[default]
  BASE_ASSET,
  ALGO_ASSET,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinationsResponse {
  pub amount: String,
  pub destination: TransferPeerPath,
  pub amount_usd: Option<BigDecimal>,
  pub destination_address: Option<String>,
  pub destination_address_description: Option<String>,
  pub aml_screening_result: Option<AmlScreeningResult>,
  pub customer_ref_id: Option<String>,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
#[allow(clippy::upper_case_acronyms)]
#[cfg_attr(feature = "sql", derive(sqlx::Type))]
#[cfg_attr(feature = "sql", sqlx(type_name = "fb_system_message_type", rename_all = "lowercase"))]
pub enum SystemMessageType {
  #[default]
  WARN,
  BLOCK,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemMessage {
  #[serde(rename = "type")]
  pub message_type: SystemMessageType,
  pub message: String,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
#[allow(clippy::upper_case_acronyms)]
#[cfg_attr(feature = "sql", derive(sqlx::Type))]
#[cfg_attr(feature = "sql", sqlx(type_name = "fb_aml_screening_verdict", rename_all = "lowercase"))]
pub enum AmlScreeningVerdict {
  ACCEPT,
  REJECT,
  #[default]
  ALERT,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AmlScreeningResult {
  #[serde(deserialize_with = "deserialize_option_empty_object", default)]
  pub provider: Option<String>,
  pub payload: serde_json::Value,
  pub screening_status: String,
  #[serde(deserialize_with = "deserialize_option_empty_object", default)]
  pub bypass_reason: Option<String>,
  #[serde(deserialize_with = "deserialize_epoch_time")]
  pub timestamp: DateTime<Utc>,
  #[serde(deserialize_with = "deserialize_option_empty_object", default)]
  pub category: Option<String>,
  #[serde(deserialize_with = "deserialize_option_empty_object", default)]
  pub category_id: Option<i64>,
  #[serde(deserialize_with = "deserialize_option_empty_object", default)]
  pub customer_ref_id: Option<String>,
  #[serde(deserialize_with = "deserialize_option_empty_object", default)]
  pub dest_address: Option<String>,
  #[serde(deserialize_with = "deserialize_option_empty_object", default)]
  pub external_id: Option<String>,
  pub matched_alert: Option<serde_json::Value>,
  pub matched_rule: Option<serde_json::Value>,
  pub matched_prescreening_rule: Option<serde_json::Value>,
  pub risk: Option<serde_json::Value>,
  #[serde(deserialize_with = "deserialize_option_empty_object", default)]
  pub verdict: Option<AmlScreeningVerdict>,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
#[allow(clippy::upper_case_acronyms)]
#[cfg_attr(feature = "sql", derive(sqlx::Type))]
#[cfg_attr(feature = "sql", sqlx(type_name = "fb_network_status", rename_all = "lowercase"))]
pub enum NetworkStatus {
  DROPPED,
  #[default]
  BROADCASTING,
  CONFIRMING,
  FAILED,
  CONFIRMED,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkRecord {
  pub source: TransferPeerPath,
  pub destination: TransferPeerPath,
  pub tx_hash: Option<String>,
  pub network_fee: Option<BigDecimal>,
  pub asset_id: String,
  pub net_amount: Option<BigDecimal>,
  pub status: NetworkStatus,
  #[serde(rename = "type")]
  pub op_type: Option<String>,
  pub destination_address: Option<String>,
  pub source_address: Option<String>,
}

#[cfg(test)]
mod test {
  use super::*;
  use std::fs;

  #[test]
  fn test_transaction_details_deserialization() {
    for dir_result in fs::read_dir("./data/webhooks/transactions").unwrap() {
      let path = dir_result.unwrap().path();

      let data = fs::read_to_string(path.clone()).expect("Unable to read file");

      let webhook_entry: WebhookEntry = serde_json::from_str(&data).unwrap();
      let _webhook_data: TransactionDetails = serde_json::from_value(webhook_entry.data.clone())
        .expect(&format!("deserialization failed for path: {}", path.to_str().unwrap()));
    }
  }
}
