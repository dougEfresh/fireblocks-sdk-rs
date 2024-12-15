/*
 * Fireblocks API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.7.5
 * Contact: support@fireblocks.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionFee {
    #[serde(rename = "feePerByte", skip_serializing_if = "Option::is_none")]
    pub fee_per_byte: Option<String>,
    #[serde(rename = "gasPrice", skip_serializing_if = "Option::is_none")]
    pub gas_price: Option<String>,
    #[serde(rename = "gasLimit", skip_serializing_if = "Option::is_none")]
    pub gas_limit: Option<String>,
    #[serde(rename = "networkFee", skip_serializing_if = "Option::is_none")]
    pub network_fee: Option<String>,
    /// (optional) Base Fee according to EIP-1559 (ETH assets)
    #[serde(rename = "baseFee", skip_serializing_if = "Option::is_none")]
    pub base_fee: Option<String>,
    /// (optional) Priority Fee according to EIP-1559 (ETH assets)
    #[serde(rename = "priorityFee", skip_serializing_if = "Option::is_none")]
    pub priority_fee: Option<String>,
    /// (optional) Max Fee Per Gas Delta added only for EIP-1559 (ETH assets)
    #[serde(rename = "maxFeePerGasDelta", skip_serializing_if = "Option::is_none")]
    pub max_fee_per_gas_delta: Option<String>,
    /// (optional) Layer 1 fee for Layer 2 chains
    #[serde(rename = "l1Fee", skip_serializing_if = "Option::is_none")]
    pub l1_fee: Option<String>,
}

impl TransactionFee {
    pub fn new() -> TransactionFee {
        TransactionFee {
            fee_per_byte: None,
            gas_price: None,
            gas_limit: None,
            network_fee: None,
            base_fee: None,
            priority_fee: None,
            max_fee_per_gas_delta: None,
            l1_fee: None,
        }
    }
}

