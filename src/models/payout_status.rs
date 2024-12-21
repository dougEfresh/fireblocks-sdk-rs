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

/// PayoutStatus : - REQUESTED payout requested with all its details - VERIFIED
/// payout instruction set details were verified - PROCESSING payout instruction
/// set executed and is processing - FINALIZED payout done (all payout
/// instructions completed successfully) - INSUFFICIENT_BALANCE insufficient
/// balance in the payment account (can be a temporary state) - FAILED one or
/// more of the payout instructions failed
/// - REQUESTED payout requested with all its details - VERIFIED payout
///   instruction set details were verified - PROCESSING payout instruction set
///   executed and is processing - FINALIZED payout done (all payout
///   instructions completed successfully) - INSUFFICIENT_BALANCE insufficient
///   balance in the payment account (can be a temporary state) - FAILED one or
///   more of the payout instructions failed
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PayoutStatus {
    #[serde(rename = "REGISTERED")]
    Registered,
    #[serde(rename = "VERIFYING")]
    Verifying,
    #[serde(rename = "IN_PROGRESS")]
    InProgress,
    #[serde(rename = "DONE")]
    Done,
    #[serde(rename = "INSUFFICIENT_BALANCE")]
    InsufficientBalance,
    #[serde(rename = "FAILED")]
    Failed,
}

impl std::fmt::Display for PayoutStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Registered => write!(f, "REGISTERED"),
            Self::Verifying => write!(f, "VERIFYING"),
            Self::InProgress => write!(f, "IN_PROGRESS"),
            Self::Done => write!(f, "DONE"),
            Self::InsufficientBalance => write!(f, "INSUFFICIENT_BALANCE"),
            Self::Failed => write!(f, "FAILED"),
        }
    }
}

impl Default for PayoutStatus {
    fn default() -> PayoutStatus {
        Self::Registered
    }
}
