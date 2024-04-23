use std::str::FromStr;

use chrono::{DateTime, LocalResult, TimeZone, Utc};
use serde::{de::Error as SerdeError, Deserialize, Deserializer};
use serde_json::Value;

pub mod address;
pub mod asset;
pub mod connect;
pub mod fee;
mod page;
pub mod staking;
pub mod transaction;
pub mod vault;
pub mod wallet;

pub use address::*;
pub use asset::*;
pub use fee::*;
pub use page::*;
pub use staking::*;
pub use transaction::*;
pub use vault::*;
pub use wallet::*;

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

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct PaginatedAssetWallet {
  pub asset_wallets: Vec<AssetResponse>,
  pub paging: Paging,
}
