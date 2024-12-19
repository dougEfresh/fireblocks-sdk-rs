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
pub struct SmartTransferNotFoundResponse {
    /// Not found error message
    #[serde(rename = "message")]
    pub message: String,
    /// Error code
    #[serde(rename = "code")]
    pub code: String,
}

impl SmartTransferNotFoundResponse {
    pub fn new(message: String, code: String) -> SmartTransferNotFoundResponse {
        SmartTransferNotFoundResponse {
            message,
            code,
        }
    }
}

