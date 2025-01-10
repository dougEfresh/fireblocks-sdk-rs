// Fireblocks API
//
// Fireblocks provides a suite of applications to manage digital asset operations and a complete development platform to build your business on the blockchain.  - Visit our website for more information: [Fireblocks Website](https://fireblocks.com) - Visit our developer docs: [Fireblocks DevPortal](https://developers.fireblocks.com)
//
// The version of the OpenAPI document: 1.8.0
// Contact: developers@fireblocks.com
// Generated by: https://openapi-generator.tech

use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct VaultWalletAddress {
    #[serde(rename = "assetId")]
    pub asset_id: String,
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "customerRefId", skip_serializing_if = "Option::is_none")]
    pub customer_ref_id: Option<String>,
    #[serde(rename = "addressFormat", skip_serializing_if = "Option::is_none")]
    pub address_format: Option<AddressFormat>,
    #[serde(rename = "legacyAddress", skip_serializing_if = "Option::is_none")]
    pub legacy_address: Option<String>,
    #[serde(rename = "enterpriseAddress", skip_serializing_if = "Option::is_none")]
    pub enterprise_address: Option<String>,
    #[serde(rename = "bip44AddressIndex", skip_serializing_if = "Option::is_none")]
    pub bip44_address_index: Option<i32>,
    #[serde(rename = "userDefined", skip_serializing_if = "Option::is_none")]
    pub user_defined: Option<bool>,
}

impl VaultWalletAddress {
    pub fn new() -> VaultWalletAddress {
        VaultWalletAddress {
            asset_id: String::new(),
            address: String::new(),
            description: None,
            tag: None,
            r#type: None,
            customer_ref_id: None,
            address_format: None,
            legacy_address: None,
            enterprise_address: None,
            bip44_address_index: None,
            user_defined: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AddressFormat {
    #[serde(rename = "SEGWIT")]
    Segwit,
    #[serde(rename = "LEGACY")]
    Legacy,
    #[serde(rename = "BASE")]
    Base,
    #[serde(rename = "PAYMENT")]
    Payment,
}

impl Default for AddressFormat {
    fn default() -> AddressFormat {
        Self::Segwit
    }
}
