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
pub struct TravelRuleTransactionBlockchainInfo {
    #[serde(rename = "txHash", skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,
    #[serde(rename = "origin", skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(rename = "destination", skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
}

impl TravelRuleTransactionBlockchainInfo {
    pub fn new() -> TravelRuleTransactionBlockchainInfo {
        TravelRuleTransactionBlockchainInfo {
            tx_hash: None,
            origin: None,
            destination: None,
        }
    }
}
