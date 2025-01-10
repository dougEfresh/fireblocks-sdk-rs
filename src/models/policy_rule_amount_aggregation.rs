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

/// PolicyRuleAmountAggregation : Defines the method by which the Policy Engine
/// calculates accumulation. It uses the Initiator, Source, and Destination to
/// calculate accumulation toward the value under Minimum, for the time under
/// Time Period.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicyRuleAmountAggregation {
    #[serde(rename = "operators", skip_serializing_if = "Option::is_none")]
    pub operators: Option<models::AmountAggregationTimePeriodMethod>,
    #[serde(rename = "srcTransferPeers", skip_serializing_if = "Option::is_none")]
    pub src_transfer_peers: Option<models::AmountAggregationTimePeriodMethod>,
    #[serde(rename = "dstTransferPeers", skip_serializing_if = "Option::is_none")]
    pub dst_transfer_peers: Option<models::AmountAggregationTimePeriodMethod>,
}

impl PolicyRuleAmountAggregation {
    /// Defines the method by which the Policy Engine calculates accumulation.
    /// It uses the Initiator, Source, and Destination to calculate accumulation
    /// toward the value under Minimum, for the time under Time Period.
    pub fn new() -> PolicyRuleAmountAggregation {
        PolicyRuleAmountAggregation {
            operators: None,
            src_transfer_peers: None,
            dst_transfer_peers: None,
        }
    }
}