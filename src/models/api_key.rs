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
pub struct ApiKey {
    /// The unique identifier of the API key
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    /// The date the API key was last seen
    #[serde(rename = "lastSeen")]
    pub last_seen: String,
}

impl ApiKey {
    pub fn new(id: uuid::Uuid, last_seen: String) -> ApiKey {
        ApiKey { id, last_seen }
    }
}
