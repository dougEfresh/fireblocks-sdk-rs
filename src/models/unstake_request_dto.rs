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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UnstakeRequestDto {
    /// id of position to unstake
    #[serde(rename = "id")]
    pub id: String,
    /// Represents the fee for a transaction, which can be specified as a percentage value. Only one of fee/feeLevel is required.
    #[serde(rename = "fee", skip_serializing_if = "Option::is_none")]
    pub fee: Option<String>,
    /// Represents the fee level for a transaction, which can be set as slow, medium, or fast. Only one of fee/feeLevel is required.
    #[serde(rename = "feeLevel", skip_serializing_if = "Option::is_none")]
    pub fee_level: Option<String>,
    /// The note to associate with the transactions.
    #[serde(rename = "txNote", skip_serializing_if = "Option::is_none")]
    pub tx_note: Option<String>,
}

impl UnstakeRequestDto {
    pub fn new(id: String) -> UnstakeRequestDto {
        UnstakeRequestDto {
            id,
            fee: None,
            fee_level: None,
            tx_note: None,
        }
    }
}

