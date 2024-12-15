/*
 * Fireblocks API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.7.5
 * Contact: support@fireblocks.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// PublishResult : Response object of the publish policy operation
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublishResult {
    #[serde(rename = "status")]
    pub status: models::PolicyStatus,
    #[serde(rename = "rules")]
    pub rules: Vec<models::PolicyRule>,
    #[serde(rename = "checkResult")]
    pub check_result: models::PolicyCheckResult,
    #[serde(rename = "metadata")]
    pub metadata: models::PolicyMetadata,
}

impl PublishResult {
    /// Response object of the publish policy operation
    pub fn new(status: models::PolicyStatus, rules: Vec<models::PolicyRule>, check_result: models::PolicyCheckResult, metadata: models::PolicyMetadata) -> PublishResult {
        PublishResult {
            status,
            rules,
            check_result,
            metadata,
        }
    }
}

