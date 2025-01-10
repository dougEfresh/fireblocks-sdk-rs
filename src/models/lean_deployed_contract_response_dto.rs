// Fireblocks API
//
// Fireblocks provides a suite of applications to manage digital asset operations and a complete development platform to build your business on the blockchain.  - Visit our website for more information: [Fireblocks Website](https://fireblocks.com) - Visit our developer docs: [Fireblocks DevPortal](https://developers.fireblocks.com)
//
// The version of the OpenAPI document: 1.8.0
// Contact: developers@fireblocks.com
// Generated by: https://openapi-generator.tech

use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LeanDeployedContractResponseDto {
    /// The deployed contract data identifier
    #[serde(rename = "id")]
    pub id: String,
    /// The contract's onchain address
    #[serde(rename = "contractAddress")]
    pub contract_address: String,
    /// The contract template identifier
    #[serde(rename = "contractTemplateId")]
    pub contract_template_id: String,
    /// The blockchain's base assetId
    #[serde(rename = "baseAssetId")]
    pub base_asset_id: String,
}

impl LeanDeployedContractResponseDto {
    pub fn new(
        id: String,
        contract_address: String,
        contract_template_id: String,
        base_asset_id: String,
    ) -> LeanDeployedContractResponseDto {
        LeanDeployedContractResponseDto {
            id,
            contract_address,
            contract_template_id,
            base_asset_id,
        }
    }
}
