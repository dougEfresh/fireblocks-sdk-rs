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
pub struct GetMaxSpendableAmountResponse {
    /// The maximum amount that can be spent from the vault account
    #[serde(rename = "maxSpendableAmount", skip_serializing_if = "Option::is_none")]
    pub max_spendable_amount: Option<String>,
}

impl GetMaxSpendableAmountResponse {
    pub fn new() -> GetMaxSpendableAmountResponse {
        GetMaxSpendableAmountResponse {
            max_spendable_amount: None,
        }
    }
}
