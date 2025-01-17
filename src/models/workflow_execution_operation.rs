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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WorkflowExecutionOperation {
    ExecutionScreeningOperation(models::ExecutionScreeningOperation),
    ExecutionConversionOperation(models::ExecutionConversionOperation),
    ExecutionTransferOperation(models::ExecutionTransferOperation),
    ExecutionDisbursementOperation(models::ExecutionDisbursementOperation),
}

impl Default for WorkflowExecutionOperation {
    fn default() -> Self {
        Self::ExecutionScreeningOperation(Default::default())
    }
}
