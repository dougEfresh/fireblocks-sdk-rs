// Fireblocks API
//
// Fireblocks provides a suite of applications to manage digital asset operations and a complete development platform to build your business on the blockchain.  - Visit our website for more information: [Fireblocks Website](https://fireblocks.com) - Visit our developer docs: [Fireblocks DevPortal](https://developers.fireblocks.com)
//
// The version of the OpenAPI document: 1.8.0
// Contact: developers@fireblocks.com
// Generated by: https://openapi-generator.tech

use {
    crate::models,
    serde::{Deserialize, Serialize},
};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ToExchangeTransaction {
    #[serde(rename = "assetId", skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<String>,
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(rename = "dstAddress", skip_serializing_if = "Option::is_none")]
    pub dst_address: Option<String>,
    /// optional
    #[serde(rename = "dstTag", skip_serializing_if = "Option::is_none")]
    pub dst_tag: Option<String>,
}

impl ToExchangeTransaction {
    pub fn new() -> ToExchangeTransaction {
        ToExchangeTransaction {
            asset_id: None,
            amount: None,
            dst_address: None,
            dst_tag: None,
        }
    }
}
