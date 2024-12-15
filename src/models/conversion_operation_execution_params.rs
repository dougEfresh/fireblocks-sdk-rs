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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConversionOperationExecutionParams {
    #[serde(rename = "configOperationId")]
    pub config_operation_id: String,
    #[serde(rename = "executionParams", skip_serializing_if = "Option::is_none")]
    pub execution_params: Option<models::ConversionOperationExecutionParamsExecutionParams>,
}

impl ConversionOperationExecutionParams {
    pub fn new(config_operation_id: String) -> ConversionOperationExecutionParams {
        ConversionOperationExecutionParams {
            config_operation_id,
            execution_params: None,
        }
    }
}

