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

/// PolicyRuleRawMessageSigning : Raw message signing configuration
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicyRuleRawMessageSigning {
    #[serde(rename = "algorithm", skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,
    #[serde(rename = "derivationPath", skip_serializing_if = "Option::is_none")]
    pub derivation_path: Option<models::PolicyRuleRawMessageSigningDerivationPath>,
}

impl PolicyRuleRawMessageSigning {
    /// Raw message signing configuration
    pub fn new() -> PolicyRuleRawMessageSigning {
        PolicyRuleRawMessageSigning {
            algorithm: None,
            derivation_path: None,
        }
    }
}

