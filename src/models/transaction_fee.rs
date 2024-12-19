/*
 * Fireblocks API
 *
 * Fireblocks provides a suite of applications to manage digital asset operations and a complete development platform to build your business on the blockchain.  - Visit our website for more information: [Fireblocks Website](https://fireblocks.com) - Visit our developer docs: [Fireblocks DevPortal](https://developers.fireblocks.com) 
 *
 * The version of the OpenAPI document: 1.8.0
 * Contact: developers@fireblocks.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionFee {
    /// Gas limit value for EVM based networks
    #[serde(rename = "gasLimit", skip_serializing_if = "Option::is_none")]
    pub gas_limit: Option<String>,
    /// The fee per byte value for UTXO based assets
    #[serde(rename = "feePerByte", skip_serializing_if = "Option::is_none")]
    pub fee_per_byte: Option<String>,
    /// Gas price in gwei units for EVM based networks
    #[serde(rename = "gasPrice", skip_serializing_if = "Option::is_none")]
    pub gas_price: Option<String>,
    /// The full network fee price
    #[serde(rename = "networkFee", skip_serializing_if = "Option::is_none")]
    pub network_fee: Option<String>,
    /// (optional) Base Fee according to EIP-1559 (ETH assets)
    #[serde(rename = "baseFee", skip_serializing_if = "Option::is_none")]
    pub base_fee: Option<String>,
    /// (optional) Priority Fee according to EIP-1559 (ETH assets)
    #[serde(rename = "priorityFee", skip_serializing_if = "Option::is_none")]
    pub priority_fee: Option<String>,
    /// Max Fee Per Gas Delta added only for EIP-1559 (ETH assets)
    #[serde(rename = "maxFeePerGasDelta", skip_serializing_if = "Option::is_none")]
    pub max_fee_per_gas_delta: Option<String>,
    /// Layer 1 fee for Layer 2 chains
    #[serde(rename = "l1Fee", skip_serializing_if = "Option::is_none")]
    pub l1_fee: Option<String>,
}

impl TransactionFee {
    pub fn new() -> TransactionFee {
        TransactionFee {
            gas_limit: None,
            fee_per_byte: None,
            gas_price: None,
            network_fee: None,
            base_fee: None,
            priority_fee: None,
            max_fee_per_gas_delta: None,
            l1_fee: None,
        }
    }
}

