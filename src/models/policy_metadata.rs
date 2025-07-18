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

/// PolicyMetadata : Policy related metadata
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicyMetadata {
    /// The user id of the user who last edited the policy
    #[serde(rename = "editedBy", skip_serializing_if = "Option::is_none")]
    pub edited_by: Option<String>,
    /// The timestamp of the last edit of the policy
    #[serde(rename = "editedAt", skip_serializing_if = "Option::is_none")]
    pub edited_at: Option<String>,
    /// The user id of the user who last published the policy
    #[serde(rename = "publishedBy", skip_serializing_if = "Option::is_none")]
    pub published_by: Option<String>,
    /// The timestamp of the last publish of the policy
    #[serde(rename = "publishedAt", skip_serializing_if = "Option::is_none")]
    pub published_at: Option<String>,
}

impl PolicyMetadata {
    /// Policy related metadata
    pub fn new() -> PolicyMetadata {
        PolicyMetadata {
            edited_by: None,
            edited_at: None,
            published_by: None,
            published_at: None,
        }
    }
}
