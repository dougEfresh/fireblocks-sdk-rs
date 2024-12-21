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
pub struct ResendWebhooksResponse {
    /// The amount of resent notifications
    #[serde(rename = "messagesCount", skip_serializing_if = "Option::is_none")]
    pub messages_count: Option<f64>,
}

impl ResendWebhooksResponse {
    pub fn new() -> ResendWebhooksResponse {
        ResendWebhooksResponse {
            messages_count: None,
        }
    }
}
