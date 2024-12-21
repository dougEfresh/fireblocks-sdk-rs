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
pub struct InternalTransferResponse {
    /// Indicates whether the transfer was successful
    #[serde(rename = "success")]
    pub success: bool,
    /// The transaction ID of the internal transfer
    #[serde(
        rename = "id",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub id: Option<Option<String>>,
}

impl InternalTransferResponse {
    pub fn new(success: bool) -> InternalTransferResponse {
        InternalTransferResponse { success, id: None }
    }
}
