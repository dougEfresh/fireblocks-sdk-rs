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
pub struct ContractWithAbiDto {
    /// The address of the contract
    #[serde(rename = "contractAddress")]
    pub contract_address: String,
    /// The blockchain base assetId
    #[serde(rename = "baseAssetId")]
    pub base_asset_id: String,
    /// The name of the contract
    #[serde(rename = "name")]
    pub name: String,
    /// The ABI of the contract
    #[serde(rename = "abi")]
    pub abi: Vec<models::AbiFunction>,
    /// Whether the contract is a proxy contract
    #[serde(rename = "isProxy", skip_serializing_if = "Option::is_none")]
    pub is_proxy: Option<bool>,
    /// The implementation contract address
    #[serde(rename = "implementation", skip_serializing_if = "Option::is_none")]
    pub implementation: Option<String>,
    /// Whether the contract ABI is public
    #[serde(rename = "isPublic")]
    pub is_public: bool,
}

impl ContractWithAbiDto {
    pub fn new(contract_address: String, base_asset_id: String, name: String, abi: Vec<models::AbiFunction>, is_public: bool) -> ContractWithAbiDto {
        ContractWithAbiDto {
            contract_address,
            base_asset_id,
            name,
            abi,
            is_proxy: None,
            implementation: None,
            is_public,
        }
    }
}

