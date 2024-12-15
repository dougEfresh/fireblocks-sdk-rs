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
pub struct ExchangeAccountsExchangeAccountIdInternalTransferPostRequest {
    #[serde(rename = "asset", skip_serializing_if = "Option::is_none")]
    pub asset: Option<String>,
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(rename = "sourceType", skip_serializing_if = "Option::is_none")]
    pub source_type: Option<models::TradingAccountType>,
    #[serde(rename = "destType", skip_serializing_if = "Option::is_none")]
    pub dest_type: Option<models::TradingAccountType>,
}

impl ExchangeAccountsExchangeAccountIdInternalTransferPostRequest {
    pub fn new() -> ExchangeAccountsExchangeAccountIdInternalTransferPostRequest {
        ExchangeAccountsExchangeAccountIdInternalTransferPostRequest {
            asset: None,
            amount: None,
            source_type: None,
            dest_type: None,
        }
    }
}

