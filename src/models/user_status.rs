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

/// UserStatus : The status of the user
/// The status of the user
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UserStatus {
    #[serde(rename = "PENDING_ACTIVATION")]
    PendingActivation,
    #[serde(rename = "PENDING_DEVICE_PAIRING")]
    PendingDevicePairing,
    #[serde(rename = "PENDING_DEVICE_SETUP")]
    PendingDeviceSetup,
    #[serde(rename = "READY")]
    Ready,
}

impl std::fmt::Display for UserStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::PendingActivation => write!(f, "PENDING_ACTIVATION"),
            Self::PendingDevicePairing => write!(f, "PENDING_DEVICE_PAIRING"),
            Self::PendingDeviceSetup => write!(f, "PENDING_DEVICE_SETUP"),
            Self::Ready => write!(f, "READY"),
        }
    }
}

impl Default for UserStatus {
    fn default() -> UserStatus {
        Self::PendingActivation
    }
}
