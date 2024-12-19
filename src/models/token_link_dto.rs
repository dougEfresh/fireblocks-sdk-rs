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
pub struct TokenLinkDto {
    /// The token link id
    #[serde(rename = "id")]
    pub id: String,
    /// The token status
    #[serde(rename = "status")]
    pub status: Status,
    /// The type of token
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    /// The Fireblocks' reference id
    #[serde(rename = "refId", skip_serializing_if = "Option::is_none")]
    pub ref_id: Option<String>,
    /// The token display name. If was not provided, would be taken from the contract template
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "tokenMetadata", skip_serializing_if = "Option::is_none")]
    pub token_metadata: Option<models::TokenLinkDtoTokenMetadata>,
}

impl TokenLinkDto {
    pub fn new(id: String, status: Status) -> TokenLinkDto {
        TokenLinkDto {
            id,
            status,
            r#type: None,
            ref_id: None,
            display_name: None,
            token_metadata: None,
        }
    }
}
/// The token status
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
/// The type of token
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "FUNGIBLE_TOKEN")]
    FungibleToken,
    #[serde(rename = "NON_FUNGIBLE_TOKEN")]
    NonFungibleToken,
    #[serde(rename = "TOKEN_UTILITY")]
    TokenUtility,
    #[serde(rename = "TOKEN_EXTENSION")]
    TokenExtension,
}

impl Default for Type {
    fn default() -> Type {
        Self::FungibleToken
    }
}

