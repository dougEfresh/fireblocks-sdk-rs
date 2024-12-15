/*
 * Fireblocks API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.7.5
 * Contact: support@fireblocks.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmartTransferUserGroupsDto {
    /// Data object with result data
    #[serde(rename = "userGroupIds", deserialize_with = "Option::deserialize")]
    pub user_group_ids: Option<Vec<String>>,
}

impl SmartTransferUserGroupsDto {
    pub fn new(user_group_ids: Option<Vec<String>>) -> SmartTransferUserGroupsDto {
        SmartTransferUserGroupsDto {
            user_group_ids,
        }
    }
}

