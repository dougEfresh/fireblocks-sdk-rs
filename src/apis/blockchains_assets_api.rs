/*
 * Fireblocks API
 *
 * Fireblocks provides a suite of applications to manage digital asset operations and a complete development platform to build your business on the blockchain.  - Visit our website for more information: [Fireblocks Website](https://fireblocks.com) - Visit our developer docs: [Fireblocks DevPortal](https://developers.fireblocks.com) 
 *
 * The version of the OpenAPI document: 1.8.0
 * Contact: developers@fireblocks.com
 * Generated by: https://openapi-generator.tech
 */


use async_trait::async_trait;
use reqwest;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};

#[async_trait]
pub trait BlockchainsAssetsApi: Send + Sync {
    async fn estimate_network_fee(&self,  params: EstimateNetworkFeeParams ) -> Result<models::EstimatedNetworkFeeResponse, Error<EstimateNetworkFeeError>>;
    async fn get_supported_assets(&self, ) -> Result<Vec<models::AssetTypeResponse>, Error<GetSupportedAssetsError>>;
    async fn register_new_asset(&self,  params: RegisterNewAssetParams ) -> Result<models::AssetResponse, Error<RegisterNewAssetError>>;
    async fn validate_address(&self,  params: ValidateAddressParams ) -> Result<models::ValidateAddressResponse, Error<ValidateAddressError>>;
}

pub struct BlockchainsAssetsApiClient {
    configuration: Arc<configuration::Configuration>
}

impl BlockchainsAssetsApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self { configuration }
    }
}


/// struct for passing parameters to the method [`estimate_network_fee`]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct EstimateNetworkFeeParams {
    /// The asset for which to estimate the fee
    pub asset_id: String
}

/// struct for passing parameters to the method [`register_new_asset`]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct RegisterNewAssetParams {
    /// A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours.
    pub idempotency_key: Option<String>,
    pub register_new_asset_request: Option<models::RegisterNewAssetRequest>
}

/// struct for passing parameters to the method [`validate_address`]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct ValidateAddressParams {
    /// The asset of the address
    pub asset_id: String,
    /// The address to validate
    pub address: String
}


#[async_trait]
impl BlockchainsAssetsApi for BlockchainsAssetsApiClient {
    /// Gets the estimated required fee for an asset. Fireblocks fetches, calculates and caches the result every 30 seconds. Customers should query this API while taking the caching interval into consideration.  - For UTXO based assets, the response will contain the suggested fee per byte - For ETH (and all EVM) based assets, the suggested gas price - For XRP/XLM, the transaction fee 
    async fn estimate_network_fee(&self,  params: EstimateNetworkFeeParams ) -> Result<models::EstimatedNetworkFeeResponse, Error<EstimateNetworkFeeError>> {
        
        let EstimateNetworkFeeParams {
            asset_id,
        } = params;
        

        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/estimate_network_fee", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        local_var_req_builder = local_var_req_builder.query(&[("assetId", &asset_id.to_string())]);
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
            let local_var_entity: Option<EstimateNetworkFeeError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Returns all asset types supported by Fireblocks.   The response includes all assets supported by Fireblocks globally in addition to assets added to the specific workspace. 
    async fn get_supported_assets(&self, ) -> Result<Vec<models::AssetTypeResponse>, Error<GetSupportedAssetsError>> {
        

        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/supported_assets", local_var_configuration.base_path);
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
            let local_var_entity: Option<GetSupportedAssetsError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Register a new asset to a workspace and return the newly created asset's details. Currently supported chains are: - EVM based chains - Stellar - Algorand - TRON - NEAR - Solana 
    async fn register_new_asset(&self,  params: RegisterNewAssetParams ) -> Result<models::AssetResponse, Error<RegisterNewAssetError>> {
        
        let RegisterNewAssetParams {
            idempotency_key,
            register_new_asset_request,
        } = params;
        

        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/assets", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(local_var_param_value) = idempotency_key {
            local_var_req_builder = local_var_req_builder.header("Idempotency-Key", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.json(&register_new_asset_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<RegisterNewAssetError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Checks if an address is valid and active (for XRP, DOT, XLM, and EOS).
    async fn validate_address(&self,  params: ValidateAddressParams ) -> Result<models::ValidateAddressResponse, Error<ValidateAddressError>> {
        
        let ValidateAddressParams {
            asset_id,
            address,
        } = params;
        

        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/transactions/validate_address/{assetId}/{address}", local_var_configuration.base_path, assetId=crate::apis::urlencode(asset_id), address=crate::apis::urlencode(address));
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
            let local_var_entity: Option<ValidateAddressError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

}

/// struct for typed errors of method [`estimate_network_fee`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EstimateNetworkFeeError {
    DefaultResponse(models::ErrorSchema),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_supported_assets`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSupportedAssetsError {
    DefaultResponse(models::ErrorSchema),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`register_new_asset`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RegisterNewAssetError {
    Status400(models::AssetBadRequestErrorResponse),
    Status403(models::AssetForbiddenErrorResponse),
    Status404(models::AssetNotFoundErrorResponse),
    Status409(models::AssetConflictErrorResponse),
    Status500(models::AssetInternalServerErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`validate_address`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ValidateAddressError {
    DefaultResponse(models::ErrorSchema),
    UnknownValue(serde_json::Value),
}

