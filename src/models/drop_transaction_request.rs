// Fireblocks API
//
// Fireblocks provides a suite of applications to manage digital asset operations and a complete development platform to build your business on the blockchain.  - Visit our website for more information: [Fireblocks Website](https://fireblocks.com) - Visit our developer docs: [Fireblocks DevPortal](https://developers.fireblocks.com)
//
// The version of the OpenAPI document: 1.8.0
// Contact: developers@fireblocks.com
// Generated by: https://openapi-generator.tech

use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DropTransactionRequest {
    /// Fireblocks Transaction ID
    #[serde(rename = "txId", skip_serializing_if = "Option::is_none")]
    pub tx_id: Option<String>,
    /// Transcation Fee Level
    #[serde(rename = "feeLevel", skip_serializing_if = "Option::is_none")]
    pub fee_level: Option<FeeLevel>,
    /// Transaction Gas Price in gwei
    #[serde(rename = "gasPrice", skip_serializing_if = "Option::is_none")]
    pub gas_price: Option<String>,
}

impl DropTransactionRequest {
    pub fn new() -> DropTransactionRequest {
        DropTransactionRequest {
            tx_id: None,
            fee_level: None,
            gas_price: None,
        }
    }
}
/// Transcation Fee Level
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FeeLevel {
    #[serde(rename = "LOW")]
    Low,
    #[serde(rename = "MEDIUM")]
    Medium,
    #[serde(rename = "HIGH")]
    High,
}

impl Default for FeeLevel {
    fn default() -> FeeLevel {
        Self::Low
    }
}