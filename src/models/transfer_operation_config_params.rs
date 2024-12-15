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
pub struct TransferOperationConfigParams {
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(rename = "assetId", skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<String>,
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<models::Account>,
    #[serde(rename = "destination")]
    pub destination: models::Destination,
}

impl TransferOperationConfigParams {
    pub fn new(destination: models::Destination) -> TransferOperationConfigParams {
        TransferOperationConfigParams {
            amount: None,
            asset_id: None,
            source: None,
            destination,
        }
    }
}

