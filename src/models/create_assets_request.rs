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
pub struct CreateAssetsRequest {
    /// Optional - when creating an EOS wallet, the account name. If not
    /// provided, a random name will be generated
    #[serde(rename = "eosAccountName", skip_serializing_if = "Option::is_none")]
    pub eos_account_name: Option<String>,
}

impl CreateAssetsRequest {
    pub fn new() -> CreateAssetsRequest {
        CreateAssetsRequest {
            eos_account_name: None,
        }
    }
}
