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
pub struct CollectionTokenMetadataAttributeDto {
    /// Name of the trait
    #[serde(rename = "trait_type")]
    pub trait_type: String,
    /// Value of the trait
    #[serde(rename = "value")]
    pub value: String,
    /// A field indicating how you would like trait to be displayed
    #[serde(rename = "display_type", skip_serializing_if = "Option::is_none")]
    pub display_type: Option<String>,
}

impl CollectionTokenMetadataAttributeDto {
    pub fn new(trait_type: String, value: String) -> CollectionTokenMetadataAttributeDto {
        CollectionTokenMetadataAttributeDto {
            trait_type,
            value,
            display_type: None,
        }
    }
}

