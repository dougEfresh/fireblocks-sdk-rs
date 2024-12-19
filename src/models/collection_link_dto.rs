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
pub struct CollectionLinkDto {
    /// The collection id
    #[serde(rename = "id")]
    pub id: String,
    /// The collection status
    #[serde(rename = "status")]
    pub status: Status,
    #[serde(rename = "type")]
    pub r#type: models::CollectionType,
    /// The display name of the collection. If was not provided, would be taken from the contract template
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// The collection's metadata
    #[serde(rename = "collectionMetadata", skip_serializing_if = "Option::is_none")]
    pub collection_metadata: Option<models::CollectionMetadataDto>,
}

impl CollectionLinkDto {
    pub fn new(id: String, status: Status, r#type: models::CollectionType) -> CollectionLinkDto {
        CollectionLinkDto {
            id,
            status,
            r#type,
            display_name: None,
            collection_metadata: None,
        }
    }
}
/// The collection status
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "COMPLETED")]
    Completed,
}

impl Default for Status {
    fn default() -> Status {
        Self::Pending
    }
}

