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
pub struct ContractDoc {
    /// A description of the contract
    #[serde(rename = "details", skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    /// A description of the contract`s events
    #[serde(rename = "events", skip_serializing_if = "Option::is_none")]
    pub events: Option<String>,
    /// Is it devdoc or userdoc
    #[serde(rename = "kind")]
    pub kind: String,
    /// The description of the contract functions
    #[serde(rename = "methods")]
    pub methods: std::collections::HashMap<String, models::FunctionDoc>,
    /// The version of the contract
    #[serde(rename = "version")]
    pub version: String,
}

impl ContractDoc {
    pub fn new(kind: String, methods: std::collections::HashMap<String, models::FunctionDoc>, version: String) -> ContractDoc {
        ContractDoc {
            details: None,
            events: None,
            kind,
            methods,
            version,
        }
    }
}

