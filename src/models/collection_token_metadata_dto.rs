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
pub struct CollectionTokenMetadataDto {
    /// Token's name
    #[serde(rename = "name")]
    pub name: String,
    /// Token's description
    #[serde(rename = "description")]
    pub description: String,
    /// Token's image URL
    #[serde(rename = "image", skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// Token's animation URL
    #[serde(rename = "animation_url", skip_serializing_if = "Option::is_none")]
    pub animation_url: Option<String>,
    /// Token's external URL
    #[serde(rename = "external_url", skip_serializing_if = "Option::is_none")]
    pub external_url: Option<String>,
    /// Token's metadata attributes
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<models::CollectionTokenMetadataAttributeDto>>,
}

impl CollectionTokenMetadataDto {
    pub fn new(name: String, description: String) -> CollectionTokenMetadataDto {
        CollectionTokenMetadataDto {
            name,
            description,
            image: None,
            animation_url: None,
            external_url: None,
            attributes: None,
        }
    }
}

