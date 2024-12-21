// Fireblocks API
//
// Fireblocks provides a suite of applications to manage digital asset operations and a complete development platform to build your business on the blockchain.  - Visit our website for more information: [Fireblocks Website](https://fireblocks.com) - Visit our developer docs: [Fireblocks DevPortal](https://developers.fireblocks.com)
//
// The version of the OpenAPI document: 1.8.0
// Contact: developers@fireblocks.com
// Generated by: https://openapi-generator.tech

use {
    crate::models,
    serde::{Deserialize, Serialize},
};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TokenLinkRequestDto {
    /// The type of token being linked
    #[serde(rename = "type")]
    pub r#type: Type,
    /// The Fireblocks' token link reference id. For example, 'BQ5R_BDESC_ABC'
    /// if it's a fungible       asset
    #[serde(rename = "refId", skip_serializing_if = "Option::is_none")]
    pub ref_id: Option<String>,
    /// The token display name
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// The blockchain base assetId
    #[serde(rename = "baseAssetId", skip_serializing_if = "Option::is_none")]
    pub base_asset_id: Option<String>,
    /// The contract's onchain address
    #[serde(rename = "contractAddress", skip_serializing_if = "Option::is_none")]
    pub contract_address: Option<String>,
}

impl TokenLinkRequestDto {
    pub fn new(r#type: Type) -> TokenLinkRequestDto {
        TokenLinkRequestDto {
            r#type,
            ref_id: None,
            display_name: None,
            base_asset_id: None,
            contract_address: None,
        }
    }
}
/// The type of token being linked
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
