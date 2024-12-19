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

/// DisbursementOperationType : Disbursment operation type
/// Disbursment operation type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DisbursementOperationType {
    #[serde(rename = "DISBURSEMENT")]
    Disbursement,

}

impl std::fmt::Display for DisbursementOperationType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Disbursement => write!(f, "DISBURSEMENT"),
        }
    }
}

impl Default for DisbursementOperationType {
    fn default() -> DisbursementOperationType {
        Self::Disbursement
    }
}

