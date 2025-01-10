// Fireblocks API
//
// Fireblocks provides a suite of applications to manage digital asset operations and a complete development platform to build your business on the blockchain.  - Visit our website for more information: [Fireblocks Website](https://fireblocks.com) - Visit our developer docs: [Fireblocks DevPortal](https://developers.fireblocks.com)
//
// The version of the OpenAPI document: 1.8.0
// Contact: developers@fireblocks.com
// Generated by: https://openapi-generator.tech

use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StellarRippleCreateParamsDto {
    /// The symbol of the token
    #[serde(rename = "symbol")]
    pub symbol: String,
    /// The name of the token
    #[serde(rename = "name")]
    pub name: String,
    /// The address of the issuer of this token. Will be part of the identifier
    /// of this token on chain.
    #[serde(rename = "issuerAddress")]
    pub issuer_address: String,
}

impl StellarRippleCreateParamsDto {
    pub fn new(
        symbol: String,
        name: String,
        issuer_address: String,
    ) -> StellarRippleCreateParamsDto {
        StellarRippleCreateParamsDto {
            symbol,
            name,
            issuer_address,
        }
    }
}