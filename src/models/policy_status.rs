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

/// PolicyStatus : * SUCCESS - success * UNVALIDATED - not validated yet *
/// INVALID_CONFIGURATION - at least one rule is invalid * PENDING - pending
/// approval * PENDING_CONSOLE_APPROVAL - pending approval from the console app
/// * AWAITING_QUORUM - pending quorum approval * UNHANDLED_ERROR - unhandled
/// error
/// * SUCCESS - success * UNVALIDATED - not validated yet *
///   INVALID_CONFIGURATION - at least one rule is invalid * PENDING - pending
///   approval * PENDING_CONSOLE_APPROVAL - pending approval from the console
///   app * AWAITING_QUORUM - pending quorum approval * UNHANDLED_ERROR -
///   unhandled error
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PolicyStatus {
    #[serde(rename = "SUCCESS")]
    Success,
    #[serde(rename = "UNVALIDATED")]
    Unvalidated,
    #[serde(rename = "INVALID_CONFIGURATION")]
    InvalidConfiguration,
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "PENDING_CONSOLE_APPROVAL")]
    PendingConsoleApproval,
    #[serde(rename = "AWAITING_QUORUM")]
    AwaitingQuorum,
    #[serde(rename = "UNHANDLED_ERROR")]
    UnhandledError,
}

impl std::fmt::Display for PolicyStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Success => write!(f, "SUCCESS"),
            Self::Unvalidated => write!(f, "UNVALIDATED"),
            Self::InvalidConfiguration => write!(f, "INVALID_CONFIGURATION"),
            Self::Pending => write!(f, "PENDING"),
            Self::PendingConsoleApproval => write!(f, "PENDING_CONSOLE_APPROVAL"),
            Self::AwaitingQuorum => write!(f, "AWAITING_QUORUM"),
            Self::UnhandledError => write!(f, "UNHANDLED_ERROR"),
        }
    }
}

impl Default for PolicyStatus {
    fn default() -> PolicyStatus {
        Self::Success
    }
}
