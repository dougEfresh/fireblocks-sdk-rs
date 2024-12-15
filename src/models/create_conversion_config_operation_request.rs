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
pub struct CreateConversionConfigOperationRequest {
    #[serde(rename = "type")]
    pub r#type: models::ConversionOperationType,
    #[serde(rename = "params")]
    pub params: models::ConversionOperationConfigParams,
}

impl CreateConversionConfigOperationRequest {
    pub fn new(r#type: models::ConversionOperationType, params: models::ConversionOperationConfigParams) -> CreateConversionConfigOperationRequest {
        CreateConversionConfigOperationRequest {
            r#type,
            params,
        }
    }
}

