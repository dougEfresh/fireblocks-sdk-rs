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
pub struct SettlementRequestBody {
    #[serde(
        rename = "mainExchangeAccountId",
        skip_serializing_if = "Option::is_none"
    )]
    pub main_exchange_account_id: Option<String>,
}

impl SettlementRequestBody {
    pub fn new() -> SettlementRequestBody {
        SettlementRequestBody {
            main_exchange_account_id: None,
        }
    }
}
