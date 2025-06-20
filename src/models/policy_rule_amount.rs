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

/// PolicyRuleAmount : Defines the value a transaction must exceed for the rule
/// to apply to it (according to the amountCurrency field)
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicyRuleAmount {}

impl PolicyRuleAmount {
    /// Defines the value a transaction must exceed for the rule to apply to it
    /// (according to the amountCurrency field)
    pub fn new() -> PolicyRuleAmount {
        PolicyRuleAmount {}
    }
}
