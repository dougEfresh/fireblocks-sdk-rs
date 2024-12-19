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
pub struct ExchangeAsset {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "balance", skip_serializing_if = "Option::is_none")]
    pub balance: Option<String>,
    #[serde(rename = "lockedAmount", skip_serializing_if = "Option::is_none")]
    pub locked_amount: Option<String>,
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<String>,
    #[serde(rename = "available", skip_serializing_if = "Option::is_none")]
    pub available: Option<String>,
    #[serde(rename = "credit", skip_serializing_if = "Option::is_none")]
    pub credit: Option<String>,
}

impl ExchangeAsset {
    pub fn new() -> ExchangeAsset {
        ExchangeAsset {
            id: None,
            balance: None,
            locked_amount: None,
            total: None,
            available: None,
            credit: None,
        }
    }
}

