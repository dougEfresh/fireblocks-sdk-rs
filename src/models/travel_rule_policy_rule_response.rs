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
pub struct TravelRulePolicyRuleResponse {
    #[serde(rename = "sourceType", skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    #[serde(rename = "sourceSubType", skip_serializing_if = "Option::is_none")]
    pub source_sub_type: Option<String>,
    #[serde(rename = "destType", skip_serializing_if = "Option::is_none")]
    pub dest_type: Option<String>,
    #[serde(rename = "destSubType", skip_serializing_if = "Option::is_none")]
    pub dest_sub_type: Option<String>,
    #[serde(rename = "destAddress", skip_serializing_if = "Option::is_none")]
    pub dest_address: Option<String>,
    #[serde(rename = "sourceId", skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    #[serde(rename = "destId", skip_serializing_if = "Option::is_none")]
    pub dest_id: Option<String>,
    #[serde(rename = "asset", skip_serializing_if = "Option::is_none")]
    pub asset: Option<String>,
    #[serde(rename = "baseAsset", skip_serializing_if = "Option::is_none")]
    pub base_asset: Option<String>,
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[serde(rename = "amountUSD", skip_serializing_if = "Option::is_none")]
    pub amount_usd: Option<f64>,
    #[serde(rename = "networkProtocol", skip_serializing_if = "Option::is_none")]
    pub network_protocol: Option<String>,
    #[serde(rename = "operation", skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(rename = "action")]
    pub action: Action,
}

impl TravelRulePolicyRuleResponse {
    pub fn new(action: Action) -> TravelRulePolicyRuleResponse {
        TravelRulePolicyRuleResponse {
            source_type: None,
            source_sub_type: None,
            dest_type: None,
            dest_sub_type: None,
            dest_address: None,
            source_id: None,
            dest_id: None,
            asset: None,
            base_asset: None,
            amount: None,
            amount_usd: None,
            network_protocol: None,
            operation: None,
            action,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "SCREEN")]
    Screen,
    #[serde(rename = "PASS")]
    Pass,
    #[serde(rename = "FREEZE")]
    Freeze,
}

impl Default for Action {
    fn default() -> Action {
        Self::Screen
    }
}
