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
pub struct DisbursementAmountInstruction {
    #[serde(rename = "payeeAccount")]
    pub payee_account: models::Destination,
    /// Asset unique identifier
    #[serde(rename = "assetId")]
    pub asset_id: String,
    /// The amount to disburse
    #[serde(rename = "amount")]
    pub amount: String,
}

impl DisbursementAmountInstruction {
    pub fn new(payee_account: models::Destination, asset_id: String, amount: String) -> DisbursementAmountInstruction {
        DisbursementAmountInstruction {
            payee_account,
            asset_id,
            amount,
        }
    }
}

