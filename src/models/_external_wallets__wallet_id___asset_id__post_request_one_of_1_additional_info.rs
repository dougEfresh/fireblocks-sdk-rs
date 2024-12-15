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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExternalWalletsWalletIdAssetIdPostRequestOneOf1AdditionalInfo {
    ExternalWalletsWalletIdAssetIdPostRequestOneOf1AdditionalInfoOneOf(models::ExternalWalletsWalletIdAssetIdPostRequestOneOf1AdditionalInfoOneOf),
    ExternalWalletsWalletIdAssetIdPostRequestOneOf1AdditionalInfoOneOf1(models::ExternalWalletsWalletIdAssetIdPostRequestOneOf1AdditionalInfoOneOf1),
    ExternalWalletsWalletIdAssetIdPostRequestOneOf1AdditionalInfoOneOf2(models::ExternalWalletsWalletIdAssetIdPostRequestOneOf1AdditionalInfoOneOf2),
}

impl Default for ExternalWalletsWalletIdAssetIdPostRequestOneOf1AdditionalInfo {
    fn default() -> Self {
        Self::ExternalWalletsWalletIdAssetIdPostRequestOneOf1AdditionalInfoOneOf(Default::default())
    }
}

