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

/// AdditionalInfo : Additional information related to the blockchain. This may
/// include extra details about the blockchain network.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdditionalInfo {
    /// The estimated annual reward rate for the blockchain, represented as a
    /// decimal percentage value.
    #[serde(rename = "estimatedAnnualReward")]
    pub estimated_annual_reward: f64,
    /// The duration of the lockup period for certain actions on the blockchain,
    /// measured in milliseconds.
    #[serde(rename = "lockupPeriod")]
    pub lockup_period: f64,
    /// The duration of the activation period for certain actions on the
    /// blockchain, measured in milliseconds.
    #[serde(rename = "activationPeriod")]
    pub activation_period: f64,
}

impl AdditionalInfo {
    /// Additional information related to the blockchain. This may include extra
    /// details about the blockchain network.
    pub fn new(
        estimated_annual_reward: f64,
        lockup_period: f64,
        activation_period: f64,
    ) -> AdditionalInfo {
        AdditionalInfo {
            estimated_annual_reward,
            lockup_period,
            activation_period,
        }
    }
}
