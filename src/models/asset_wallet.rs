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
pub struct AssetWallet {
    /// ID of the vault account. You can [get the vault account by this ID](https://developers.fireblocks.com/reference/get_vault-accounts-vaultaccountid) to retrieve vault properties such as its name, auto fueling, hidden on UI or customer reference ID.
    #[serde(rename = "vaultId", skip_serializing_if = "Option::is_none")]
    pub vault_id: Option<String>,
    /// ID of the asset. You can get more information about this asset by using the [supported assets API](https://developers.fireblocks.com/reference/get_supported-assets)
    #[serde(rename = "assetId", skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<String>,
    /// Available balance, available to use in a transaction.
    #[serde(rename = "available", skip_serializing_if = "Option::is_none")]
    pub available: Option<String>,
    /// Total balance at the asset wallet, as seen at the blockchain explorers. This includes balance available, and any kind of unavailable balance such as locked, frozen, or others.
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<String>,
    /// Pending balance.
    #[serde(rename = "pending", skip_serializing_if = "Option::is_none")]
    pub pending: Option<String>,
    /// Staked balance.
    #[serde(rename = "staked", skip_serializing_if = "Option::is_none")]
    pub staked: Option<String>,
    /// Funds frozen due to the anti-money laundering policy at this workspace.
    #[serde(rename = "frozen", skip_serializing_if = "Option::is_none")]
    pub frozen: Option<String>,
    /// Locked balance.
    #[serde(rename = "lockedAmount", skip_serializing_if = "Option::is_none")]
    pub locked_amount: Option<String>,
    /// The height (number) of the block of the balance. Can by empty.
    #[serde(rename = "blockHeight", skip_serializing_if = "Option::is_none")]
    pub block_height: Option<String>,
    /// The hash of the block of the balance. Can by empty.
    #[serde(rename = "blockHash", skip_serializing_if = "Option::is_none")]
    pub block_hash: Option<String>,
    /// Unix timestamp of the time the asset wallet was created.
    #[serde(rename = "creationTimestamp", skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
}

impl AssetWallet {
    pub fn new() -> AssetWallet {
        AssetWallet {
            vault_id: None,
            asset_id: None,
            available: None,
            total: None,
            pending: None,
            staked: None,
            frozen: None,
            locked_amount: None,
            block_height: None,
            block_hash: None,
            creation_timestamp: None,
        }
    }
}

