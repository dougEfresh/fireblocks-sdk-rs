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
pub struct XbSettlementConfigEditRequestBody {
    /// The name for the cross-border settlement configuration
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "steps")]
    pub steps: models::XbSettlementConfigStepsRecord,
    /// Slippage configuarion in basis points, the default value is 10% 
    #[serde(rename = "conversionSlippageBasisPoints", skip_serializing_if = "Option::is_none")]
    pub conversion_slippage_basis_points: Option<u32>,
}

impl XbSettlementConfigEditRequestBody {
    pub fn new(name: String, steps: models::XbSettlementConfigStepsRecord) -> XbSettlementConfigEditRequestBody {
        XbSettlementConfigEditRequestBody {
            name,
            steps,
            conversion_slippage_basis_points: None,
        }
    }
}

