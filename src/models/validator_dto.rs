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
pub struct ValidatorDto {
    /// The protocol identifier (e.g. \"ETH\"/\"SOL\") of the validator
    #[serde(rename = "chainDescriptor")]
    pub chain_descriptor: String,
    /// The service fee as a percentage out of the earned rewards
    #[serde(rename = "feePercent")]
    pub fee_percent: f64,
}

impl ValidatorDto {
    pub fn new(chain_descriptor: String, fee_percent: f64) -> ValidatorDto {
        ValidatorDto {
            chain_descriptor,
            fee_percent,
        }
    }
}
