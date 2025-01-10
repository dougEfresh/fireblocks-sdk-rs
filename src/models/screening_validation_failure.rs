// Fireblocks API
//
// Fireblocks provides a suite of applications to manage digital asset operations and a complete development platform to build your business on the blockchain.  - Visit our website for more information: [Fireblocks Website](https://fireblocks.com) - Visit our developer docs: [Fireblocks DevPortal](https://developers.fireblocks.com)
//
// The version of the OpenAPI document: 1.8.0
// Contact: developers@fireblocks.com
// Generated by: https://openapi-generator.tech

use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScreeningValidationFailure {
    #[serde(rename = "reason")]
    pub reason: Reason,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<std::collections::HashMap<String, serde_json::Value>>,
}

impl ScreeningValidationFailure {
    pub fn new(reason: Reason) -> ScreeningValidationFailure {
        ScreeningValidationFailure { reason, data: None }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Reason {
    #[serde(rename = "SCREENING_DISABLED_IN_TENANT")]
    ScreeningDisabledInTenant,
}

impl Default for Reason {
    fn default() -> Reason {
        Self::ScreeningDisabledInTenant
    }
}
