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
pub struct Parameter {
    /// The name of the parameter as it appears in the ABI
    #[serde(rename = "name")]
    pub name: String,
    /// A description of the parameter, fetched from the devdoc of this contract
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The internal type of the parameter as it appears in the ABI
    #[serde(rename = "internalType", skip_serializing_if = "Option::is_none")]
    pub internal_type: Option<String>,
    /// The type of the parameter as it appears in the ABI
    #[serde(rename = "type")]
    pub r#type: String,
    /// In case it’s a struct, it will hold the struct data
    #[serde(rename = "components", skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<String>>,
}

impl Parameter {
    pub fn new(name: String, r#type: String) -> Parameter {
        Parameter {
            name,
            description: None,
            internal_type: None,
            r#type,
            components: None,
        }
    }
}

