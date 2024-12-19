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
pub struct InstructionAmount {
    /// The amount as string
    #[serde(rename = "amount")]
    pub amount: String,
    /// Unique asset identifier
    #[serde(rename = "assetId")]
    pub asset_id: String,
}

impl InstructionAmount {
    pub fn new(amount: String, asset_id: String) -> InstructionAmount {
        InstructionAmount {
            amount,
            asset_id,
        }
    }
}

