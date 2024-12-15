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
pub struct SourceTransferPeerPathResponse {
    #[serde(rename = "type")]
    pub r#type: String,
    /// In case the type is set to `EXCHANGE_ACCOUNT` or `FIAT_ACCOUNT`, the specific exchange vendor name or fiat vendor name. In case the type is set to `INTERNAL_WALLET` or `EXTERNAL_WALLET`, the subType is set to `Internal` or `External`.
    #[serde(rename = "subType", skip_serializing_if = "Option::is_none")]
    pub sub_type: Option<String>,
    /// The ID of the peer. You can retrieve the ID of each venue object using the endpoints for [listing vault accounts](https://developers.fireblocks.com/reference/get_vault-accounts-paged), [listing exchange account](https://developers.fireblocks.com/reference/get_exchange-accounts), [listing fiat accounts](https://developers.fireblocks.com/reference/get_fiat-accounts), [listing internal wallets](https://developers.fireblocks.com/reference/get_internal-wallets), [listing external wallets](https://developers.fireblocks.com/reference/get_external-wallets), [listing network connections](https://developers.fireblocks.com/reference/get_network-connections). For the other types, this parameter is not needed.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of the peer.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "walletId", skip_serializing_if = "Option::is_none")]
    pub wallet_id: Option<uuid::Uuid>,
}

impl SourceTransferPeerPathResponse {
    pub fn new(r#type: String) -> SourceTransferPeerPathResponse {
        SourceTransferPeerPathResponse {
            r#type,
            sub_type: None,
            id: None,
            name: None,
            wallet_id: None,
        }
    }
}

