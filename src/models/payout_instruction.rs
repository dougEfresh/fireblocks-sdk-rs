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
pub struct PayoutInstruction {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "payeeAccount")]
    pub payee_account: models::PayeeAccount,
    #[serde(rename = "amount")]
    pub amount: models::InstructionAmount,
}

impl PayoutInstruction {
    pub fn new(payee_account: models::PayeeAccount, amount: models::InstructionAmount) -> PayoutInstruction {
        PayoutInstruction {
            id: None,
            payee_account,
            amount,
        }
    }
}

