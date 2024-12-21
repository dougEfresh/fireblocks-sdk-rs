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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetOtaStatusResponse {
    SetOtaStatusResponseOneOf(models::SetOtaStatusResponseOneOf),
    /// Message indicating the result of the operation when config was
    /// successfully updated
    String(String),
}

impl Default for SetOtaStatusResponse {
    fn default() -> Self {
        Self::SetOtaStatusResponseOneOf(Default::default())
    }
}
