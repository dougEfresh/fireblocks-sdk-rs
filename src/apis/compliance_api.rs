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
pub trait ComplianceApi: Send + Sync {
    async fn get_aml_post_screening_policy(&self, ) -> Result<models::ScreeningPolicyResponse, Error<GetAmlPostScreeningPolicyError>>;
    async fn get_aml_screening_configuration(&self, ) -> Result<models::ScreeningConfigurationsRequest, Error<GetAmlScreeningConfigurationError>>;
    async fn get_aml_screening_policy(&self, ) -> Result<models::ScreeningProviderRulesConfigurationResponse, Error<GetAmlScreeningPolicyError>>;
    async fn get_post_screening_policy(&self, ) -> Result<models::ScreeningPolicyResponse, Error<GetPostScreeningPolicyError>>;
    async fn get_screening_configuration(&self, ) -> Result<models::ScreeningConfigurationsRequest, Error<GetScreeningConfigurationError>>;
    async fn get_screening_policy(&self, ) -> Result<models::ScreeningProviderRulesConfigurationResponse, Error<GetScreeningPolicyError>>;
    async fn get_vaspby_did(&self,  params: GetVaspbyDidParams ) -> Result<models::TravelRuleVasp, Error<GetVaspbyDidError>>;
    async fn get_vasps(&self,  params: GetVaspsParams ) -> Result<models::TravelRuleGetAllVaspsResponse, Error<GetVaspsError>>;
    async fn update_aml_screening_configuration(&self,  params: UpdateAmlScreeningConfigurationParams ) -> Result<models::ScreeningConfigurationsRequest, Error<UpdateAmlScreeningConfigurationError>>;
    async fn update_screening_configuration(&self,  params: UpdateScreeningConfigurationParams ) -> Result<models::ScreeningUpdateConfigurationsRequest, Error<UpdateScreeningConfigurationError>>;
    async fn update_travel_rule_config(&self,  params: UpdateTravelRuleConfigParams ) -> Result<models::ScreeningConfigurationsRequest, Error<UpdateTravelRuleConfigError>>;
    async fn update_vasp(&self,  params: UpdateVaspParams ) -> Result<models::TravelRuleUpdateVaspDetails, Error<UpdateVaspError>>;
    async fn validate_full_travel_rule_transaction(&self,  params: ValidateFullTravelRuleTransactionParams ) -> Result<models::TravelRuleValidateTransactionResponse, Error<ValidateFullTravelRuleTransactionError>>;
    async fn validate_travel_rule_transaction(&self,  params: ValidateTravelRuleTransactionParams ) -> Result<models::TravelRuleValidateTransactionResponse, Error<ValidateTravelRuleTransactionError>>;
}

pub struct ComplianceApiClient {
    configuration: Arc<configuration::Configuration>
}

impl ComplianceApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self { configuration }
    }
}


/// struct for passing parameters to the method [`get_vaspby_did`]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct GetVaspbyDidParams {
    pub did: String,
    /// CSV of fields to return (all, \"blank\" or see list of all field names below)
    pub fields: Option<String>
}

/// struct for passing parameters to the method [`get_vasps`]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct GetVaspsParams {
    /// Field to order by
    pub order: Option<String>,
    /// Records per page
    pub per_page: Option<f64>,
    /// Page number
    pub page: Option<f64>,
    /// CSV of fields to return (all, \"blank\" or see list of all field names below)
    pub fields: Option<String>
}

/// struct for passing parameters to the method [`update_aml_screening_configuration`]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct UpdateAmlScreeningConfigurationParams {
    /// A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours.
    pub idempotency_key: Option<String>
}

/// struct for passing parameters to the method [`update_screening_configuration`]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct UpdateScreeningConfigurationParams {
    /// A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours.
    pub idempotency_key: Option<String>
}

/// struct for passing parameters to the method [`update_travel_rule_config`]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct UpdateTravelRuleConfigParams {
    /// A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours.
    pub idempotency_key: Option<String>
}

/// struct for passing parameters to the method [`update_vasp`]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct UpdateVaspParams {
    pub travel_rule_update_vasp_details: models::TravelRuleUpdateVaspDetails,
    /// A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours.
    pub idempotency_key: Option<String>
}

/// struct for passing parameters to the method [`validate_full_travel_rule_transaction`]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct ValidateFullTravelRuleTransactionParams {
    pub travel_rule_validate_full_transaction_request: models::TravelRuleValidateFullTransactionRequest,
    /// A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours.
    pub idempotency_key: Option<String>
}

/// struct for passing parameters to the method [`validate_travel_rule_transaction`]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct ValidateTravelRuleTransactionParams {
    pub travel_rule_validate_transaction_request: models::TravelRuleValidateTransactionRequest,
    /// A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours.
    pub idempotency_key: Option<String>
}


#[async_trait]
impl ComplianceApi for ComplianceApiClient {
    /// Get the post-screening policy for AML.
    async fn get_aml_post_screening_policy(&self, ) -> Result<models::ScreeningPolicyResponse, Error<GetAmlPostScreeningPolicyError>> {
        

        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/screening/aml/post_screening_policy", local_var_configuration.base_path);
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
            let local_var_entity: Option<GetAmlPostScreeningPolicyError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Retrieves the configuration for Travel Rule screening policy.
    async fn get_aml_screening_configuration(&self, ) -> Result<models::ScreeningConfigurationsRequest, Error<GetAmlScreeningConfigurationError>> {
        

        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/screening/aml/policy_configuration", local_var_configuration.base_path);
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
            let local_var_entity: Option<GetAmlScreeningConfigurationError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Get the screening policy for AML.
    async fn get_aml_screening_policy(&self, ) -> Result<models::ScreeningProviderRulesConfigurationResponse, Error<GetAmlScreeningPolicyError>> {
        

        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/screening/aml/screening_policy", local_var_configuration.base_path);
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
            let local_var_entity: Option<GetAmlScreeningPolicyError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Get the post-screening policy for Travel Rule.
    async fn get_post_screening_policy(&self, ) -> Result<models::ScreeningPolicyResponse, Error<GetPostScreeningPolicyError>> {
        

        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/screening/travel_rule/post_screening_policy", local_var_configuration.base_path);
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
            let local_var_entity: Option<GetPostScreeningPolicyError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Retrieves the configuration for Travel Rule screening policy.
    async fn get_screening_configuration(&self, ) -> Result<models::ScreeningConfigurationsRequest, Error<GetScreeningConfigurationError>> {
        

        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/screening/travel_rule/policy_configuration", local_var_configuration.base_path);
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
            let local_var_entity: Option<GetScreeningConfigurationError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Get the screening policy for Travel Rule.
    async fn get_screening_policy(&self, ) -> Result<models::ScreeningProviderRulesConfigurationResponse, Error<GetScreeningPolicyError>> {
        

        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/screening/travel_rule/screening_policy", local_var_configuration.base_path);
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
            let local_var_entity: Option<GetScreeningPolicyError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Get VASP Details.  Returns information about a VASP that has the specified DID.  **Note:** The reference content in this section documents the Travel Rule beta endpoint. The beta endpoint includes APIs that are currently in preview and aren't yet generally available.  To enroll in the beta and enable this endpoint, contact your Fireblocks Customer Success Manager or send an email to [CSM@fireblocks.com](mailto:CSM@fireblocks.com).
    async fn get_vaspby_did(&self,  params: GetVaspbyDidParams ) -> Result<models::TravelRuleVasp, Error<GetVaspbyDidError>> {
        
        let GetVaspbyDidParams {
            did,
            fields,
        } = params;
        

        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/screening/travel_rule/vasp/{did}", local_var_configuration.base_path, did=crate::apis::urlencode(did));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_str) = fields {
            local_var_req_builder = local_var_req_builder.query(&[("fields", &local_var_str.to_string())]);
        }
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
            let local_var_entity: Option<GetVaspbyDidError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Get All VASPs.  Returns a list of VASPs. VASPs can be searched and sorted and results are paginated.  **Note:** The reference content in this section documents the Travel Rule beta endpoint. The beta endpoint includes APIs that are currently in preview and aren't yet generally available.  To enroll in the beta and enable this endpoint, contact your Fireblocks Customer Success Manager or send an email to [CSM@fireblocks.com](mailto:CSM@fireblocks.com).
    async fn get_vasps(&self,  params: GetVaspsParams ) -> Result<models::TravelRuleGetAllVaspsResponse, Error<GetVaspsError>> {
        
        let GetVaspsParams {
            order,
            per_page,
            page,
            fields,
        } = params;
        

        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/screening/travel_rule/vasp", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_str) = order {
            local_var_req_builder = local_var_req_builder.query(&[("order", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = per_page {
            local_var_req_builder = local_var_req_builder.query(&[("per_page", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = page {
            local_var_req_builder = local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = fields {
            local_var_req_builder = local_var_req_builder.query(&[("fields", &local_var_str.to_string())]);
        }
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
            let local_var_entity: Option<GetVaspsError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Updates bypass screening, inbound delay, or outbound delay configurations for AML.
    async fn update_aml_screening_configuration(&self,  params: UpdateAmlScreeningConfigurationParams ) -> Result<models::ScreeningConfigurationsRequest, Error<UpdateAmlScreeningConfigurationError>> {
        
        let UpdateAmlScreeningConfigurationParams {
            idempotency_key,
        } = params;
        

        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/screening/aml/policy_configuration", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(local_var_param_value) = idempotency_key {
            local_var_req_builder = local_var_req_builder.header("Idempotency-Key", local_var_param_value.to_string());
        }

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<UpdateAmlScreeningConfigurationError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Update Workspace screening configuration.
    async fn update_screening_configuration(&self,  params: UpdateScreeningConfigurationParams ) -> Result<models::ScreeningUpdateConfigurationsRequest, Error<UpdateScreeningConfigurationError>> {
        
        let UpdateScreeningConfigurationParams {
            idempotency_key,
        } = params;
        

        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/screening/configurations", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(local_var_param_value) = idempotency_key {
            local_var_req_builder = local_var_req_builder.header("Idempotency-Key", local_var_param_value.to_string());
        }

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<UpdateScreeningConfigurationError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Updates bypass screening, inbound delay, or outbound delay configurations for Travel Rule.
    async fn update_travel_rule_config(&self,  params: UpdateTravelRuleConfigParams ) -> Result<models::ScreeningConfigurationsRequest, Error<UpdateTravelRuleConfigError>> {
        
        let UpdateTravelRuleConfigParams {
            idempotency_key,
        } = params;
        

        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/screening/travel_rule/policy_configuration", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(local_var_param_value) = idempotency_key {
            local_var_req_builder = local_var_req_builder.header("Idempotency-Key", local_var_param_value.to_string());
        }

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<UpdateTravelRuleConfigError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Update VASP Details.  Updates a VASP with the provided parameters. Use this endpoint to add your public jsonDIDkey generated by Notabene.  **Note:** The reference content in this section documents the Travel Rule beta endpoint. The beta endpoint includes APIs that are currently in preview and aren't yet generally available.  To enroll in the beta and enable this endpoint, contact your Fireblocks Customer Success Manager or send an email to [CSM@fireblocks.com](mailto:CSM@fireblocks.com).
    async fn update_vasp(&self,  params: UpdateVaspParams ) -> Result<models::TravelRuleUpdateVaspDetails, Error<UpdateVaspError>> {
        
        let UpdateVaspParams {
            travel_rule_update_vasp_details,
            idempotency_key,
        } = params;
        

        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/screening/travel_rule/vasp/update", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(local_var_param_value) = idempotency_key {
            local_var_req_builder = local_var_req_builder.header("Idempotency-Key", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.json(&travel_rule_update_vasp_details);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<UpdateVaspError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Validate Full Travel Rule transactions.  Checks for all required information on the originator and beneficiary VASPs.  **Note:** The reference content in this section documents the Travel Rule beta endpoint. The beta endpoint includes APIs that are currently in preview and aren't yet generally available.  To enroll in the beta and enable this endpoint, contact your Fireblocks Customer Success Manager or send an email to [CSM@fireblocks.com](mailto:CSM@fireblocks.com).
    async fn validate_full_travel_rule_transaction(&self,  params: ValidateFullTravelRuleTransactionParams ) -> Result<models::TravelRuleValidateTransactionResponse, Error<ValidateFullTravelRuleTransactionError>> {
        
        let ValidateFullTravelRuleTransactionParams {
            travel_rule_validate_full_transaction_request,
            idempotency_key,
        } = params;
        

        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/screening/travel_rule/transaction/validate/full", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(local_var_param_value) = idempotency_key {
            local_var_req_builder = local_var_req_builder.header("Idempotency-Key", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.json(&travel_rule_validate_full_transaction_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<ValidateFullTravelRuleTransactionError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Validate Travel Rule transactions.  Checks what beneficiary VASP details are required by your jurisdiction and the beneficiary's jurisdiction.  **Note:** The reference content in this section documents the Travel Rule beta endpoint. The beta endpoint includes APIs that are currently in preview and aren't yet generally available.  To enroll in the beta and enable this endpoint, contact your Fireblocks Customer Success Manager or send an email to [CSM@fireblocks.com](mailto:CSM@fireblocks.com).
    async fn validate_travel_rule_transaction(&self,  params: ValidateTravelRuleTransactionParams ) -> Result<models::TravelRuleValidateTransactionResponse, Error<ValidateTravelRuleTransactionError>> {
        
        let ValidateTravelRuleTransactionParams {
            travel_rule_validate_transaction_request,
            idempotency_key,
        } = params;
        

        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/screening/travel_rule/transaction/validate", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(local_var_param_value) = idempotency_key {
            local_var_req_builder = local_var_req_builder.header("Idempotency-Key", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.json(&travel_rule_validate_transaction_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<ValidateTravelRuleTransactionError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

}

/// struct for typed errors of method [`get_aml_post_screening_policy`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAmlPostScreeningPolicyError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_aml_screening_configuration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAmlScreeningConfigurationError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_aml_screening_policy`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAmlScreeningPolicyError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_post_screening_policy`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPostScreeningPolicyError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_screening_configuration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetScreeningConfigurationError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_screening_policy`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetScreeningPolicyError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_vaspby_did`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetVaspbyDidError {
    Status400(),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_vasps`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetVaspsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_aml_screening_configuration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateAmlScreeningConfigurationError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_screening_configuration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateScreeningConfigurationError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_travel_rule_config`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateTravelRuleConfigError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_vasp`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateVaspError {
    Status400(),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`validate_full_travel_rule_transaction`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ValidateFullTravelRuleTransactionError {
    Status400(),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`validate_travel_rule_transaction`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ValidateTravelRuleTransactionError {
    Status400(),
    Status500(),
    UnknownValue(serde_json::Value),
}

