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

/// PolicyResponse : Response object for policy operations
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicyResponse {
    /// A set of policy rules
    #[serde(rename = "rules")]
    pub rules: Vec<models::PolicyRule>,
    #[serde(rename = "metadata")]
    pub metadata: models::PolicyMetadata,
}

impl PolicyResponse {
    /// Response object for policy operations
    pub fn new(rules: Vec<models::PolicyRule>, metadata: models::PolicyMetadata) -> PolicyResponse {
        PolicyResponse { rules, metadata }
    }
}
