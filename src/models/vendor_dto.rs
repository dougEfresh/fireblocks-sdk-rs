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
pub struct VendorDto {
    /// The unique identifier of the vendor of this contract template
    #[serde(rename = "id")]
    pub id: String,
    /// The name of the vendor of this contract template
    #[serde(rename = "name")]
    pub name: String,
}

impl VendorDto {
    pub fn new(id: String, name: String) -> VendorDto {
        VendorDto {
            id,
            name,
        }
    }
}

