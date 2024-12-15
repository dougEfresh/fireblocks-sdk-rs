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
pub struct Transaction {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "state")]
    pub state: State,
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<f64>,
    #[serde(rename = "instructionId", skip_serializing_if = "Option::is_none")]
    pub instruction_id: Option<String>,
}

impl Transaction {
    pub fn new(id: String, state: State) -> Transaction {
        Transaction {
            id,
            state,
            timestamp: None,
            instruction_id: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "SUBMITTED")]
    Submitted,
    #[serde(rename = "QUEUED")]
    Queued,
    #[serde(rename = "PENDING_AUTHORIZATION")]
    PendingAuthorization,
    #[serde(rename = "PENDING_SIGNATURE")]
    PendingSignature,
    #[serde(rename = "BROADCASTING")]
    Broadcasting,
    #[serde(rename = "PENDING_3RD_PARTY_MANUAL_APPROVAL")]
    Pending3RdPartyManualApproval,
    #[serde(rename = "PENDING_3RD_PARTY")]
    Pending3RdParty,
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "CONFIRMING")]
    Confirming,
    #[serde(rename = "CONFIRMED")]
    Confirmed,
    #[serde(rename = "COMPLETED")]
    Completed,
    #[serde(rename = "PARTIALLY_COMPLETED")]
    PartiallyCompleted,
    #[serde(rename = "PENDING_AML_SCREENING")]
    PendingAmlScreening,
    #[serde(rename = "CANCELLING")]
    Cancelling,
    #[serde(rename = "CANCELLED")]
    Cancelled,
    #[serde(rename = "REJECTED")]
    Rejected,
    #[serde(rename = "BLOCKED")]
    Blocked,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "TIMEOUT")]
    Timeout,
}

impl Default for State {
    fn default() -> State {
        Self::Submitted
    }
}

