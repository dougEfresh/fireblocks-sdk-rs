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
pub struct TransferOperationFailure {
    #[serde(rename = "reason")]
    pub reason: Reason,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<models::TransferOperationFailureData>,
}

impl TransferOperationFailure {
    pub fn new(reason: Reason) -> TransferOperationFailure {
        TransferOperationFailure {
            reason,
            data: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Reason {
    #[serde(rename = "INVALID_AMOUNT")]
    InvalidAmount,
    #[serde(rename = "SUBMISSION_FAILED")]
    SubmissionFailed,
    #[serde(rename = "TRANSACTION_FAILED")]
    TransactionFailed,
}

impl Default for Reason {
    fn default() -> Reason {
        Self::InvalidAmount
    }
}

