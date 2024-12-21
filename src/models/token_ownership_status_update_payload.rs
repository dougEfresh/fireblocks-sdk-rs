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
pub struct TokenOwnershipStatusUpdatePayload {
    /// Token's asset id
    #[serde(rename = "assetId")]
    pub asset_id: String,
    /// Token's ownership new status
    #[serde(rename = "status")]
    pub status: Status,
}

impl TokenOwnershipStatusUpdatePayload {
    pub fn new(asset_id: String, status: Status) -> TokenOwnershipStatusUpdatePayload {
        TokenOwnershipStatusUpdatePayload { asset_id, status }
    }
}
/// Token's ownership new status
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "LISTED")]
    Listed,
    #[serde(rename = "ARCHIVED")]
    Archived,
}

impl Default for Status {
    fn default() -> Status {
        Self::Listed
    }
}
