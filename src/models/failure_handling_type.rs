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

/// FailureHandlingType : Type of failure handling mechanism
/// Type of failure handling mechanism
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FailureHandlingType {
    #[serde(rename = "REVERSE")]
    Reverse,

}

impl std::fmt::Display for FailureHandlingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Reverse => write!(f, "REVERSE"),
        }
    }
}

impl Default for FailureHandlingType {
    fn default() -> FailureHandlingType {
        Self::Reverse
    }
}

