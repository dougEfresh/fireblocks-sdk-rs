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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WorkflowExecutionOperation {
    ExecutionScreeningOperation(models::ExecutionScreeningOperation),
    ExecutionConversionOperation(models::ExecutionConversionOperation),
    ExecutionTransferOperation(models::ExecutionTransferOperation),
    ExecutionDisbursementOperation(models::ExecutionDisbursementOperation),
}

impl Default for WorkflowExecutionOperation {
    fn default() -> Self {
        Self::ExecutionScreeningOperation(Default::default())
    }
}

