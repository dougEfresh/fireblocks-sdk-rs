/*
 * Fireblocks API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.7.5
 * Contact: support@fireblocks.com
 * Generated by: https://openapi-generator.tech
 */


use async_trait::async_trait;
use reqwest;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};

#[async_trait]
pub trait GasStationsApi: Send + Sync {
    async fn gas_station_asset_id_get(&self,  params: GasStationAssetIdGetParams ) -> Result<models::GasStationPropertiesResponse, Error<GasStationAssetIdGetError>>;
    async fn gas_station_configuration_asset_id_put(&self,  params: GasStationConfigurationAssetIdPutParams ) -> Result<(), Error<GasStationConfigurationAssetIdPutError>>;
    async fn gas_station_configuration_put(&self,  params: GasStationConfigurationPutParams ) -> Result<(), Error<GasStationConfigurationPutError>>;
    async fn gas_station_get(&self, ) -> Result<models::GasStationPropertiesResponse, Error<GasStationGetError>>;
}

pub struct GasStationsApiClient {
    configuration: Arc<configuration::Configuration>
}

impl GasStationsApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self { configuration }
    }
}


/// struct for passing parameters to the method [`gas_station_asset_id_get`]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct GasStationAssetIdGetParams {
    /// The ID of the asset
    pub asset_id: String
}

/// struct for passing parameters to the method [`gas_station_configuration_asset_id_put`]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct GasStationConfigurationAssetIdPutParams {
    /// The ID of the asset
    pub asset_id: String,
    pub gas_station_configuration: models::GasStationConfiguration
}

/// struct for passing parameters to the method [`gas_station_configuration_put`]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct GasStationConfigurationPutParams {
    pub gas_station_configuration: models::GasStationConfiguration
}


#[async_trait]
impl GasStationsApi for GasStationsApiClient {
    /// Returns gas station settings and balances for a requested asset.
    async fn gas_station_asset_id_get(&self,  params: GasStationAssetIdGetParams ) -> Result<models::GasStationPropertiesResponse, Error<GasStationAssetIdGetError>> {
        
        let GasStationAssetIdGetParams {
            asset_id,
        } = params;
        

        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/gas_station/{assetId}", local_var_configuration.base_path, assetId=crate::apis::urlencode(asset_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<GasStationAssetIdGetError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Configures gas station settings for a requested asset.
    async fn gas_station_configuration_asset_id_put(&self,  params: GasStationConfigurationAssetIdPutParams ) -> Result<(), Error<GasStationConfigurationAssetIdPutError>> {
        
        let GasStationConfigurationAssetIdPutParams {
            asset_id,
            gas_station_configuration,
        } = params;
        

        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/gas_station/configuration/{assetId}", local_var_configuration.base_path, assetId=crate::apis::urlencode(asset_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        local_var_req_builder = local_var_req_builder.json(&gas_station_configuration);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            Ok(())
        } else {
            let local_var_entity: Option<GasStationConfigurationAssetIdPutError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Configures gas station settings for ETH.
    async fn gas_station_configuration_put(&self,  params: GasStationConfigurationPutParams ) -> Result<(), Error<GasStationConfigurationPutError>> {
        
        let GasStationConfigurationPutParams {
            gas_station_configuration,
        } = params;
        

        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/gas_station/configuration", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        local_var_req_builder = local_var_req_builder.json(&gas_station_configuration);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            Ok(())
        } else {
            let local_var_entity: Option<GasStationConfigurationPutError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Returns gas station settings and ETH balance.
    async fn gas_station_get(&self, ) -> Result<models::GasStationPropertiesResponse, Error<GasStationGetError>> {
        

        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/gas_station", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<GasStationGetError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

}

/// struct for typed errors of method [`gas_station_asset_id_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GasStationAssetIdGetError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`gas_station_configuration_asset_id_put`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GasStationConfigurationAssetIdPutError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`gas_station_configuration_put`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GasStationConfigurationPutError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`gas_station_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GasStationGetError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

