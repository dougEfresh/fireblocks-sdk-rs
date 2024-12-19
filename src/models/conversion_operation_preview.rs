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
pub struct ConversionOperationPreview {
    #[serde(rename = "input")]
    pub input: models::ConversionOperationConfigParams,
    #[serde(rename = "output", skip_serializing_if = "Option::is_none")]
    pub output: Option<models::ConversionOperationPreviewOutput>,
    #[serde(rename = "failure", skip_serializing_if = "Option::is_none")]
    pub failure: Option<models::ConversionOperationFailure>,
}

impl ConversionOperationPreview {
    pub fn new(input: models::ConversionOperationConfigParams) -> ConversionOperationPreview {
        ConversionOperationPreview {
            input,
            output: None,
            failure: None,
        }
    }
}

