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
pub struct DisbursementOperationPreviewOutput {
    #[serde(rename = "instructionSet")]
    pub instruction_set: Vec<models::DisbursementOperationPreviewOutputInstructionSetInner>,
}

impl DisbursementOperationPreviewOutput {
    pub fn new(instruction_set: Vec<models::DisbursementOperationPreviewOutputInstructionSetInner>) -> DisbursementOperationPreviewOutput {
        DisbursementOperationPreviewOutput {
            instruction_set,
        }
    }
}

