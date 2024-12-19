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
pub struct ScreeningPolicyResponse {
    #[serde(rename = "policy")]
    pub policy: models::TravelRulePolicyRuleResponse,
    #[serde(rename = "policyStatus", skip_serializing_if = "Option::is_none")]
    pub policy_status: Option<String>,
    #[serde(rename = "isDefault")]
    pub is_default: bool,
    #[serde(rename = "createDate", skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    #[serde(rename = "lastUpdate")]
    pub last_update: String,
}

impl ScreeningPolicyResponse {
    pub fn new(policy: models::TravelRulePolicyRuleResponse, is_default: bool, last_update: String) -> ScreeningPolicyResponse {
        ScreeningPolicyResponse {
            policy,
            policy_status: None,
            is_default,
            create_date: None,
            last_update,
        }
    }
}

