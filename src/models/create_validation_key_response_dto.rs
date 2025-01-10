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
pub struct CreateValidationKeyResponseDto {
    /// Created validation key
    #[serde(rename = "validationKey")]
    pub validation_key: models::ValidationKeyDto,
    /// Admins who have to approve the validation key addition
    #[serde(rename = "admins")]
    pub admins: Vec<String>,
    /// Minimal number of approvers required. 0 for all
    #[serde(rename = "approvalThreshold")]
    pub approval_threshold: f64,
    /// Approval request id. Can be cancelled
    #[serde(rename = "requestId")]
    pub request_id: f64,
}

impl CreateValidationKeyResponseDto {
    pub fn new(
        validation_key: models::ValidationKeyDto,
        admins: Vec<String>,
        approval_threshold: f64,
        request_id: f64,
    ) -> CreateValidationKeyResponseDto {
        CreateValidationKeyResponseDto {
            validation_key,
            admins,
            approval_threshold,
            request_id,
        }
    }
}