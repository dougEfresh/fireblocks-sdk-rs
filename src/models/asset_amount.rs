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
pub struct AssetAmount {
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "assetId")]
    pub asset_id: String,
}

impl AssetAmount {
    pub fn new(amount: String, asset_id: String) -> AssetAmount {
        AssetAmount {
            amount,
            asset_id,
        }
    }
}

