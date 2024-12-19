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

/// CreateSigningKeyDtoProofOfOwnership : An object containing proof of ownership for the signing key.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateSigningKeyDtoProofOfOwnership {
    /// The message to be signed by the key as proof of ownership. 64 to 1024 bytes in hexadecimal format.
    #[serde(rename = "message")]
    pub message: String,
    /// The signature of the message. 64 bytes in hexadecimal format.
    #[serde(rename = "signature")]
    pub signature: String,
}

impl CreateSigningKeyDtoProofOfOwnership {
    /// An object containing proof of ownership for the signing key.
    pub fn new(message: String, signature: String) -> CreateSigningKeyDtoProofOfOwnership {
        CreateSigningKeyDtoProofOfOwnership {
            message,
            signature,
        }
    }
}

