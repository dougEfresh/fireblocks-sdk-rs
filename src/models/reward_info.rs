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

/// RewardInfo : This field is relevant only for Algorand transactions. Both
/// `srcRewards` and `destRewards` will appear only for Vault to Vault
/// transactions, otherwise you will receive only the Fireblocks’ side of the
/// transaction.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RewardInfo {
    #[serde(rename = "srcRewards", skip_serializing_if = "Option::is_none")]
    pub src_rewards: Option<String>,
    #[serde(rename = "destRewards", skip_serializing_if = "Option::is_none")]
    pub dest_rewards: Option<String>,
}

impl RewardInfo {
    /// This field is relevant only for Algorand transactions. Both `srcRewards`
    /// and `destRewards` will appear only for Vault to Vault transactions,
    /// otherwise you will receive only the Fireblocks’ side of the transaction.
    pub fn new() -> RewardInfo {
        RewardInfo {
            src_rewards: None,
            dest_rewards: None,
        }
    }
}
