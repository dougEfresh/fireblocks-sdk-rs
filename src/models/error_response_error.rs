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
pub struct ErrorResponseError {
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "message")]
    pub message: String,
}

impl ErrorResponseError {
    pub fn new(r#type: Type, message: String) -> ErrorResponseError {
        ErrorResponseError {
            r#type,
            message,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "INTERNAL")]
    Internal,
    #[serde(rename = "AUTHENTICATION")]
    Authentication,
    #[serde(rename = "AUTHORIZATION")]
    Authorization,
    #[serde(rename = "VALIDATION")]
    Validation,
    #[serde(rename = "NOT_FOUND")]
    NotFound,
    #[serde(rename = "UNPROCESSABLE_ENTITY")]
    UnprocessableEntity,
    #[serde(rename = "FORBIDDEN")]
    Forbidden,
}

impl Default for Type {
    fn default() -> Type {
        Self::Internal
    }
}

