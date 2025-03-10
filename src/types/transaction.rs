use crate::assets::Asset;
use crate::{impl_base_query_params, Epoch, QueryParams};
use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_derive::Serialize;
use std::borrow::Borrow;

use crate::types::page::BasePageParams;
use crate::types::{deserialize_epoch_time, deserialize_option_empty_object};

#[allow(clippy::upper_case_acronyms)]
#[allow(non_camel_case_types)]
#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "sql", derive(sqlx::Type))]
#[cfg_attr(feature = "sql", sqlx(type_name = "peer_type", rename_all = "lowercase"))]
pub enum PeerType {
  #[default]
  VAULT_ACCOUNT,
  EXCHANGE_ACCOUNT,
  INTERNAL_WALLET,
  EXTERNAL_WALLET,
  FIAT_ACCOUNT,
  NETWORK_CONNECTION,
  COMPOUND,
  CONTRACT,
  UNKNOWN,
  GAS_STATION,
  END_USER_WALLET,
  ONE_TIME_ADDRESS,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
#[allow(clippy::upper_case_acronyms)]
#[cfg_attr(feature = "sql", derive(sqlx::Type))]
#[cfg_attr(feature = "sql", sqlx(type_name = "transaction_operation_type", rename_all = "lowercase"))]
pub enum TransactionOperation {
  #[default]
  TRANSFER,
  RAW,
  CONTRACT_CALL,
  MINT,
  BURN,
  SUPPLY_TO_COMPOUND,
  REDEEM_FROM_COMPOUND,
  TYPED_MESSAGE,
}

#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "sql", derive(sqlx::Type))]
#[cfg_attr(feature = "sql", sqlx(type_name = "transaction_status", rename_all = "lowercase"))]
pub enum TransactionStatus {
  SUBMITTED,
  QUEUED,
  PENDING_SIGNATURE,
  PENDING_AUTHORIZATION,
  PENDING_3RD_PARTY_MANUAL_APPROVAL,
  PENDING_3RD_PARTY,
  BROADCASTING,
  CONFIRMING,
  #[default]
  COMPLETED,
  PENDING_AML_SCREENING,
  PARTIALLY_COMPLETED,
  CANCELLING,
  CANCELLED,
  REJECTED,
  FAILED,
  TIMEOUT,
  BLOCKED,
  UNKNOWN,
}

/// Search for transactions
///
/// [getTransactions](https://docs.fireblocks.com/api/swagger-ui/#/Transactions/getTransactions)
#[derive(Debug, Default)]
pub struct TransactionListBuilder {
  params: QueryParams,
  base: BasePageParams,
}

impl_base_query_params!(TransactionListBuilder);

impl TransactionListBuilder {
  pub fn source_id(&mut self, id: i32) -> &mut Self {
    self.params.push(("sourceId".to_string(), id.to_string()));
    self
  }

  pub fn sort_desc(&mut self) -> &mut Self {
    self.params.push(("sort".to_string(), String::from("DESC")));
    self
  }

  pub fn sort_asc(&mut self) -> &mut Self {
    self.params.push(("sort".to_string(), String::from("ASC")));
    self
  }

  pub fn order_created_at(&mut self) -> &mut Self {
    self.params.push(("orderBy".to_string(), String::from("createdAt")));
    self
  }

  pub fn order_last_updated(&mut self) -> &mut Self {
    self.params.push(("orderBy".to_string(), String::from("lastUpdated")));
    self
  }

  pub fn hash(&mut self, s: &str) -> &mut Self {
    self.params.push(("txHash".to_string(), String::from(s)));
    self
  }

  pub fn destination_id(&mut self, id: i32) -> &mut Self {
    self.params.push(("destId".to_string(), id.to_string()));
    self
  }

  pub fn assets<T: Borrow<str>>(&mut self, a: &[T]) -> &mut Self {
    self.params.push(("assets".to_owned(), a.join(",")));
    self
  }

  pub fn before(&mut self, t: &Epoch) -> &mut Self {
    self.add_instant("before", t)
  }

  pub fn after(&mut self, t: &Epoch) -> &mut Self {
    self.add_instant("after", t)
  }

  fn add_instant(&mut self, param: &str, t: &Epoch) -> &mut Self {
    self.params.push((param.to_owned(), Self::epoch(t)));
    self
  }

  fn epoch(before: &Epoch) -> String {
    format!("{}", before.timestamp_millis())
  }

  /// Alias to [`TransactionListBuilder::hash`]
  pub fn tx_hash(&mut self, tx: &str) -> &mut Self {
    self.hash(tx)
  }
}

#[allow(clippy::upper_case_acronyms)]
#[allow(non_camel_case_types)]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "sql", derive(sqlx::Type))]
#[cfg_attr(feature = "sql", sqlx(type_name = "virtual_type", rename_all = "lowercase"))]
pub enum VirtualType {
  OFF_EXCHANGE,
  #[default]
  DEFAULT,
  OEC_FEE_BANK,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TransferPeerPath {
  #[serde(deserialize_with = "deserialize_option_empty_object", default)]
  pub id: Option<String>,
  #[serde(rename = "type")]
  pub peer_type: PeerType,
  pub name: String,
  #[serde(deserialize_with = "deserialize_option_empty_object", default)]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub sub_type: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub virtual_type: Option<VirtualType>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub virtual_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub wallet_id: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AmountInfo {
  pub amount: Option<BigDecimal>,
  pub requested_amount: Option<BigDecimal>,
  pub net_amount: Option<BigDecimal>,
  #[serde(rename = "amountUSD")]
  pub amount_usd: Option<BigDecimal>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BlockInfo {
  pub block_hash: Option<String>,
  //#[serde(deserialize_with = "deserialize_str_u64_opt")]
  // TODO convert to u64
  pub block_height: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FeeInfo {
  pub network_fee: Option<BigDecimal>,
  pub service_fee: Option<BigDecimal>,
  pub gas_price: Option<BigDecimal>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RewardInfo {
  pub src_rewards: Option<String>,
  pub dest_rewards: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FeePayerInfo {
  pub fee_payer_account_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Logic {
  OR,
  AND,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizationInfo {
  pub allow_operator_as_authorizer: bool,
  pub logic: Logic,
}
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Default)]
#[allow(non_camel_case_types)]
#[cfg_attr(feature = "sql", derive(sqlx::Type))]
#[cfg_attr(feature = "sql", sqlx(type_name = "signing_algorithm", rename_all = "lowercase"))]
pub enum SigningAlgorithm {
  #[default]
  MPC_ECDSA_SECP256K1,
  MPC_ECDSA_SECP256R1,
  MPC_EDDSA_ED25519,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Signature {
  pub full_sig: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SignedMessage {
  pub derivation_path: Vec<u64>,
  pub algorithm: SigningAlgorithm,
  pub public_key: String,
  pub signature: Signature,
}

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OneTimeAddress {
  pub address: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub tag: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinationTransferPeerPath {
  #[serde(rename = "type")]
  pub peer_type: PeerType,
  pub id: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub wallet_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub virtual_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub virtual_type: Option<VirtualType>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub one_time_address: Option<OneTimeAddress>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TransactionDestination {
  amount: BigDecimal,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
  pub id: String,
  pub asset_id: String,
  pub status: TransactionStatus,
  pub destination: Option<TransferPeerPath>,
  pub source: Option<TransferPeerPath>,
  pub amount: Option<BigDecimal>,
  pub network_fee: Option<BigDecimal>,
  #[serde(rename = "amountUSD")]
  pub amount_usd: Option<BigDecimal>,
  pub net_amount: Option<BigDecimal>,
  #[serde(deserialize_with = "deserialize_epoch_time")]
  pub created_at: DateTime<Utc>,
  #[serde(deserialize_with = "deserialize_epoch_time")]
  pub last_updated: DateTime<Utc>,
  #[serde(deserialize_with = "deserialize_option_empty_object", default)]
  pub tx_hash: Option<String>,
  pub num_of_confirmations: Option<i64>,
  #[serde(deserialize_with = "deserialize_option_empty_object", default)]
  pub sub_status: Option<String>,
  pub signed_by: Vec<String>,
  #[serde(deserialize_with = "deserialize_option_empty_object", default)]
  pub created_by: Option<String>,
  pub rejected_by: Option<String>,
  #[serde(deserialize_with = "deserialize_option_empty_object", default)]
  pub destination_address: Option<String>,
  #[serde(deserialize_with = "deserialize_option_empty_object", default)]
  pub source_address: Option<String>,
  #[serde(deserialize_with = "deserialize_option_empty_object", default)]
  pub destination_address_description: Option<String>,
  #[serde(deserialize_with = "deserialize_option_empty_object", default)]
  pub destination_tag: Option<String>,
  #[serde(deserialize_with = "deserialize_option_empty_object", default)]
  pub address_type: Option<String>,
  pub note: String,
  #[serde(deserialize_with = "deserialize_option_empty_object", default)]
  pub exchange_tx_id: Option<String>,
  pub requested_amount: Option<BigDecimal>,
  pub service_fee: Option<BigDecimal>,
  pub fee_currency: String,

  // amlScreeningResult?: AmlScreeningResult;
  pub customer_ref_id: Option<String>,
  #[serde(deserialize_with = "deserialize_option_empty_object", default)]
  pub amount_info: Option<AmountInfo>,
  #[serde(deserialize_with = "deserialize_option_empty_object", default)]
  pub fee_info: Option<FeeInfo>,
  pub signed_messages: Option<Vec<SignedMessage>>,
  pub external_tx_id: Option<String>,

  pub destinations: Option<Vec<TransactionDestination>>,
  #[serde(deserialize_with = "deserialize_option_empty_object", default)]
  pub block_info: Option<BlockInfo>,
  pub authorization_info: Option<AuthorizationInfo>,
  pub index: Option<u64>,
  pub reward_info: Option<RewardInfo>,
  pub fee_payer_info: Option<FeePayerInfo>,
  pub extra_parameters: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionArguments {
  #[serde(rename = "assetId")]
  pub asset_id: String,
  pub operation: TransactionOperation,
  pub source: TransferPeerPath,
  pub destination: DestinationTransferPeerPath,
  pub amount: String,

  #[serde(rename = "externalTxId", skip_serializing_if = "Option::is_none")]
  pub external_tx_id: Option<String>,
  #[serde(rename = "customerRefId", skip_serializing_if = "Option::is_none")]
  pub customer_ref_id: Option<String>,

  // pub extra_parameters: Option<ExtraParameters>,
  // pub extra_parameters: Option<String>,
  #[serde(rename = "treatAsGrossAmount", skip_serializing_if = "Option::is_none")]
  pub treat_as_gross_amount: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub gas_price: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub gas_limit: Option<String>,
  #[serde(rename = "feeLevel", skip_serializing_if = "Option::is_none")]
  pub fee_level: Option<String>,
  pub note: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ExtraParameters {
  ContractCallData(String),
  RawMessageData(TypedMessages),
  // TypeMessageData(TypedMessageData),
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypedMessages {
  pub messages: Vec<UnsignedMessage>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawMessageData {
  pub messages: Vec<UnsignedMessage>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnsignedMessage {
  pub content: String,
  #[serde(rename = "type")]
  pub message_type: String,
  pub index: i32,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreateTransactionResponse {
  pub id: String,
  pub status: TransactionStatus,
}

#[cfg(test)]
mod test {
  use crate::types::TransactionListBuilder;
  use chrono::Utc;

  #[test]
  fn transaction_builder() -> color_eyre::Result<()> {
    let request = TransactionListBuilder::new().sort_desc().order_last_updated().before(&Utc::now()).build()?;
    let found = request.into_iter().find(|v| v.0 == "before");
    assert!(found.is_some());
    Ok(())
  }
}
