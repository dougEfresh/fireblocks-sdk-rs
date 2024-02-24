use serde::{de::Error, Deserialize, Deserializer};
use serde_derive::Serialize;

use crate::types::{deserialize_option_empty_object, Paging};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[allow(dead_code)]
pub enum AddressType {
  #[default]
  Permanent,
  Change,
  Deposit,
  Address, // ????
}

fn deserialize_addr_type<'de, D>(deserializer: D) -> Result<AddressType, D::Error>
where
  D: Deserializer<'de>,
{
  let t = String::deserialize(deserializer)?.to_lowercase();
  match t.as_str() {
    "permanent" => Ok(AddressType::Permanent),
    "change" => Ok(AddressType::Change),
    "deposit" => Ok(AddressType::Deposit),
    "address" => Ok(AddressType::Address),
    _ => Err(Error::custom(format!("unexpected address type: {t}"))),
  }
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct AddressContainer {
  pub addresses: Vec<Address>,
  pub paging: Option<Paging>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct CreateAddressResponse {
  pub id: String,
  pub address: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct Address {
  pub asset_id: String,
  pub address: String,
  #[serde(rename = "type", deserialize_with = "deserialize_addr_type")]
  pub address_type: AddressType,
  #[serde(deserialize_with = "deserialize_option_empty_object", default)]
  pub tag: Option<String>,
  #[serde(deserialize_with = "deserialize_option_empty_object", default)]
  pub description: Option<String>,
  #[serde(rename = "bip44AddressIndex")]
  pub bip44address_index: i32,
  pub user_defined: bool,
}
