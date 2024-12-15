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
pub enum CreateConfigOperationRequest {
    CreateConversionConfigOperationRequest(models::CreateConversionConfigOperationRequest),
    CreateTransferConfigOperationRequest(models::CreateTransferConfigOperationRequest),
    CreateDisbursementConfigOperationRequest(models::CreateDisbursementConfigOperationRequest),
}

impl Default for CreateConfigOperationRequest {
    fn default() -> Self {
        Self::CreateConversionConfigOperationRequest(Default::default())
    }
}

