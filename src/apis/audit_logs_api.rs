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
pub trait AuditLogsApi: Send + Sync {
    async fn audits_get(&self,  params: AuditsGetParams ) -> Result<(), Error<AuditsGetError>>;
}

pub struct AuditLogsApiClient {
    configuration: Arc<configuration::Configuration>
}

impl AuditLogsApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self { configuration }
    }
}


/// struct for passing parameters to the method [`audits_get`]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct AuditsGetParams {
    /// The last time period to fetch audit logs
    pub time_period: String
}


#[async_trait]
impl AuditLogsApi for AuditLogsApiClient {
    /// Deprecated. Please use the following [endpoint](https://developers.fireblocks.com/reference/getaudits) instead.
    async fn audits_get(&self,  params: AuditsGetParams ) -> Result<(), Error<AuditsGetError>> {
        
        let AuditsGetParams {
            time_period,
        } = params;
        

        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/audits", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        local_var_req_builder = local_var_req_builder.query(&[("timePeriod", &time_period.to_string())]);
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
            let local_var_entity: Option<AuditsGetError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

}

/// struct for typed errors of method [`audits_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AuditsGetError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

