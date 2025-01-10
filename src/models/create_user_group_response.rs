// Fireblocks API
//
// Fireblocks provides a suite of applications to manage digital asset operations and a complete development platform to build your business on the blockchain.  - Visit our website for more information: [Fireblocks Website](https://fireblocks.com) - Visit our developer docs: [Fireblocks DevPortal](https://developers.fireblocks.com)
//
// The version of the OpenAPI document: 1.8.0
// Contact: developers@fireblocks.com
// Generated by: https://openapi-generator.tech

use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateUserGroupResponse {
    /// User group unique identifier
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// User group name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// IDs of the users in the group
    #[serde(rename = "memberIds", skip_serializing_if = "Option::is_none")]
    pub member_ids: Option<Vec<String>>,
    /// User group status
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
}

impl CreateUserGroupResponse {
    pub fn new() -> CreateUserGroupResponse {
        CreateUserGroupResponse {
            id: None,
            name: None,
            member_ids: None,
            status: None,
        }
    }
}
/// User group status
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "PENDING_APPROVAL")]
    PendingApproval,
    #[serde(rename = "ACTIVE")]
    Active,
}

impl Default for Status {
    fn default() -> Status {
        Self::PendingApproval
    }
}