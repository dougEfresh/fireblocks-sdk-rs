// Fireblocks API
//
// Fireblocks provides a suite of applications to manage digital asset operations and a complete development platform to build your business on the blockchain.  - Visit our website for more information: [Fireblocks Website](https://fireblocks.com) - Visit our developer docs: [Fireblocks DevPortal](https://developers.fireblocks.com)
//
// The version of the OpenAPI document: 1.8.0
// Contact: developers@fireblocks.com
// Generated by: https://openapi-generator.tech

use serde::{Deserialize, Serialize};

/// TransactionRequestGasPrice : For non-EIP-1559, EVM-based transactions. Price
/// per gas unit (in Ethereum this is specified in Gwei).  Note: Only two of the
/// three arguments can be specified in a single transaction: `gasLimit`,
/// `gasPrice` and `networkFee`. Fireblocks recommends using a numeric string
/// for accurate precision.  Although a number input exists, it is deprecated.
/// For non-EIP-1559, EVM-based transactions. Price per gas unit (in Ethereum
/// this is specified in Gwei).  Note: Only two of the three arguments can be
/// specified in a single transaction: `gasLimit`, `gasPrice` and `networkFee`.
/// Fireblocks recommends using a numeric string for accurate precision.
/// Although a number input exists, it is deprecated.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TransactionRequestGasPrice {
    /// Numeric string (recommended)
    String(String),
    /// Number (deprecated)
    Number(f64),
}

impl Default for TransactionRequestGasPrice {
    fn default() -> Self {
        Self::String(Default::default())
    }
}