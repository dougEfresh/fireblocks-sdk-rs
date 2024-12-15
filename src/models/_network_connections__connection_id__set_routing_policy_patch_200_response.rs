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
pub struct NetworkConnectionsConnectionIdSetRoutingPolicyPatch200Response {
    #[serde(rename = "success")]
    pub success: bool,
}

impl NetworkConnectionsConnectionIdSetRoutingPolicyPatch200Response {
    pub fn new(success: bool) -> NetworkConnectionsConnectionIdSetRoutingPolicyPatch200Response {
        NetworkConnectionsConnectionIdSetRoutingPolicyPatch200Response {
            success,
        }
    }
}

