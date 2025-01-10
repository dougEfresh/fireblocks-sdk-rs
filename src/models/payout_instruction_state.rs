// Fireblocks API
//
// Fireblocks provides a suite of applications to manage digital asset operations and a complete development platform to build your business on the blockchain.  - Visit our website for more information: [Fireblocks Website](https://fireblocks.com) - Visit our developer docs: [Fireblocks DevPortal](https://developers.fireblocks.com)
//
// The version of the OpenAPI document: 1.8.0
// Contact: developers@fireblocks.com
// Generated by: https://openapi-generator.tech

use serde::{Deserialize, Serialize};

/// PayoutInstructionState : - NOT_STARTED - waiting to start - TRANSACTION_SENT
/// - an underlying transaction was sent - COMPLETED - completed successfully -
/// FAILED - failed - TRANSLATION_ERROR -lookup of the destination failed (due
/// to changes in the underlying whitelisted external wallet or similar) -
/// SKIPPED- no transaction(s) created for this instruction
/// - NOT_STARTED - waiting to start - TRANSACTION_SENT - an underlying
///   transaction was sent - COMPLETED - completed successfully - FAILED -
///   failed - TRANSLATION_ERROR -lookup of the destination failed (due to
///   changes in the underlying whitelisted external wallet or similar) -
///   SKIPPED- no transaction(s) created for this instruction
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PayoutInstructionState {
    #[serde(rename = "NOT_STARTED")]
    NotStarted,
    #[serde(rename = "TRANSACTION_SENT")]
    TransactionSent,
    #[serde(rename = "COMPLETED")]
    Completed,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "TRANSLATION_ERROR")]
    TranslationError,
    #[serde(rename = "SKIPPED")]
    Skipped,
}

impl std::fmt::Display for PayoutInstructionState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::NotStarted => write!(f, "NOT_STARTED"),
            Self::TransactionSent => write!(f, "TRANSACTION_SENT"),
            Self::Completed => write!(f, "COMPLETED"),
            Self::Failed => write!(f, "FAILED"),
            Self::TranslationError => write!(f, "TRANSLATION_ERROR"),
            Self::Skipped => write!(f, "SKIPPED"),
        }
    }
}

impl Default for PayoutInstructionState {
    fn default() -> PayoutInstructionState {
        Self::NotStarted
    }
}
