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
pub struct UpdateTokenOwnershipStatusDto {
    #[serde(rename = "status")]
    pub status: Status,
}

impl UpdateTokenOwnershipStatusDto {
    pub fn new(status: Status) -> UpdateTokenOwnershipStatusDto {
        UpdateTokenOwnershipStatusDto {
            status,
        }
    }
}
/// 
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

