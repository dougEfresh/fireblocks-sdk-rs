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
pub struct DisbursementOperationInput {
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(rename = "paymentAccount")]
    pub payment_account: models::Account,
    #[serde(rename = "instructionSet")]
    pub instruction_set: Vec<models::DisbursementInstruction>,
}

impl DisbursementOperationInput {
    pub fn new(payment_account: models::Account, instruction_set: Vec<models::DisbursementInstruction>) -> DisbursementOperationInput {
        DisbursementOperationInput {
            amount: None,
            payment_account,
            instruction_set,
        }
    }
}

