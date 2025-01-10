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

/// SourceTransferPeerPathResponse : Source of the transaction.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SourceTransferPeerPathResponse {
    #[serde(rename = "type")]
    pub r#type: models::TransferPeerPathType,
    /// In case the type is set to `EXCHANGE_ACCOUNT` or `FIAT_ACCOUNT`, the
    /// specific exchange vendor name or fiat vendor name. In case the type is
    /// set to `INTERNAL_WALLET` or `EXTERNAL_WALLET`, the subType is set to
    /// `Internal` or `External`.
    #[serde(rename = "subType", skip_serializing_if = "Option::is_none")]
    pub sub_type: Option<String>,
    /// The ID of the peer. You can retrieve the ID of each venue object using the endpoints for [listing vault accounts](https://developers.fireblocks.com/reference/get_vault-accounts-paged), [listing exchange account](https://developers.fireblocks.com/reference/get_exchange-accounts), [listing fiat accounts](https://developers.fireblocks.com/reference/get_fiat-accounts), [listing internal wallets](https://developers.fireblocks.com/reference/get_internal-wallets), [listing external wallets](https://developers.fireblocks.com/reference/get_external-wallets), [listing network connections](https://developers.fireblocks.com/reference/get_network-connections). For the other types, this parameter is not needed.
    #[serde(
        rename = "id",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub id: Option<Option<String>>,
    /// The name of the peer
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "walletId", skip_serializing_if = "Option::is_none")]
    pub wallet_id: Option<uuid::Uuid>,
    /// If this transaction is an exchange internal transfer, this field will be
    /// populated with the type of that trading account.
    #[serde(
        rename = "tradingAccount",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub trading_account: Option<Option<String>>,
}

impl SourceTransferPeerPathResponse {
    /// Source of the transaction.
    pub fn new(r#type: models::TransferPeerPathType) -> SourceTransferPeerPathResponse {
        SourceTransferPeerPathResponse {
            r#type,
            sub_type: None,
            id: None,
            name: None,
            wallet_id: None,
            trading_account: None,
        }
    }
}