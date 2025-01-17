// Fireblocks API
//
// Fireblocks provides a suite of applications to manage digital asset operations and a complete development platform to build your business on the blockchain.  - Visit our website for more information: [Fireblocks Website](https://fireblocks.com) - Visit our developer docs: [Fireblocks DevPortal](https://developers.fireblocks.com)
//
// The version of the OpenAPI document: 1.8.0
// Contact: developers@fireblocks.com
// Generated by: https://openapi-generator.tech

use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssetResponseOnchain {
    /// The asset's symbol
    #[serde(rename = "symbol")]
    pub symbol: String,
    /// The asset's name
    #[serde(rename = "name")]
    pub name: String,
    /// The asset's address
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Number of decimals
    #[serde(rename = "decimals")]
    pub decimals: f64,
    /// The asset's standard
    #[serde(rename = "standard", skip_serializing_if = "Option::is_none")]
    pub standard: Option<String>,
}

impl AssetResponseOnchain {
    pub fn new(symbol: String, name: String, decimals: f64) -> AssetResponseOnchain {
        AssetResponseOnchain {
            symbol,
            name,
            address: None,
            decimals,
            standard: None,
        }
    }
}
