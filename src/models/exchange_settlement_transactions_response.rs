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
pub struct ExchangeSettlementTransactionsResponse {
    #[serde(rename = "toExchange", skip_serializing_if = "Option::is_none")]
    pub to_exchange: Option<Vec<models::ToExchangeTransaction>>,
    #[serde(rename = "toCollateral", skip_serializing_if = "Option::is_none")]
    pub to_collateral: Option<Vec<models::ToCollateralTransaction>>,
}

impl ExchangeSettlementTransactionsResponse {
    pub fn new() -> ExchangeSettlementTransactionsResponse {
        ExchangeSettlementTransactionsResponse {
            to_exchange: None,
            to_collateral: None,
        }
    }
}

