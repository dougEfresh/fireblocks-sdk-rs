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

/// SourceTransferPeerPath : The source of the transaction.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SourceTransferPeerPath {
    #[serde(rename = "type")]
    pub r#type: models::TransferPeerPathType,
    #[serde(rename = "subType", skip_serializing_if = "Option::is_none")]
    pub sub_type: Option<models::TransferPeerPathSubType>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "walletId", skip_serializing_if = "Option::is_none")]
    pub wallet_id: Option<uuid::Uuid>,
}

impl SourceTransferPeerPath {
    /// The source of the transaction.
    pub fn new(r#type: models::TransferPeerPathType) -> SourceTransferPeerPath {
        SourceTransferPeerPath {
            r#type,
            sub_type: None,
            id: None,
            name: None,
            wallet_id: None,
        }
    }
}

