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
pub struct CreateConsoleUser {
    /// User's first name
    #[serde(rename = "firstName")]
    pub first_name: String,
    /// User' last name
    #[serde(rename = "lastName")]
    pub last_name: String,
    #[serde(rename = "role")]
    pub role: models::UserRole,
    /// User's valid email address
    #[serde(rename = "email")]
    pub email: String,
}

impl CreateConsoleUser {
    pub fn new(
        first_name: String,
        last_name: String,
        role: models::UserRole,
        email: String,
    ) -> CreateConsoleUser {
        CreateConsoleUser {
            first_name,
            last_name,
            role,
            email,
        }
    }
}