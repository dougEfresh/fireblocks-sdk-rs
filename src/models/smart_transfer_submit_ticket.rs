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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmartTransferSubmitTicket {
    /// Sets the ticket expiration time (in hours) after the ticket is submitted. If no funding source is set for any term, the ticket will automatically expire after given time. If expiresIn is not sent ticket will not expire.
    #[serde(rename = "expiresIn", skip_serializing_if = "Option::is_none")]
    pub expires_in: Option<f64>,
}

impl SmartTransferSubmitTicket {
    pub fn new() -> SmartTransferSubmitTicket {
        SmartTransferSubmitTicket {
            expires_in: None,
        }
    }
}

