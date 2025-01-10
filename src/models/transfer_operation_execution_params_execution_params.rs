// Fireblocks API
//
// Fireblocks provides a suite of applications to manage digital asset operations and a complete development platform to build your business on the blockchain.  - Visit our website for more information: [Fireblocks Website](https://fireblocks.com) - Visit our developer docs: [Fireblocks DevPortal](https://developers.fireblocks.com)
//
// The version of the OpenAPI document: 1.8.0
// Contact: developers@fireblocks.com
// Generated by: https://openapi-generator.tech

use {
    crate::models,
    serde::{Deserialize, Serialize},
};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransferOperationExecutionParamsExecutionParams {
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(rename = "assetId", skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<String>,
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<models::Account>,
    #[serde(rename = "destination", skip_serializing_if = "Option::is_none")]
    pub destination: Option<models::Destination>,
}

impl TransferOperationExecutionParamsExecutionParams {
    pub fn new() -> TransferOperationExecutionParamsExecutionParams {
        TransferOperationExecutionParamsExecutionParams {
            amount: None,
            asset_id: None,
            source: None,
            destination: None,
        }
    }
}