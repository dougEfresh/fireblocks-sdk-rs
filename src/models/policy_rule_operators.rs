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

/// PolicyRuleOperators : Defines users/groups who can initiate the type of transaction to which the rule applies.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicyRuleOperators {
    /// If used then this property should appear as the only child property * \"*\" - All users are allowed 
    #[serde(rename = "wildcard", skip_serializing_if = "Option::is_none")]
    pub wildcard: Option<Wildcard>,
    /// Set of users ids
    #[serde(rename = "users", skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<String>>,
    /// Set of group ids
    #[serde(rename = "usersGroups", skip_serializing_if = "Option::is_none")]
    pub users_groups: Option<Vec<String>>,
    /// set of services to initiate transactions
    #[serde(rename = "services", skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<String>>,
}

impl PolicyRuleOperators {
    /// Defines users/groups who can initiate the type of transaction to which the rule applies.
    pub fn new() -> PolicyRuleOperators {
        PolicyRuleOperators {
            wildcard: None,
            users: None,
            users_groups: None,
            services: None,
        }
    }
}
/// If used then this property should appear as the only child property * \"*\" - All users are allowed 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Wildcard {
    #[serde(rename = "*")]
    Star,
}

impl Default for Wildcard {
    fn default() -> Wildcard {
        Self::Star
    }
}

