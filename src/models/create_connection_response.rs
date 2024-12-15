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
pub struct CreateConnectionResponse {
    /// The ID of the dApp connection initiated.
    #[serde(rename = "id")]
    pub id: String,
    /// Metadata of the dApp connection (provided by the dApp).
    #[serde(rename = "sessionMetadata")]
    pub session_metadata: models::SessionMetadata,
}

impl CreateConnectionResponse {
    pub fn new(id: String, session_metadata: models::SessionMetadata) -> CreateConnectionResponse {
        CreateConnectionResponse {
            id,
            session_metadata,
        }
    }
}

