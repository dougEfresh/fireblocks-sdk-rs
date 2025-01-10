// Fireblocks API
//
// Fireblocks provides a suite of applications to manage digital asset operations and a complete development platform to build your business on the blockchain.  - Visit our website for more information: [Fireblocks Website](https://fireblocks.com) - Visit our developer docs: [Fireblocks DevPortal](https://developers.fireblocks.com)
//
// The version of the OpenAPI document: 1.8.0
// Contact: developers@fireblocks.com
// Generated by: https://openapi-generator.tech

use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContractMetadataDto {
    /// The deployed contract ID
    #[serde(rename = "id")]
    pub id: String,
    /// The blockchain base asset ID
    #[serde(rename = "baseAssetId")]
    pub base_asset_id: String,
    /// The address of the token contract
    #[serde(rename = "contractAddress")]
    pub contract_address: String,
    /// The contract template ID
    #[serde(rename = "contractTemplateId")]
    pub contract_template_id: String,
    /// The vault account ID that initiated the request to issue the token
    #[serde(rename = "vaultAccountId", skip_serializing_if = "Option::is_none")]
    pub vault_account_id: Option<String>,
}

impl ContractMetadataDto {
    pub fn new(
        id: String,
        base_asset_id: String,
        contract_address: String,
        contract_template_id: String,
    ) -> ContractMetadataDto {
        ContractMetadataDto {
            id,
            base_asset_id,
            contract_address,
            contract_template_id,
            vault_account_id: None,
        }
    }
}