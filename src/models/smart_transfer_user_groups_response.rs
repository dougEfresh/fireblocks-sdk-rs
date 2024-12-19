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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmartTransferUserGroupsResponse {
    /// Result message
    #[serde(rename = "message", deserialize_with = "Option::deserialize")]
    pub message: Option<String>,
    #[serde(rename = "data", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub data: Option<Option<models::SmartTransferUserGroups>>,
}

impl SmartTransferUserGroupsResponse {
    pub fn new(message: Option<String>) -> SmartTransferUserGroupsResponse {
        SmartTransferUserGroupsResponse {
            message,
            data: None,
        }
    }
}

