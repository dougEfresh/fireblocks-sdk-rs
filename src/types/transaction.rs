use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_derive::Serialize;

use crate::types::{deserialize_epoch_time, deserialize_option_empty_object};

#[allow(clippy::upper_case_acronyms)]
#[allow(non_camel_case_types)]
#[derive(Debug, Deserialize, Serialize, Clone, Default, sqlx::Type)]
#[allow(dead_code)]
#[sqlx(type_name = "peer_type", rename_all = "lowercase")]
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
#[derive(Debug, Deserialize, Serialize, Clone, Default, sqlx::Type)]
#[allow(clippy::upper_case_acronyms)]
#[allow(dead_code)]
#[sqlx(type_name = "transaction_operation", rename_all = "lowercase")]
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
#[derive(Debug, Deserialize, Default, Clone, sqlx::Type, PartialEq)]
#[allow(dead_code)]
#[sqlx(type_name = "transaction_status", rename_all = "lowercase")]
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

#[derive(Debug, Default)]
pub struct TransactionListOptions {
  params: Vec<(String, String)>,
}

impl TransactionListOptions {
  pub fn new() -> Self {
    Self { params: Vec::new() }
  }

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

  pub fn before(&mut self, t: Option<&DateTime<Utc>>) -> &mut Self {
    self.add_instant("before", t)
  }

  pub fn after(&mut self, t: Option<&DateTime<Utc>>) -> &mut Self {
    self.add_instant("after", t)
  }

  fn add_instant(&mut self, param: &str, t: Option<&DateTime<Utc>>) -> &mut Self {
    if let Some(tm) = t {
      self.params.push((param.to_owned(), self.epoch(tm)));
    }
    self
  }

  pub fn assets(&mut self, a: &[String]) -> &mut Self {
    self.params.push(("assets".to_owned(), a.join(",")));
    self
  }

  pub fn tx_hash(&mut self, tx: &str) -> &mut Self {
    self.params.push(("txHash".to_owned(), String::from(tx)));
    self
  }

  pub fn limit(&mut self, limit: u16) -> &mut Self {
    self.params.push(("limit".to_owned(), format!("{}", limit)));
    self
  }

  fn epoch(&self, before: &DateTime<Utc>) -> String {
    format!("{}", before.timestamp_millis())
  }

  pub fn params(&self) -> impl Iterator<Item = &(String, String)> {
    self.params.iter()
  }
}
#[allow(clippy::upper_case_acronyms)]
#[allow(non_camel_case_types)]
#[derive(Debug, Deserialize, Serialize, Clone, Default, sqlx::Type)]
#[sqlx(type_name = "virtual_type", rename_all = "lowercase")]
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
#[derive(Debug, Deserialize, PartialEq, Clone, sqlx::Type, Default)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[sqlx(type_name = "signing_algorithm", rename_all = "lowercase")]
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

impl SignedMessage {
  pub fn derivation_path(&self) -> &Vec<u64> {
    &self.derivation_path
  }

  pub fn algorithm(&self) -> &SigningAlgorithm {
    &self.algorithm
  }

  pub fn public_key(&self) -> &str {
    &self.public_key
  }

  pub fn signature(&self) -> &Signature {
    &self.signature
  }
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
    DestinationTransferPeerPath {
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
  id: String,
  asset_id: String,
  status: TransactionStatus,
  destination: Option<TransferPeerPath>,
  source: Option<TransferPeerPath>,
  amount: Option<BigDecimal>,
  network_fee: Option<BigDecimal>,
  #[serde(rename = "amountUSD")]
  amount_usd: Option<BigDecimal>,
  net_amount: Option<BigDecimal>,
  #[serde(deserialize_with = "deserialize_epoch_time")]
  created_at: DateTime<Utc>,
  #[serde(deserialize_with = "deserialize_epoch_time")]
  last_updated: DateTime<Utc>,
  #[serde(deserialize_with = "deserialize_option_empty_object", default)]
  tx_hash: Option<String>,
  num_of_confirmations: Option<i64>,
  #[serde(deserialize_with = "deserialize_option_empty_object", default)]
  sub_status: Option<String>,
  signed_by: Vec<String>,
  #[serde(deserialize_with = "deserialize_option_empty_object", default)]
  created_by: Option<String>,
  rejected_by: Option<String>,
  #[serde(deserialize_with = "deserialize_option_empty_object", default)]
  destination_address: Option<String>,
  #[serde(deserialize_with = "deserialize_option_empty_object", default)]
  source_address: Option<String>,
  #[serde(deserialize_with = "deserialize_option_empty_object", default)]
  destination_address_description: Option<String>,
  #[serde(deserialize_with = "deserialize_option_empty_object", default)]
  destination_tag: Option<String>,
  #[serde(deserialize_with = "deserialize_option_empty_object", default)]
  address_type: Option<String>,
  note: String,
  #[serde(deserialize_with = "deserialize_option_empty_object", default)]
  exchange_tx_id: Option<String>,
  requested_amount: Option<BigDecimal>,
  service_fee: Option<BigDecimal>,
  fee_currency: String,

  // amlScreeningResult?: AmlScreeningResult;
  customer_ref_id: Option<String>,
  #[serde(deserialize_with = "deserialize_option_empty_object", default)]
  amount_info: Option<AmountInfo>,
  #[serde(deserialize_with = "deserialize_option_empty_object", default)]
  fee_info: Option<FeeInfo>,
  signed_messages: Option<Vec<SignedMessage>>,
  external_tx_id: Option<String>,

  destinations: Option<Vec<TransactionDestination>>,
  #[serde(deserialize_with = "deserialize_option_empty_object", default)]
  block_info: Option<BlockInfo>,
  authorization_info: Option<AuthorizationInfo>,
  index: Option<u64>,
  reward_info: Option<RewardInfo>,
  fee_payer_info: Option<FeePayerInfo>,
  extra_parameters: Option<serde_json::Value>,
}

impl Transaction {
  pub fn id(&self) -> &str {
    &self.id
  }

  pub fn asset_id(&self) -> &str {
    &self.asset_id
  }

  pub fn status(&self) -> &TransactionStatus {
    &self.status
  }

  pub fn destination(&self) -> Option<&TransferPeerPath> {
    self.destination.as_ref()
  }

  pub fn amount(&self) -> Option<&BigDecimal> {
    self.amount.as_ref()
  }

  pub fn network_fee(&self) -> Option<&BigDecimal> {
    self.network_fee.as_ref()
  }

  pub fn amount_usd(&self) -> Option<&BigDecimal> {
    self.amount_usd.as_ref()
  }

  pub fn net_amount(&self) -> Option<&BigDecimal> {
    self.net_amount.as_ref()
  }

  pub fn created_at(&self) -> &DateTime<Utc> {
    &self.created_at
  }

  pub fn last_updated(&self) -> &DateTime<Utc> {
    &self.last_updated
  }

  pub fn tx_hash(&self) -> Option<&str> {
    self.tx_hash.as_deref()
  }

  pub fn num_of_confirmations(&self) -> Option<i64> {
    self.num_of_confirmations
  }

  pub fn sub_status(&self) -> Option<&str> {
    self.sub_status.as_deref()
  }

  pub fn signed_by(&self) -> &Vec<String> {
    self.signed_by.as_ref()
  }

  pub fn created_by(&self) -> Option<&str> {
    self.created_by.as_deref()
  }

  pub fn rejected_by(&self) -> Option<&str> {
    self.rejected_by.as_deref()
  }

  pub fn destination_address(&self) -> Option<&str> {
    self.destination_address.as_deref()
  }

  pub fn source_address(&self) -> Option<&str> {
    self.source_address.as_deref()
  }

  pub fn destination_address_description(&self) -> Option<&str> {
    self.destination_address_description.as_deref()
  }

  pub fn destination_tag(&self) -> Option<&str> {
    self.destination_tag.as_deref()
  }

  pub fn address_type(&self) -> Option<&str> {
    self.address_type.as_deref()
  }

  pub fn note(&self) -> &str {
    &self.note
  }

  pub fn exchange_tx_id(&self) -> Option<&str> {
    self.exchange_tx_id.as_deref()
  }

  pub fn requested_amount(&self) -> &Option<BigDecimal> {
    &self.requested_amount
  }

  pub fn service_fee(&self) -> &Option<BigDecimal> {
    &self.service_fee
  }

  pub fn fee_currency(&self) -> &str {
    &self.fee_currency
  }

  pub fn customer_ref_id(&self) -> Option<&str> {
    self.customer_ref_id.as_deref()
  }

  pub fn amount_info(&self) -> Option<&AmountInfo> {
    self.amount_info.as_ref()
  }

  pub fn fee_info(&self) -> Option<&FeeInfo> {
    self.fee_info.as_ref()
  }

  pub fn signed_messages(&self) -> Option<&Vec<SignedMessage>> {
    self.signed_messages.as_ref()
  }

  pub fn external_tx_id(&self) -> Option<&str> {
    self.external_tx_id.as_deref()
  }

  pub fn destinations(&self) -> Option<&Vec<TransactionDestination>> {
    self.destinations.as_ref()
  }

  pub fn block_info(&self) -> Option<&BlockInfo> {
    self.block_info.as_ref()
  }

  pub fn authorization_info(&self) -> Option<&AuthorizationInfo> {
    self.authorization_info.as_ref()
  }

  pub fn index(&self) -> Option<u64> {
    self.index
  }

  pub fn reward_info(&self) -> Option<&RewardInfo> {
    self.reward_info.as_ref()
  }

  pub fn fee_payer_info(&self) -> Option<&FeePayerInfo> {
    self.fee_payer_info.as_ref()
  }

  pub fn extra_parameters(&self) -> Option<&serde_json::Value> {
    self.extra_parameters.as_ref()
  }

  pub fn source(&self) -> Option<&TransferPeerPath> {
    self.source.as_ref()
  }
}

impl Transaction {
  pub fn set_asset_id(&mut self, asset_id: String) {
    self.asset_id = asset_id;
  }

  pub fn set_status(&mut self, status: TransactionStatus) {
    self.status = status;
  }

  pub fn set_destination(&mut self, destination: Option<TransferPeerPath>) {
    self.destination = destination;
  }

  pub fn set_source(&mut self, source: Option<TransferPeerPath>) {
    self.source = source;
  }

  pub fn set_amount(&mut self, amount: Option<BigDecimal>) {
    self.amount = amount;
  }

  pub fn set_network_fee(&mut self, network_fee: Option<BigDecimal>) {
    self.network_fee = network_fee;
  }

  pub fn set_amount_usd(&mut self, amount_usd: Option<BigDecimal>) {
    self.amount_usd = amount_usd;
  }

  pub fn set_net_amount(&mut self, net_amount: Option<BigDecimal>) {
    self.net_amount = net_amount;
  }

  pub fn set_created_at(&mut self, created_at: DateTime<Utc>) {
    self.created_at = created_at;
  }

  pub fn set_last_updated(&mut self, last_updated: DateTime<Utc>) {
    self.last_updated = last_updated;
  }

  pub fn set_tx_hash(&mut self, tx_hash: Option<String>) {
    self.tx_hash = tx_hash;
  }

  pub fn set_num_of_confirmations(&mut self, num_of_confirmations: Option<i64>) {
    self.num_of_confirmations = num_of_confirmations;
  }

  pub fn set_sub_status(&mut self, sub_status: Option<String>) {
    self.sub_status = sub_status;
  }

  pub fn set_signed_by(&mut self, signed_by: Vec<String>) {
    self.signed_by = signed_by;
  }

  pub fn set_created_by(&mut self, created_by: Option<String>) {
    self.created_by = created_by;
  }

  pub fn set_rejected_by(&mut self, rejected_by: Option<String>) {
    self.rejected_by = rejected_by;
  }

  pub fn set_destination_address(&mut self, destination_address: Option<String>) {
    self.destination_address = destination_address;
  }

  pub fn set_source_address(&mut self, source_address: Option<String>) {
    self.source_address = source_address;
  }

  pub fn set_destination_address_description(&mut self, destination_address_description: Option<String>) {
    self.destination_address_description = destination_address_description;
  }

  pub fn set_destination_tag(&mut self, destination_tag: Option<String>) {
    self.destination_tag = destination_tag;
  }

  pub fn set_address_type(&mut self, address_type: Option<String>) {
    self.address_type = address_type;
  }

  pub fn set_note(&mut self, note: String) {
    self.note = note;
  }

  pub fn set_exchange_tx_id(&mut self, exchange_tx_id: Option<String>) {
    self.exchange_tx_id = exchange_tx_id;
  }

  pub fn set_requested_amount(&mut self, requested_amount: Option<BigDecimal>) {
    self.requested_amount = requested_amount;
  }

  pub fn set_service_fee(&mut self, service_fee: Option<BigDecimal>) {
    self.service_fee = service_fee;
  }

  pub fn set_fee_currency(&mut self, fee_currency: String) {
    self.fee_currency = fee_currency;
  }

  pub fn set_customer_ref_id(&mut self, customer_ref_id: Option<String>) {
    self.customer_ref_id = customer_ref_id;
  }

  pub fn set_amount_info(&mut self, amount_info: Option<AmountInfo>) {
    self.amount_info = amount_info;
  }

  pub fn set_fee_info(&mut self, fee_info: Option<FeeInfo>) {
    self.fee_info = fee_info;
  }

  pub fn set_signed_messages(&mut self, signed_messages: Option<Vec<SignedMessage>>) {
    self.signed_messages = signed_messages;
  }

  pub fn set_external_tx_id(&mut self, external_tx_id: Option<String>) {
    self.external_tx_id = external_tx_id;
  }

  pub fn set_destinations(&mut self, destinations: Option<Vec<TransactionDestination>>) {
    self.destinations = destinations;
  }

  pub fn set_block_info(&mut self, block_info: Option<BlockInfo>) {
    self.block_info = block_info;
  }

  pub fn set_authorization_info(&mut self, authorization_info: Option<AuthorizationInfo>) {
    self.authorization_info = authorization_info;
  }

  pub fn set_index(&mut self, index: Option<u64>) {
    self.index = index;
  }

  pub fn set_reward_info(&mut self, reward_info: Option<RewardInfo>) {
    self.reward_info = reward_info;
  }

  pub fn set_fee_payer_info(&mut self, fee_payer_info: Option<FeePayerInfo>) {
    self.fee_payer_info = fee_payer_info;
  }

  pub fn set_extra_parameters(&mut self, extra_parameters: Option<serde_json::Value>) {
    self.extra_parameters = extra_parameters;
  }

  pub fn set_id(&mut self, id: String) {
    self.id = id;
  }
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct CreateTransactionResponse {
  pub id: String,
  pub status: TransactionStatus,
}
