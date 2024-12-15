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
pub struct InternalWalletsPostRequest {
    /// the wallet's display name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Optional - Sets a customer reference ID
    #[serde(rename = "customerRefId", skip_serializing_if = "Option::is_none")]
    pub customer_ref_id: Option<String>,
}

impl InternalWalletsPostRequest {
    pub fn new() -> InternalWalletsPostRequest {
        InternalWalletsPostRequest {
            name: None,
            customer_ref_id: None,
        }
    }
}

