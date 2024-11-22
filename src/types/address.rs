use crate::assets::Asset;
use serde::{de::Error, Deserialize, Deserializer};
use serde_derive::Serialize;

use crate::types::{deserialize_option_empty_object, Paging};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "sql", derive(sqlx::Type))]
#[cfg_attr(feature = "sql", sqlx(type_name = "address_type", rename_all = "lowercase"))]
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
pub struct AddressContainer {
  pub addresses: Vec<Address>,
  pub paging: Option<Paging>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreateAddressResponse {
  pub address: String,
  pub legacy_address: String,
  pub enterprise_address: Option<String>,
  #[serde(rename = "bip44AddressIndex")]
  pub bip44address_index: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Address {
  pub asset_id: Asset,
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

mod tests {

  #[test]
  fn deposit_types() {
    use super::Address;
    use super::AddressType;
    use serde_json::json;

    let v = json!({
      "assetId": "BTC",
      "address": "",
      "type": "change",
      "bip44AddressIndex": 0,
      "userDefined": true
    });
    let t: Address = serde_json::from_value(v).expect("failed to find address type");
    assert_eq!(t.address_type, AddressType::Change);

    let v = json!({
      "assetId": "BTC",
      "address": "",
      "type": "deposit",
      "bip44AddressIndex": 0,
      "userDefined": true
    });
    let t: Address = serde_json::from_value(v).expect("failed to find address type");
    assert_eq!(t.address_type, AddressType::Deposit);

    let v = json!({
      "assetId": "BTC",
      "address": "",
      "type": "address",
      "bip44AddressIndex": 0,
      "userDefined": true
    });
    let t: Address = serde_json::from_value(v).expect("failed to find address type");
    assert_eq!(t.address_type, AddressType::Address);

    let v = json!({
      "assetId": "BTC",
      "address": "",
      "type": "permanent",
      "bip44AddressIndex": 0,
      "userDefined": true
    });
    let t: Address = serde_json::from_value(v).expect("failed to find address type");
    assert_eq!(t.address_type, AddressType::Permanent);

    let v = json!({
      "assetId": "BTC",
      "address": "",
      "type": "nothing",
      "bip44AddressIndex": 0,
      "userDefined": true
    });
    let r = serde_json::from_value::<Address>(v);
    assert!(r.is_err());
  }
}
