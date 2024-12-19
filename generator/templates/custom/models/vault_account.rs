use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VaultAccount {
    pub id: String,
    pub name: String,
    pub assets: Vec<models::VaultAsset>,
    #[serde(rename = "hiddenOnUI")]
    pub hidden_on_ui: bool,
    pub assets: Vec<AccountAsset>,
    pub customer_ref_id: Option<String>,
    pub auto_fuel: bool,
}
