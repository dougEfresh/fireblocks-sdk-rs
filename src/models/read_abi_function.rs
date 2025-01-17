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
pub struct ReadAbiFunction {
    #[serde(rename = "stateMutability")]
    pub state_mutability: StateMutability,
    #[serde(rename = "outputs", skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<models::Parameter>>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "inputs")]
    pub inputs: Vec<models::ParameterWithValue>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl ReadAbiFunction {
    pub fn new(
        state_mutability: StateMutability,
        r#type: String,
        inputs: Vec<models::ParameterWithValue>,
    ) -> ReadAbiFunction {
        ReadAbiFunction {
            state_mutability,
            outputs: None,
            name: None,
            r#type,
            inputs,
            description: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum StateMutability {
    #[serde(rename = "pure")]
    Pure,
    #[serde(rename = "view")]
    View,
}

impl Default for StateMutability {
    fn default() -> StateMutability {
        Self::Pure
    }
}
