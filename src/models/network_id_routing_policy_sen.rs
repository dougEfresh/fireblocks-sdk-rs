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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NetworkIdRoutingPolicySen {
    CustomFiatRoutingDest(models::CustomFiatRoutingDest),
    NoneNetworkRoutingDest(models::NoneNetworkRoutingDest),
}

impl Default for NetworkIdRoutingPolicySen {
    fn default() -> Self {
        Self::CustomFiatRoutingDest(Default::default())
    }
}
/// No network routing logic.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Scheme {
    #[serde(rename = "NONE")]
    None,
}

impl Default for Scheme {
    fn default() -> Scheme {
        Self::None
    }
}
/// The fiat account the funds are being sent to.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DstType {
    #[serde(rename = "FIAT_ACCOUNT")]
    FiatAccount,
}

impl Default for DstType {
    fn default() -> DstType {
        Self::FiatAccount
    }
}

