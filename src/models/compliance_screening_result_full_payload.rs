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
pub struct ComplianceScreeningResultFullPayload {
    #[serde(rename = "provider", skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    /// The payload of the screening result. The payload is a JSON object that
    /// contains the screening result. The payload is different for each
    /// screening provider.
    #[serde(rename = "payload", skip_serializing_if = "Option::is_none")]
    pub payload: Option<serde_json::Value>,
    #[serde(rename = "bypassReason", skip_serializing_if = "Option::is_none")]
    pub bypass_reason: Option<String>,
    #[serde(rename = "screeningStatus", skip_serializing_if = "Option::is_none")]
    pub screening_status: Option<ScreeningStatus>,
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<f64>,
}

impl ComplianceScreeningResultFullPayload {
    pub fn new() -> ComplianceScreeningResultFullPayload {
        ComplianceScreeningResultFullPayload {
            provider: None,
            payload: None,
            bypass_reason: None,
            screening_status: None,
            timestamp: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ScreeningStatus {
    #[serde(rename = "COMPLETED")]
    Completed,
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "BYPASSED")]
    Bypassed,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "FROZEN")]
    Frozen,
}

impl Default for ScreeningStatus {
    fn default() -> ScreeningStatus {
        Self::Completed
    }
}
