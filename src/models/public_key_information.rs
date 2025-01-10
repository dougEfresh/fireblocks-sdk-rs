// Fireblocks API
//
// Fireblocks provides a suite of applications to manage digital asset operations and a complete development platform to build your business on the blockchain.  - Visit our website for more information: [Fireblocks Website](https://fireblocks.com) - Visit our developer docs: [Fireblocks DevPortal](https://developers.fireblocks.com)
//
// The version of the OpenAPI document: 1.8.0
// Contact: developers@fireblocks.com
// Generated by: https://openapi-generator.tech

use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublicKeyInformation {
    /// Elliptic Curve
    #[serde(rename = "algorithm", skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<Algorithm>,
    /// BIP44 derivation path
    #[serde(rename = "derivationPath", skip_serializing_if = "Option::is_none")]
    pub derivation_path: Option<Vec<i32>>,
    /// Compressed/Uncompressed public key value in hex representation
    #[serde(rename = "publicKey", skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
}

impl PublicKeyInformation {
    pub fn new() -> PublicKeyInformation {
        PublicKeyInformation {
            algorithm: None,
            derivation_path: None,
            public_key: None,
        }
    }
}
/// Elliptic Curve
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Algorithm {
    #[serde(rename = "MPC_ECDSA_SECP256K1")]
    EcdsaSecp256K1,
    #[serde(rename = "MPC_ECDSA_SECP256R1")]
    EcdsaSecp256R1,
    #[serde(rename = "MPC_EDDSA_ED25519")]
    EddsaEd25519,
}

impl Default for Algorithm {
    fn default() -> Algorithm {
        Self::EcdsaSecp256K1
    }
}
