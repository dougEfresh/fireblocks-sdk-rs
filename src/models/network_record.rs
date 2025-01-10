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
pub struct NetworkRecord {
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<models::SourceTransferPeerPathResponse>,
    #[serde(rename = "destination", skip_serializing_if = "Option::is_none")]
    pub destination: Option<models::DestinationTransferPeerPathResponse>,
    #[serde(rename = "txHash", skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,
    #[serde(rename = "networkFee", skip_serializing_if = "Option::is_none")]
    pub network_fee: Option<String>,
    #[serde(rename = "assetId", skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<String>,
    /// The net amount of the transaction, after fee deduction
    #[serde(rename = "netAmount", skip_serializing_if = "Option::is_none")]
    pub net_amount: Option<String>,
    #[serde(rename = "isDropped", skip_serializing_if = "Option::is_none")]
    pub is_dropped: Option<bool>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "destinationAddress", skip_serializing_if = "Option::is_none")]
    pub destination_address: Option<String>,
    #[serde(rename = "sourceAddress", skip_serializing_if = "Option::is_none")]
    pub source_address: Option<String>,
    #[serde(rename = "amountUSD", skip_serializing_if = "Option::is_none")]
    pub amount_usd: Option<String>,
    #[serde(rename = "index", skip_serializing_if = "Option::is_none")]
    pub index: Option<f64>,
    #[serde(rename = "rewardInfo", skip_serializing_if = "Option::is_none")]
    pub reward_info: Option<models::RewardInfo>,
}

impl NetworkRecord {
    pub fn new() -> NetworkRecord {
        NetworkRecord {
            source: None,
            destination: None,
            tx_hash: None,
            network_fee: None,
            asset_id: None,
            net_amount: None,
            is_dropped: None,
            r#type: None,
            destination_address: None,
            source_address: None,
            amount_usd: None,
            index: None,
            reward_info: None,
        }
    }
}