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
pub struct ContractAttributes {
    #[serde(rename = "useCases")]
    pub use_cases: Vec<String>,
    #[serde(rename = "standards")]
    pub standards: Vec<String>,
    #[serde(rename = "auditor")]
    pub auditor: models::AuditorData,
}

impl ContractAttributes {
    pub fn new(
        use_cases: Vec<String>,
        standards: Vec<String>,
        auditor: models::AuditorData,
    ) -> ContractAttributes {
        ContractAttributes {
            use_cases,
            standards,
            auditor,
        }
    }
}