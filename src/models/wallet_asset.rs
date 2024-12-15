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
pub struct WalletAsset {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "balance", skip_serializing_if = "Option::is_none")]
    pub balance: Option<String>,
    #[serde(rename = "lockedAmount", skip_serializing_if = "Option::is_none")]
    pub locked_amount: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<models::ConfigChangeRequestStatus>,
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(rename = "activationTime", skip_serializing_if = "Option::is_none")]
    pub activation_time: Option<String>,
}

impl WalletAsset {
    pub fn new() -> WalletAsset {
        WalletAsset {
            id: None,
            balance: None,
            locked_amount: None,
            status: None,
            address: None,
            tag: None,
            activation_time: None,
        }
    }
}

