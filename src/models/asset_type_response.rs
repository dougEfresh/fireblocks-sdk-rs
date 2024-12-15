/*
 * Fireblocks API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.7.5
 * Contact: support@fireblocks.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssetTypeResponse {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "contractAddress", skip_serializing_if = "Option::is_none")]
    pub contract_address: Option<String>,
    #[serde(rename = "nativeAsset", skip_serializing_if = "Option::is_none")]
    pub native_asset: Option<String>,
    #[serde(rename = "decimals", skip_serializing_if = "Option::is_none")]
    pub decimals: Option<f64>,
}

impl AssetTypeResponse {
    pub fn new(id: String, name: String, r#type: Type) -> AssetTypeResponse {
        AssetTypeResponse {
            id,
            name,
            r#type,
            contract_address: None,
            native_asset: None,
            decimals: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "ALGO_ASSET")]
    AlgoAsset,
    #[serde(rename = "BASE_ASSET")]
    BaseAsset,
    #[serde(rename = "BEP20")]
    Bep20,
    #[serde(rename = "COMPOUND")]
    Compound,
    #[serde(rename = "ERC20")]
    Erc20,
    #[serde(rename = "FIAT")]
    Fiat,
    #[serde(rename = "SOL_ASSET")]
    SolAsset,
    #[serde(rename = "TRON_TRC20")]
    TronTrc20,
    #[serde(rename = "XLM_ASSET")]
    XlmAsset,
    #[serde(rename = "XDB_ASSET")]
    XdbAsset,
}

impl Default for Type {
    fn default() -> Type {
        Self::AlgoAsset
    }
}

