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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConfigOperationSnapshot {
    ConfigConversionOperationSnapshot(models::ConfigConversionOperationSnapshot),
    ConfigTransferOperationSnapshot(models::ConfigTransferOperationSnapshot),
    ConfigDisbursementOperationSnapshot(models::ConfigDisbursementOperationSnapshot),
}

impl Default for ConfigOperationSnapshot {
    fn default() -> Self {
        Self::ConfigConversionOperationSnapshot(Default::default())
    }
}

