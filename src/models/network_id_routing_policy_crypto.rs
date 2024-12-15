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
pub enum NetworkIdRoutingPolicyCrypto {
    CustomCryptoRoutingDest(models::CustomCryptoRoutingDest),
    NoneNetworkRoutingDest(models::NoneNetworkRoutingDest),
}

impl Default for NetworkIdRoutingPolicyCrypto {
    fn default() -> Self {
        Self::CustomCryptoRoutingDest(Default::default())
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
/// The type of destination account the funds are being sent to.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DstType {
    #[serde(rename = "VAULT")]
    Vault,
    #[serde(rename = "EXCHANGE")]
    Exchange,
}

impl Default for DstType {
    fn default() -> DstType {
        Self::Vault
    }
}

