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

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ExecutionOperationStatus {
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "VALIDATION_IN_PROGRESS")]
    ValidationInProgress,
    #[serde(rename = "VALIDATION_FAILED")]
    ValidationFailed,
    #[serde(rename = "VALIDATION_COMPLETED")]
    ValidationCompleted,
    #[serde(rename = "PREVIEW_REQUESTED")]
    PreviewRequested,
    #[serde(rename = "PREVIEW_IN_PROGRESS")]
    PreviewInProgress,
    #[serde(rename = "PREVIEW_FAILED")]
    PreviewFailed,
    #[serde(rename = "READY_FOR_LAUNCH")]
    ReadyForLaunch,
    #[serde(rename = "EXECUTION_REQUESTED")]
    ExecutionRequested,
    #[serde(rename = "EXECUTION_IN_PROGRESS")]
    ExecutionInProgress,
    #[serde(rename = "EXECUTION_COMPLETED")]
    ExecutionCompleted,
    #[serde(rename = "EXECUTION_FAILED")]
    ExecutionFailed,

}

impl std::fmt::Display for ExecutionOperationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Pending => write!(f, "PENDING"),
            Self::ValidationInProgress => write!(f, "VALIDATION_IN_PROGRESS"),
            Self::ValidationFailed => write!(f, "VALIDATION_FAILED"),
            Self::ValidationCompleted => write!(f, "VALIDATION_COMPLETED"),
            Self::PreviewRequested => write!(f, "PREVIEW_REQUESTED"),
            Self::PreviewInProgress => write!(f, "PREVIEW_IN_PROGRESS"),
            Self::PreviewFailed => write!(f, "PREVIEW_FAILED"),
            Self::ReadyForLaunch => write!(f, "READY_FOR_LAUNCH"),
            Self::ExecutionRequested => write!(f, "EXECUTION_REQUESTED"),
            Self::ExecutionInProgress => write!(f, "EXECUTION_IN_PROGRESS"),
            Self::ExecutionCompleted => write!(f, "EXECUTION_COMPLETED"),
            Self::ExecutionFailed => write!(f, "EXECUTION_FAILED"),
        }
    }
}

impl Default for ExecutionOperationStatus {
    fn default() -> ExecutionOperationStatus {
        Self::Pending
    }
}

