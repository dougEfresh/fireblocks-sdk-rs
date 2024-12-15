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
pub struct TravelRuleOwnershipProof {
    /// Type of ownership proof
    #[serde(rename = "type")]
    pub r#type: String,
    /// Identification number
    #[serde(rename = "id")]
    pub id: String,
    /// Name of owner
    #[serde(rename = "name")]
    pub name: String,
    /// Country of issuance
    #[serde(rename = "country")]
    pub country: String,
    /// Date of issuance
    #[serde(rename = "issueDate")]
    pub issue_date: String,
    /// Name of issuing entity
    #[serde(rename = "issuer")]
    pub issuer: String,
}

impl TravelRuleOwnershipProof {
    pub fn new(r#type: String, id: String, name: String, country: String, issue_date: String, issuer: String) -> TravelRuleOwnershipProof {
        TravelRuleOwnershipProof {
            r#type,
            id,
            name,
            country,
            issue_date,
            issuer,
        }
    }
}

