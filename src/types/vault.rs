use serde_derive::{Deserialize, Serialize};

use crate::types::{asset::AccountAsset, deserialize_str_i32, Paging};

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct VaultAccounts {
  pub accounts: Vec<Account>,
  pub paging: Paging,
  pub previous_url: Option<String>,
  pub next_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Account {
  #[serde(deserialize_with = "deserialize_str_i32")]
  pub id: i32,
  pub name: String,
  #[serde(rename = "hiddenOnUI")]
  pub hidden_on_ui: bool,
  pub assets: Vec<AccountAsset>,
  pub customer_ref_id: Option<String>,
  pub auto_fuel: bool,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateAccount {
  pub name: String,
  #[serde(rename = "hiddenOnUI")]
  pub hidden_on_ui: bool,
  pub customer_ref_id: Option<String>,
  pub auto_fuel: bool,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VaultRenameResponse {
  pub name: String,
  #[serde(deserialize_with = "deserialize_str_i32")]
  pub id: i32,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateAddress {
  pub description: Option<String>,
  pub customer_ref_id: Option<String>,
}
