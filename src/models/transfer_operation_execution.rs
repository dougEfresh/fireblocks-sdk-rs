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
pub struct TransferOperationExecution {
    #[serde(rename = "input")]
    pub input: models::TransferOperationConfigParams,
    #[serde(rename = "output", skip_serializing_if = "Option::is_none")]
    pub output: Option<models::TransferOperationExecutionOutput>,
    #[serde(rename = "txId", skip_serializing_if = "Option::is_none")]
    pub tx_id: Option<String>,
    #[serde(rename = "startedAt")]
    pub started_at: f64,
    #[serde(rename = "finishedAt", skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<f64>,
    #[serde(rename = "failure", skip_serializing_if = "Option::is_none")]
    pub failure: Option<models::TransferOperationFailure>,
}

impl TransferOperationExecution {
    pub fn new(input: models::TransferOperationConfigParams, started_at: f64) -> TransferOperationExecution {
        TransferOperationExecution {
            input,
            output: None,
            tx_id: None,
            started_at,
            finished_at: None,
            failure: None,
        }
    }
}

