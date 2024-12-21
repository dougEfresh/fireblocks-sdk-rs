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
pub struct ConversionOperationFailure {
    #[serde(rename = "reason")]
    pub reason: Reason,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<std::collections::HashMap<String, serde_json::Value>>,
}

impl ConversionOperationFailure {
    pub fn new(reason: Reason) -> ConversionOperationFailure {
        ConversionOperationFailure { reason, data: None }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Reason {
    #[serde(rename = "INVALID_AMOUNT")]
    InvalidAmount,
    #[serde(rename = "SLIPPAGE_EXCEEDED")]
    SlippageExceeded,
    #[serde(rename = "AMOUNT_TOO_SMALL")]
    AmountTooSmall,
    #[serde(rename = "INSUFFICIENT_FUNDS")]
    InsufficientFunds,
}

impl Default for Reason {
    fn default() -> Reason {
        Self::InvalidAmount
    }
}
