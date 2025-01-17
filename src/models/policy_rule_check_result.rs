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

/// PolicyRuleCheckResult : The rule validation result
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicyRuleCheckResult {
    /// Rule index number in the policy
    #[serde(rename = "index")]
    pub index: f64,
    /// Validation status
    #[serde(rename = "status")]
    pub status: Status,
    /// A set of rule validation error objects
    #[serde(rename = "errors")]
    pub errors: Vec<models::PolicyRuleError>,
}

impl PolicyRuleCheckResult {
    /// The rule validation result
    pub fn new(
        index: f64,
        status: Status,
        errors: Vec<models::PolicyRuleError>,
    ) -> PolicyRuleCheckResult {
        PolicyRuleCheckResult {
            index,
            status,
            errors,
        }
    }
}
/// Validation status
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "ok")]
    Ok,
    #[serde(rename = "failure")]
    Failure,
}

impl Default for Status {
    fn default() -> Status {
        Self::Ok
    }
}
