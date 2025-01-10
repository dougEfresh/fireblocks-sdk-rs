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
pub struct CollectionMintRequestDto {
    /// The id of the vault account that initiates the mint function.
    #[serde(rename = "vaultAccountId")]
    pub vault_account_id: String,
    /// The EVM address to mint to
    #[serde(rename = "to")]
    pub to: String,
    /// The token id, recommended to have numerical format and in sequential
    /// order
    #[serde(rename = "tokenId")]
    pub token_id: String,
    /// For ERC721, amount is optional or should always be 1 and for ERC1155,
    /// amount should be 1 or greater
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    /// URL of metadata uploaded, skip uploading to IPFS if this field is
    /// provided with any value
    #[serde(rename = "metadataURI", skip_serializing_if = "Option::is_none")]
    pub metadata_uri: Option<String>,
    /// Metadata to upload
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<models::CollectionTokenMetadataDto>,
}

impl CollectionMintRequestDto {
    pub fn new(vault_account_id: String, to: String, token_id: String) -> CollectionMintRequestDto {
        CollectionMintRequestDto {
            vault_account_id,
            to,
            token_id,
            amount: None,
            metadata_uri: None,
            metadata: None,
        }
    }
}