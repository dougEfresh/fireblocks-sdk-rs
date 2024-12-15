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
pub struct GasStationConfiguration {
    #[serde(rename = "gasThreshold", skip_serializing_if = "Option::is_none")]
    pub gas_threshold: Option<String>,
    #[serde(rename = "gasCap", skip_serializing_if = "Option::is_none")]
    pub gas_cap: Option<String>,
    #[serde(rename = "maxGasPrice", skip_serializing_if = "Option::is_none")]
    pub max_gas_price: Option<String>,
}

impl GasStationConfiguration {
    pub fn new() -> GasStationConfiguration {
        GasStationConfiguration {
            gas_threshold: None,
            gas_cap: None,
            max_gas_price: None,
        }
    }
}

