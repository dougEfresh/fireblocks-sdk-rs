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

/// OneTimeAddressAccount : Non whitelisted destination address
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OneTimeAddressAccount {
    /// The destination address
    #[serde(rename = "oneTimeAddress")]
    pub one_time_address: String,
    /// required For Tag/Memo based assets only
    #[serde(rename = "tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl OneTimeAddressAccount {
    /// Non whitelisted destination address
    pub fn new(one_time_address: String) -> OneTimeAddressAccount {
        OneTimeAddressAccount {
            one_time_address,
            tag: None,
        }
    }
}

