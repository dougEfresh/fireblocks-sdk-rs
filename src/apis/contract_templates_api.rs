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
pub trait ContractTemplatesApi: Send + Sync {
    async fn delete_contract_template_by_id(&self,  params: DeleteContractTemplateByIdParams ) -> Result<(), Error<DeleteContractTemplateByIdError>>;
    async fn deploy_contract(&self,  params: DeployContractParams ) -> Result<models::ContractDeployResponse, Error<DeployContractError>>;
    async fn get_constructor_by_contract_template_id(&self,  params: GetConstructorByContractTemplateIdParams ) -> Result<models::AbiFunction, Error<GetConstructorByContractTemplateIdError>>;
    async fn get_contract_template_by_id(&self,  params: GetContractTemplateByIdParams ) -> Result<models::ContractTemplateDto, Error<GetContractTemplateByIdError>>;
    async fn get_contract_templates(&self,  params: GetContractTemplatesParams ) -> Result<models::TemplatesPaginatedResponse, Error<GetContractTemplatesError>>;
    async fn get_function_abi_by_contract_template_id(&self,  params: GetFunctionAbiByContractTemplateIdParams ) -> Result<models::AbiFunction, Error<GetFunctionAbiByContractTemplateIdError>>;
    async fn upload_contract_template(&self,  params: UploadContractTemplateParams ) -> Result<models::ContractTemplateDto, Error<UploadContractTemplateError>>;
}

pub struct ContractTemplatesApiClient {
    configuration: Arc<configuration::Configuration>
}

impl ContractTemplatesApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self { configuration }
    }
}


/// struct for passing parameters to the method [`delete_contract_template_by_id`]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct DeleteContractTemplateByIdParams {
    /// The Contract Template identifier
    pub contract_template_id: String
}

/// struct for passing parameters to the method [`deploy_contract`]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct DeployContractParams {
    /// The Contract Template identifier
    pub contract_template_id: String,
    pub contract_deploy_request: models::ContractDeployRequest,
    /// A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours.
    pub idempotency_key: Option<String>
}

/// struct for passing parameters to the method [`get_constructor_by_contract_template_id`]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct GetConstructorByContractTemplateIdParams {
    /// The Contract Template identifier
    pub contract_template_id: String,
    /// true if you want to get the abi with its docs
    pub with_docs: Option<bool>
}

/// struct for passing parameters to the method [`get_contract_template_by_id`]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct GetContractTemplateByIdParams {
    /// The Contract Template identifier
    pub contract_template_id: String
}

/// struct for passing parameters to the method [`get_contract_templates`]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct GetContractTemplatesParams {
    /// Page cursor to get the next page
    pub page_cursor: Option<String>,
    /// Number of items per page, requesting more then max will return max items
    pub page_size: Option<f64>,
    /// The type of the contract templates you wish to retrieve. Can accept one type, more or none
    pub r#type: Option<String>,
    /// For standalone contracts use ON_DEPLOYMENT and for contracts that are behind proxies use POST_DEPLOYMENT
    pub initialization_phase: Option<String>
}

/// struct for passing parameters to the method [`get_function_abi_by_contract_template_id`]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct GetFunctionAbiByContractTemplateIdParams {
    /// The Contract Template identifier
    pub contract_template_id: String,
    /// The contract's function signature
    pub function_signature: String
}

/// struct for passing parameters to the method [`upload_contract_template`]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct UploadContractTemplateParams {
    pub contract_upload_request: models::ContractUploadRequest,
    /// A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours.
    pub idempotency_key: Option<String>
}


#[async_trait]
impl ContractTemplatesApi for ContractTemplatesApiClient {
    /// Delete a contract by id. allowed only for private contract templates. Notice: it is irreversible!
    async fn delete_contract_template_by_id(&self,  params: DeleteContractTemplateByIdParams ) -> Result<(), Error<DeleteContractTemplateByIdError>> {
        
        let DeleteContractTemplateByIdParams {
            contract_template_id,
        } = params;
        

        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/tokenization/templates/{contractTemplateId}", local_var_configuration.base_path, contractTemplateId=crate::apis::urlencode(contract_template_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            Ok(())
        } else {
            let local_var_entity: Option<DeleteContractTemplateByIdError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Deploy a new contract by contract template id. If you wish to deploy a token (ERC20, ERC721 etc), and create asset please use POST /tokenization
    async fn deploy_contract(&self,  params: DeployContractParams ) -> Result<models::ContractDeployResponse, Error<DeployContractError>> {
        
        let DeployContractParams {
            contract_template_id,
            contract_deploy_request,
            idempotency_key,
        } = params;
        

        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/tokenization/templates/{contractTemplateId}/deploy", local_var_configuration.base_path, contractTemplateId=crate::apis::urlencode(contract_template_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(local_var_param_value) = idempotency_key {
            local_var_req_builder = local_var_req_builder.header("Idempotency-Key", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.json(&contract_deploy_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<DeployContractError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Return contract template's constructor ABI
    async fn get_constructor_by_contract_template_id(&self,  params: GetConstructorByContractTemplateIdParams ) -> Result<models::AbiFunction, Error<GetConstructorByContractTemplateIdError>> {
        
        let GetConstructorByContractTemplateIdParams {
            contract_template_id,
            with_docs,
        } = params;
        

        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/tokenization/templates/{contractTemplateId}/constructor", local_var_configuration.base_path, contractTemplateId=crate::apis::urlencode(contract_template_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_str) = with_docs {
            local_var_req_builder = local_var_req_builder.query(&[("withDocs", &local_var_str.to_string())]);
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
            let local_var_entity: Option<GetConstructorByContractTemplateIdError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Return detailed information about the contract template
    async fn get_contract_template_by_id(&self,  params: GetContractTemplateByIdParams ) -> Result<models::ContractTemplateDto, Error<GetContractTemplateByIdError>> {
        
        let GetContractTemplateByIdParams {
            contract_template_id,
        } = params;
        

        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/tokenization/templates/{contractTemplateId}", local_var_configuration.base_path, contractTemplateId=crate::apis::urlencode(contract_template_id));
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
            let local_var_entity: Option<GetContractTemplateByIdError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Return minimal representation of all the contract templates available for the workspace
    async fn get_contract_templates(&self,  params: GetContractTemplatesParams ) -> Result<models::TemplatesPaginatedResponse, Error<GetContractTemplatesError>> {
        
        let GetContractTemplatesParams {
            page_cursor,
            page_size,
            r#type,
            initialization_phase,
        } = params;
        

        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/tokenization/templates", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_str) = page_cursor {
            local_var_req_builder = local_var_req_builder.query(&[("pageCursor", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = page_size {
            local_var_req_builder = local_var_req_builder.query(&[("pageSize", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = r#type {
            local_var_req_builder = local_var_req_builder.query(&[("type", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = initialization_phase {
            local_var_req_builder = local_var_req_builder.query(&[("initializationPhase", &local_var_str.to_string())]);
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
            let local_var_entity: Option<GetContractTemplatesError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Return contract template`s function ABI by signature
    async fn get_function_abi_by_contract_template_id(&self,  params: GetFunctionAbiByContractTemplateIdParams ) -> Result<models::AbiFunction, Error<GetFunctionAbiByContractTemplateIdError>> {
        
        let GetFunctionAbiByContractTemplateIdParams {
            contract_template_id,
            function_signature,
        } = params;
        

        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/tokenization/templates/{contractTemplateId}/function", local_var_configuration.base_path, contractTemplateId=crate::apis::urlencode(contract_template_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        local_var_req_builder = local_var_req_builder.query(&[("functionSignature", &function_signature.to_string())]);
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
            let local_var_entity: Option<GetFunctionAbiByContractTemplateIdError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Upload a new contract template. This contract template will be available for the workspace
    async fn upload_contract_template(&self,  params: UploadContractTemplateParams ) -> Result<models::ContractTemplateDto, Error<UploadContractTemplateError>> {
        
        let UploadContractTemplateParams {
            contract_upload_request,
            idempotency_key,
        } = params;
        

        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/tokenization/templates", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(local_var_param_value) = idempotency_key {
            local_var_req_builder = local_var_req_builder.header("Idempotency-Key", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.json(&contract_upload_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<UploadContractTemplateError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

}

/// struct for typed errors of method [`delete_contract_template_by_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteContractTemplateByIdError {
    Status404(models::HttpContractDoesNotExistError),
    DefaultResponse(models::ErrorSchema),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`deploy_contract`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeployContractError {
    Status404(models::HttpContractDoesNotExistError),
    DefaultResponse(models::ErrorSchema),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_constructor_by_contract_template_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetConstructorByContractTemplateIdError {
    Status404(models::HttpContractDoesNotExistError),
    DefaultResponse(models::ErrorSchema),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_contract_template_by_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetContractTemplateByIdError {
    Status404(models::HttpContractDoesNotExistError),
    DefaultResponse(models::ErrorSchema),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_contract_templates`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetContractTemplatesError {
    DefaultResponse(models::ErrorSchema),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_function_abi_by_contract_template_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFunctionAbiByContractTemplateIdError {
    Status404(models::HttpContractDoesNotExistError),
    DefaultResponse(models::ErrorSchema),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`upload_contract_template`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UploadContractTemplateError {
    DefaultResponse(models::ErrorSchema),
    UnknownValue(serde_json::Value),
}

