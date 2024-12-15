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
pub enum NetworkConnectionRoutingPolicySignetTest {
    NoneNetworkRoutingDest(models::NoneNetworkRoutingDest),
    CustomFiatRoutingDest(models::CustomFiatRoutingDest),
    DefaultNetworkRoutingDest(models::DefaultNetworkRoutingDest),
}

impl Default for NetworkConnectionRoutingPolicySignetTest {
    fn default() -> Self {
        Self::NoneNetworkRoutingDest(Default::default())
    }
}
/// The network routing logic.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Scheme {
    #[serde(rename = "DEFAULT")]
    Default,
}

impl Default for Scheme {
    fn default() -> Scheme {
        Self::Default
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

