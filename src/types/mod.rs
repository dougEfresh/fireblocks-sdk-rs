use std::str::FromStr;

use chrono::{DateTime, LocalResult, TimeZone, Utc};
use serde::{de::Error as SerdeError, Deserialize, Deserializer};
use serde_derive::Serialize;
use serde_json::Value;

pub mod address;
pub mod asset;
pub mod fee;
pub mod staking;
pub mod transaction;
pub mod vault;
pub mod wallet;

pub use address::{Address, AddressContainer, AddressType, CreateAddressResponse};
pub use asset::{AccountAsset, AssetResponse, SupportedAsset};
pub use fee::*;
pub use staking::*;
pub use transaction::*;
pub use vault::*;
pub use wallet::*;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct DeleteResponse {
  pub message: String,
  pub code: i32,
}

fn deserialize_option_empty_object<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
  T: Deserialize<'de>,
  D: Deserializer<'de>,
{
  let v: Result<Value, D::Error> = Deserialize::deserialize(deserializer);

  match v {
    Ok(Value::Object(ref map)) if map.is_empty() => Ok(None),
    Ok(Value::String(ref s)) if s.is_empty() => Ok(None),
    Ok(val) => T::deserialize(val).map(Some).map_err(SerdeError::custom),
    Err(_) => Ok(None), // Assume field is missing and return None
  }
}

fn deserialize_epoch_time<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
  D: Deserializer<'de>,
{
  let millis = i64::deserialize(deserializer)?;
  match Utc.timestamp_millis_opt(millis) {
    LocalResult::Single(dt) => Ok(dt),
    _ => Err(SerdeError::custom(format!("invalid timestamp {millis}"))),
  }
}
fn deserialize_str_i32<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
  D: Deserializer<'de>,
{
  let s = String::deserialize(deserializer)?;
  i32::from_str(&s).map_err(SerdeError::custom)
}

fn deserialize_str_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
  D: Deserializer<'de>,
{
  let s = String::deserialize(deserializer)?;
  u64::from_str(&s).map_err(SerdeError::custom)
}
#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct Paging {
  pub before: Option<String>,
  pub after: Option<String>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct PagingVaultRequest {
  pub limit: i32,
  pub before: Option<String>,
  pub after: Option<String>,
  pub name_prefix: Option<String>,
  pub name_suffix: Option<String>,
}

impl Default for PagingVaultRequest {
  fn default() -> Self {
    Self { limit: 500, before: None, after: None, name_prefix: None, name_suffix: None }
  }
}

impl PagingVaultRequest {
  pub fn params(&self) -> Vec<(String, String)> {
    let mut params: Vec<(String, String)> = Vec::new();
    params.push(("limit".to_owned(), self.limit.to_string()));
    self.name_prefix.clone().inspect(|v| params.push(("namePrefix".to_owned(), String::from(v))));
    self.name_suffix.clone().inspect(|v| params.push(("nameSuffix".to_owned(), String::from(v))));
    params
  }
}

impl Paging {
  fn epoch(before: &DateTime<Utc>) -> String {
    format!("{}", before.timestamp_millis())
  }

  pub fn set_before(&mut self, before: &DateTime<Utc>) {
    self.before = Some(Self::epoch(before));
  }

  pub fn set_after(&mut self, after: &DateTime<Utc>) {
    self.after = Some(Self::epoch(after));
  }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct PaginatedAssetWallet {
  pub asset_wallets: Vec<AssetResponse>,
  pub paging: Paging,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Deserialize)]
#[allow(clippy::upper_case_acronyms)]
pub enum PeerType {
  VAULT_ACCOUNT,
  EXCHANGE_ACCOUNT,
  INTERNAL_WALLET,
  EXTERNAL_WALLET,
  ONE_TIME_ADDRESS,
  NETWORK_CONNECTION,
  FIAT_ACCOUNT,
  COMPOUND,
}
