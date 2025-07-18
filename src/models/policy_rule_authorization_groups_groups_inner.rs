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
pub struct PolicyRuleAuthorizationGroupsGroupsInner {
    /// User ids
    #[serde(rename = "users", skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<String>>,
    /// Group ids
    #[serde(rename = "usersGroups", skip_serializing_if = "Option::is_none")]
    pub users_groups: Option<Vec<String>>,
    /// Represents the min amount of entities which are required to approve the
    /// transaction, default is 1.
    #[serde(rename = "th", skip_serializing_if = "Option::is_none")]
    pub th: Option<f64>,
}

impl PolicyRuleAuthorizationGroupsGroupsInner {
    pub fn new() -> PolicyRuleAuthorizationGroupsGroupsInner {
        PolicyRuleAuthorizationGroupsGroupsInner {
            users: None,
            users_groups: None,
            th: None,
        }
    }
}
