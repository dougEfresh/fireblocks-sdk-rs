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
pub struct CreateAddressResponse {
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "legacyAddress", skip_serializing_if = "Option::is_none")]
    pub legacy_address: Option<String>,
    #[serde(rename = "enterpriseAddress", skip_serializing_if = "Option::is_none")]
    pub enterprise_address: Option<String>,
    #[serde(rename = "tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(rename = "bip44AddressIndex", skip_serializing_if = "Option::is_none")]
    pub bip44_address_index: Option<i32>,
}

impl CreateAddressResponse {
    pub fn new() -> CreateAddressResponse {
        CreateAddressResponse {
            address: None,
            legacy_address: None,
            enterprise_address: None,
            tag: None,
            bip44_address_index: None,
        }
    }
}

