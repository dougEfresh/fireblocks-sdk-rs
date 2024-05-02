use crate::types::deserialize_str_u64;
use crate::Asset;
use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StakingPosition {
  /// The unique identifier of the staking position
  pub id: String,

  /// The unique identifier of the staking provider
  pub provider_id: String,

  /// The source vault account to stake from.
  #[serde(deserialize_with = "deserialize_str_u64")]
  pub vault_account_id: u64,

  /// The destination validator address name.
  pub validator_name: String,
  /// The destination validator provider name.
  pub provider_name: String,

  /// The blockchain descriptor to use.
  pub chain_descriptor: String,

  /// Amount of tokens to stake.
  pub amount: BigDecimal,

  /// The amount staked in the position, measured in the blockchain descriptor unit.
  pub rewards_amount: BigDecimal,

  /// When was the request made (ISO Date).
  pub date_created: DateTime<Utc>,

  /// The current status.
  // status: PositionStatus;

  /// An array of transaction objects related to this position.
  /// Each object includes a 'txId' representing the transaction ID
  /// and a 'isSuccessful' boolean indicating if the transaction was successful.
  // relatedTransactions: RelatedTransactionDto[];

  /// Indicates whether there is an ongoing action for this position (true if ongoing, false if not).
  pub in_progress: bool,

  /// The transaction ID of the ongoing request
  pub in_progress_tx_id: Option<String>,

  /// Additional fields per blockchain - can be empty or missing if not initialized or no additional info exists.
  /// The type depends on the chainDescriptor value.
  /// For Solana (SOL), stake account address.
  /// For Ethereum (ETH), an empty object is returned as no specific data is available.
  // blockchain_position_info: TBlockchainPositionInfo;

  /// The destination address of the staking transaction.
  pub validator_address: String,

  /// An array of available actions that can be performed. for example, actions like "unstake" or "withdraw".
  pub available_actions: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StakingAmountSummary {
  #[serde(rename = "chainDescriptor")]
  pub chain: String,
  pub amount: BigDecimal,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StakingPositionsSummary {
  pub active: Vec<StakingAmountSummary>,
  pub inactive: Vec<StakingAmountSummary>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StakingAmounts {
  pub active: Vec<StakingAmountSummary>,
  pub inactive: Vec<StakingAmountSummary>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StakingAdditionalInfo {
  estimated_annual_reward: BigDecimal,
  lockup_period: u64,
  activation_period: u64,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StakingChainInfo {
  chain_descriptor: Asset,
  current_epoch: u64,
  epoch_elapsed: f64,
  epoch_duration: u64,
  additional_info: StakingAdditionalInfo,
}
