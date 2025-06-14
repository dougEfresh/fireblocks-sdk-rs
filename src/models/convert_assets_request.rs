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
pub struct ConvertAssetsRequest {
    /// Name of the source asset (must be in a currency that is supported for
    /// conversions in the selected exchange type that corresponds to your
    /// exchange ID)
    #[serde(rename = "srcAsset")]
    pub src_asset: String,
    /// Name of the destination asset (must be in a currency that is supported
    /// for conversions in the selected exchange type that corresponds to your
    /// exchange ID)
    #[serde(rename = "destAsset")]
    pub dest_asset: String,
    /// The amount to transfer (in the currency of the source asset)
    #[serde(rename = "amount")]
    pub amount: f64,
}

impl ConvertAssetsRequest {
    pub fn new(src_asset: String, dest_asset: String, amount: f64) -> ConvertAssetsRequest {
        ConvertAssetsRequest {
            src_asset,
            dest_asset,
            amount,
        }
    }
}
