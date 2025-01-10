// Fireblocks API
//
// Fireblocks provides a suite of applications to manage digital asset operations and a complete development platform to build your business on the blockchain.  - Visit our website for more information: [Fireblocks Website](https://fireblocks.com) - Visit our developer docs: [Fireblocks DevPortal](https://developers.fireblocks.com)
//
// The version of the OpenAPI document: 1.8.0
// Contact: developers@fireblocks.com
// Generated by: https://openapi-generator.tech

use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TravelRuleAddress {
    /// Street address
    #[serde(rename = "street")]
    pub street: String,
    /// City
    #[serde(rename = "city")]
    pub city: String,
    /// State or province
    #[serde(rename = "state")]
    pub state: String,
    /// Postal or ZIP code
    #[serde(rename = "postalCode")]
    pub postal_code: String,
}

impl TravelRuleAddress {
    pub fn new(
        street: String,
        city: String,
        state: String,
        postal_code: String,
    ) -> TravelRuleAddress {
        TravelRuleAddress {
            street,
            city,
            state,
            postal_code,
        }
    }
}
