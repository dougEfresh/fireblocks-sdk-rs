// Fireblocks API
//
// Fireblocks provides a suite of applications to manage digital asset operations and a complete development platform to build your business on the blockchain.  - Visit our website for more information: [Fireblocks Website](https://fireblocks.com) - Visit our developer docs: [Fireblocks DevPortal](https://developers.fireblocks.com)
//
// The version of the OpenAPI document: 1.8.0
// Contact: developers@fireblocks.com
// Generated by: https://openapi-generator.tech

use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SetConfirmationsThresholdRequest {
    #[serde(rename = "numOfConfirmations", skip_serializing_if = "Option::is_none")]
    pub num_of_confirmations: Option<f64>,
}

impl SetConfirmationsThresholdRequest {
    pub fn new() -> SetConfirmationsThresholdRequest {
        SetConfirmationsThresholdRequest {
            num_of_confirmations: None,
        }
    }
}
