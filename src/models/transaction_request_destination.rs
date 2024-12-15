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
pub struct TransactionRequestDestination {
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(rename = "destination", skip_serializing_if = "Option::is_none")]
    pub destination: Option<models::DestinationTransferPeerPath>,
}

impl TransactionRequestDestination {
    pub fn new() -> TransactionRequestDestination {
        TransactionRequestDestination {
            amount: None,
            destination: None,
        }
    }
}

