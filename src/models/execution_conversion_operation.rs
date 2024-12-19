/*
 * Fireblocks API
 *
 * Fireblocks provides a suite of applications to manage digital asset operations and a complete development platform to build your business on the blockchain.  - Visit our website for more information: [Fireblocks Website](https://fireblocks.com) - Visit our developer docs: [Fireblocks DevPortal](https://developers.fireblocks.com) 
 *
 * The version of the OpenAPI document: 1.8.0
 * Contact: developers@fireblocks.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExecutionConversionOperation {
    #[serde(rename = "operationId")]
    pub operation_id: String,
    #[serde(rename = "status")]
    pub status: models::ExecutionOperationStatus,
    #[serde(rename = "validationFailure", skip_serializing_if = "Option::is_none")]
    pub validation_failure: Option<models::ConversionValidationFailure>,
    #[serde(rename = "operationType")]
    pub operation_type: models::ConversionOperationType,
    #[serde(rename = "preview", skip_serializing_if = "Option::is_none")]
    pub preview: Option<models::ConversionOperationPreview>,
    #[serde(rename = "execution", skip_serializing_if = "Option::is_none")]
    pub execution: Option<models::ConversionOperationExecution>,
}

impl ExecutionConversionOperation {
    pub fn new(operation_id: String, status: models::ExecutionOperationStatus, operation_type: models::ConversionOperationType) -> ExecutionConversionOperation {
        ExecutionConversionOperation {
            operation_id,
            status,
            validation_failure: None,
            operation_type,
            preview: None,
            execution: None,
        }
    }
}

