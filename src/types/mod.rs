use std::fmt;
use std::str::FromStr;

use chrono::{DateTime, LocalResult, TimeZone, Utc};
use serde::{de::Error as SerdeError, Deserialize, Deserializer};
use serde_json::Value;

pub mod address;
pub mod asset;
pub mod connect;
pub mod fee;
pub mod hooks;
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
  struct Visitor;

  impl<'de> serde::de::Visitor<'de> for Visitor {
    type Value = i32;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
      formatter.write_str("an integer or a string representing an integer")
    }

    fn visit_i64<E>(self, value: i64) -> Result<i32, E>
    where
      E: serde::de::Error,
    {
      i32::try_from(value).map_err(E::custom)
    }

    fn visit_u64<E>(self, value: u64) -> Result<i32, E>
    where
      E: serde::de::Error,
    {
      i32::try_from(value).map_err(E::custom)
    }

    fn visit_str<E>(self, value: &str) -> Result<i32, E>
    where
      E: serde::de::Error,
    {
      value.parse::<i32>().map_err(E::custom)
    }
  }

  deserializer.deserialize_any(Visitor)
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
pub struct PaginatedAssetWallet {
  pub asset_wallets: Vec<AssetResponse>,
  pub paging: Paging,
}

#[cfg(test)]
mod test {
  use crate::types::Account;
  use serde_json::json;

  #[test]
  fn test_i32() {
    let j = json!(
        {
        "id": 483,
        "name": "z-test-1719791845664-rename",
        "hiddenOnUI": true,
        "assets": [
        {
        "id": "SOL_TEST",
        "total": "0",
        "available": "0",
        "pending": "0",
        "frozen": "0",
        "lockedAmount": "0",
        "staked": "0",
        "totalStakedCPU": null,
        "totalStakedNetwork": null,
        "selfStakedCPU": null,
        "selfStakedNetwork": null,
        "pendingRefundCPU": null,
        "pendingRefundNetwork": null,
        "blockHeight": "30",
        "blockHash": ""
        }
        ],
        "customerRefId": "",
        "autoFuel": false
      }
    );

    match serde_json::from_value::<Account>(j) {
      Err(e) => {
        assert_eq!("error! ", e.to_string());
      },
      Ok(a) => {
        assert_eq!(a.id, 483);
      },
    };
  }
}
