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
pub struct SmartTransferTicketTermDto {
    /// Unique id of Smart Transfer ticket term
    #[serde(rename = "id")]
    pub id: String,
    /// Unique id of Smart Transfer ticket
    #[serde(rename = "ticketId")]
    pub ticket_id: String,
    /// Asset name
    #[serde(rename = "asset")]
    pub asset: String,
    /// Amount
    #[serde(rename = "amount")]
    pub amount: f64,
    /// Identifier of the origination Network Profile
    #[serde(rename = "fromNetworkId")]
    pub from_network_id: String,
    /// Source network name
    #[serde(rename = "fromNetworkIdName")]
    pub from_network_id_name: String,
    /// Identifier of the destination Network Profile
    #[serde(rename = "toNetworkId")]
    pub to_network_id: String,
    /// Destination network name
    #[serde(rename = "toNetworkIdName")]
    pub to_network_id_name: String,
    /// Blockchain TX hash
    #[serde(rename = "txHash", deserialize_with = "Option::deserialize")]
    pub tx_hash: Option<String>,
    /// Fireblocks transaction ID. It is set when the funding transaction is created.
    #[serde(rename = "fbTxId", deserialize_with = "Option::deserialize")]
    pub fb_tx_id: Option<String>,
    /// Ticket term transaction status
    #[serde(rename = "txStatus", deserialize_with = "Option::deserialize")]
    pub tx_status: Option<TxStatus>,
    /// Ticket term status
    #[serde(rename = "status")]
    pub status: Status,
    /// Date and time when the term is created.
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// Date and time of last term update.
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

impl SmartTransferTicketTermDto {
    pub fn new(id: String, ticket_id: String, asset: String, amount: f64, from_network_id: String, from_network_id_name: String, to_network_id: String, to_network_id_name: String, tx_hash: Option<String>, fb_tx_id: Option<String>, tx_status: Option<TxStatus>, status: Status, created_at: String, updated_at: String) -> SmartTransferTicketTermDto {
        SmartTransferTicketTermDto {
            id,
            ticket_id,
            asset,
            amount,
            from_network_id,
            from_network_id_name,
            to_network_id,
            to_network_id_name,
            tx_hash,
            fb_tx_id,
            tx_status,
            status,
            created_at,
            updated_at,
        }
    }
}
/// Ticket term transaction status
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TxStatus {
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "submitted")]
    Submitted,
    #[serde(rename = "pendingSignature")]
    PendingSignature,
    #[serde(rename = "pendingAuthorization")]
    PendingAuthorization,
    #[serde(rename = "broadcasting")]
    Broadcasting,
    #[serde(rename = "confirming")]
    Confirming,
    #[serde(rename = "pendingAmlScreening")]
    PendingAmlScreening,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "rejected")]
    Rejected,
    #[serde(rename = "blocked")]
    Blocked,
    #[serde(rename = "thirdPartyPendingManualApproval")]
    ThirdPartyPendingManualApproval,
    #[serde(rename = "thirdPartyPending")]
    ThirdPartyPending,
    #[serde(rename = "partiallyCompleted")]
    PartiallyCompleted,
    #[serde(rename = "cancelling")]
    Cancelling,
}

impl Default for TxStatus {
    fn default() -> TxStatus {
        Self::Queued
    }
}
/// Ticket term status
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "CREATED")]
    Created,
    #[serde(rename = "FUNDING")]
    Funding,
    #[serde(rename = "FUNDING_FAILED")]
    FundingFailed,
    #[serde(rename = "FUNDED")]
    Funded,
    #[serde(rename = "REJECTED")]
    Rejected,
}

impl Default for Status {
    fn default() -> Status {
        Self::Created
    }
}

