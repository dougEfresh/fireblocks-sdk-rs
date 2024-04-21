use crate::{impl_base_query_params, Epoch};
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
#[allow(dead_code)]
pub enum PeerType {
  VAULT_ACCOUNT,
  EXCHANGE_ACCOUNT,
  INTERNAL_WALLET,
  EXTERNAL_WALLET,
  FIAT_ACCOUNT,
  NETWORK_CONNECTION,
  COMPOUND,
  CONTRACT,
  #[default]
  UNKNOWN,
  GAS_STATION,
  END_USER_WALLET,
  ONE_TIME_ADDRESS,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
#[allow(clippy::upper_case_acronyms)]
#[allow(dead_code)]
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
#[allow(dead_code)]
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

pub struct TransactionListOptions {
  params: Vec<(String, String)>,
}

#[derive(Debug, Default)]
pub struct TransactionListBuilder {
  params: Vec<(String, String)>,
  base: BasePageParams,
}

impl_base_query_params!(TransactionListBuilder);

impl TransactionListBuilder {
  pub fn source_id(&mut self, id: i32) -> &mut Self {
    self.params.push(("sourceId".to_string(), id.to_string()));
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

  pub fn tx_hash(&mut self, tx: &str) -> &mut Self {
    self.params.push(("txHash".to_owned(), String::from(tx)));
    self
  }
}

impl TransactionListOptions {
  pub const fn new(params: Vec<(String, String)>) -> Self {
    Self { params }
  }

  pub fn params(&self) -> impl Iterator<Item = &(String, String)> {
    self.params.iter()
  }
}

#[allow(clippy::upper_case_acronyms)]
#[allow(non_camel_case_types)]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
pub enum VirtualType {
  OFF_EXCHANGE,
  #[default]
  DEFAULT,
  OEC_FEE_BANK,
}

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct AmountInfo {
  amount: Option<BigDecimal>,
  requested_amount: Option<BigDecimal>,
  net_amount: Option<BigDecimal>,
  #[serde(rename = "amountUSD")]
  amount_usd: Option<BigDecimal>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct BlockInfo {
  pub block_hash: Option<String>,
  //#[serde(deserialize_with = "deserialize_str_u64_opt")]
  // TODO convert to u64
  pub block_height: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct FeeInfo {
  network_fee: Option<BigDecimal>,
  service_fee: Option<BigDecimal>,
  gas_price: Option<BigDecimal>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct RewardInfo {
  src_rewards: Option<String>,
  dest_rewards: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct FeePayerInfo {
  fee_payer_account_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub enum Logic {
  OR,
  AND,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct AuthorizationInfo {
  allow_operator_as_authorizer: bool,
  logic: Logic,
}
#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Default)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum SigningAlgorithm {
  #[default]
  MPC_ECDSA_SECP256K1,
  MPC_ECDSA_SECP256R1,
  MPC_EDDSA_ED25519,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct Signature {
  pub full_sig: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct SignedMessage {
  pub derivation_path: Vec<u64>,
  pub algorithm: SigningAlgorithm,
  pub public_key: String,
  pub signature: Signature,
}

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct OneTimeAddress {
  pub address: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub tag: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
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

impl Default for DestinationTransferPeerPath {
  fn default() -> Self {
    Self {
      peer_type: PeerType::VAULT_ACCOUNT,
      id: String::new(),
      wallet_id: None,
      virtual_id: None,
      virtual_type: None,
      one_time_address: None,
    }
  }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct TransactionDestination {
  amount: BigDecimal,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
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
  #[serde(skip_serializing_if = "Option::is_none")]
  pub destination: Option<DestinationTransferPeerPath>,
  pub amount: String,
  // pub extra_parameters: Option<ExtraParameters>,
  // pub extra_parameters: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub gas_price: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub gas_limit: Option<String>,
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
#[allow(dead_code)]
pub struct CreateTransactionResponse {
  pub id: String,
  pub status: TransactionStatus,
}
