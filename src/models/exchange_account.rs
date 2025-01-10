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
pub struct ExchangeAccount {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<models::ExchangeType>,
    /// Display name of the exchange account
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "assets", skip_serializing_if = "Option::is_none")]
    pub assets: Option<Vec<models::ExchangeAsset>>,
    /// Did succeed in retrieve balance data
    #[serde(rename = "success", skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    #[serde(rename = "tradingAccounts", skip_serializing_if = "Option::is_none")]
    pub trading_accounts: Option<Vec<models::ExchangeTradingAccount>>,
    /// True if the account is a subaccount in an exchange
    #[serde(rename = "isSubaccount", skip_serializing_if = "Option::is_none")]
    pub is_subaccount: Option<bool>,
    /// if the account is a sub-account, the ID of the main account
    #[serde(rename = "mainAccountId", skip_serializing_if = "Option::is_none")]
    pub main_account_id: Option<String>,
}

impl ExchangeAccount {
    pub fn new() -> ExchangeAccount {
        ExchangeAccount {
            id: None,
            r#type: None,
            name: None,
            status: None,
            assets: None,
            success: None,
            trading_accounts: None,
            is_subaccount: None,
            main_account_id: None,
        }
    }
}