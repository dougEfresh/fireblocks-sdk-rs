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

/// PolicyCheckResult : Policy rules validation result
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicyCheckResult {
    /// Number of errors
    #[serde(rename = "errors")]
    pub errors: f64,
    /// A set of validation results
    #[serde(rename = "result")]
    pub result: Vec<models::PolicyRuleCheckResult>,
}

impl PolicyCheckResult {
    /// Policy rules validation result
    pub fn new(errors: f64, result: Vec<models::PolicyRuleCheckResult>) -> PolicyCheckResult {
        PolicyCheckResult {
            errors,
            result,
        }
    }
}

