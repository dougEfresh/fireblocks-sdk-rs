// Fireblocks API
//
// Fireblocks provides a suite of applications to manage digital asset operations and a complete development platform to build your business on the blockchain.  - Visit our website for more information: [Fireblocks Website](https://fireblocks.com) - Visit our developer docs: [Fireblocks DevPortal](https://developers.fireblocks.com)
//
// The version of the OpenAPI document: 1.8.0
// Contact: developers@fireblocks.com
// Generated by: https://openapi-generator.tech

use {
    super::{Error, configuration},
    crate::{
        apis::{ContentType, ResponseContent},
        models,
    },
    async_trait::async_trait,
    reqwest,
    serde::{Deserialize, Serialize, de::Error as _},
    std::sync::Arc,
};

#[async_trait]
pub trait JobManagementApi: Send + Sync {
    /// POST /batch/{jobId}/cancel
    ///
    /// Stop the given job immediately. If the job is in the ‘Active’ state, the
    /// job will be canceled after completing the current task. Vault accounts
    /// and Wallets that are already created will not be affected.
    async fn cancel_job(&self, params: CancelJobParams) -> Result<(), Error<CancelJobError>>;

    /// POST /batch/{jobId}/continue
    ///
    /// Continue the given paused job.
    async fn continue_job(&self, params: ContinueJobParams) -> Result<(), Error<ContinueJobError>>;

    /// GET /batch/{jobId}
    ///
    /// Get an object describing the given job
    async fn get_job(&self, params: GetJobParams) -> Result<models::Job, Error<GetJobError>>;

    /// GET /batch/{jobId}/tasks
    ///
    /// Return a list of tasks for given job
    async fn get_job_tasks(
        &self,
        params: GetJobTasksParams,
    ) -> Result<Vec<models::Task>, Error<GetJobTasksError>>;

    /// GET /batch/jobs
    ///
    /// Get an array of objects including all active, paused, canceled, and
    /// complete jobs in a workspace.
    async fn get_jobs(
        &self,
        params: GetJobsParams,
    ) -> Result<Vec<models::Job>, Error<GetJobsError>>;

    /// POST /batch/{jobId}/pause
    ///
    /// Pause the given job, after the current task is done. A paused job can
    /// later be resumed by calling ‘continue’, or canceled.
    async fn pause_job(&self, params: PauseJobParams) -> Result<(), Error<PauseJobError>>;
}

pub struct JobManagementApiClient {
    configuration: Arc<configuration::Configuration>,
}

impl JobManagementApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self { configuration }
    }
}

/// struct for passing parameters to the method [`cancel_job`]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct CancelJobParams {
    /// The requested job id
    pub job_id: String,
    /// A unique identifier for the request. If the request is sent multiple
    /// times with the same idempotency key, the server will return the same
    /// response as the first request. The idempotency key is valid for 24
    /// hours.
    pub idempotency_key: Option<String>,
}

/// struct for passing parameters to the method [`continue_job`]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct ContinueJobParams {
    /// The requested job id
    pub job_id: String,
    /// A unique identifier for the request. If the request is sent multiple
    /// times with the same idempotency key, the server will return the same
    /// response as the first request. The idempotency key is valid for 24
    /// hours.
    pub idempotency_key: Option<String>,
}

/// struct for passing parameters to the method [`get_job`]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct GetJobParams {
    /// The requested job id
    pub job_id: String,
}

/// struct for passing parameters to the method [`get_job_tasks`]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct GetJobTasksParams {
    /// The requested job id
    pub job_id: String,
}

/// struct for passing parameters to the method [`get_jobs`]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct GetJobsParams {
    /// Start of time range in ms since 1970
    pub from_time: Option<i32>,
    /// End of time range in ms since 1970
    pub to_time: Option<i32>,
}

/// struct for passing parameters to the method [`pause_job`]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct PauseJobParams {
    /// The requested job id
    pub job_id: String,
    /// A unique identifier for the request. If the request is sent multiple
    /// times with the same idempotency key, the server will return the same
    /// response as the first request. The idempotency key is valid for 24
    /// hours.
    pub idempotency_key: Option<String>,
}

#[async_trait]
impl JobManagementApi for JobManagementApiClient {
    /// Stop the given job immediately. If the job is in the ‘Active’ state, the
    /// job will be canceled after completing the current task. Vault accounts
    /// and Wallets that are already created will not be affected.
    async fn cancel_job(&self, params: CancelJobParams) -> Result<(), Error<CancelJobError>> {
        let CancelJobParams {
            job_id,
            idempotency_key,
        } = params;

        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/batch/{jobId}/cancel",
            local_var_configuration.base_path,
            jobId = crate::apis::urlencode(job_id)
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(local_var_param_value) = idempotency_key {
            local_var_req_builder =
                local_var_req_builder.header("Idempotency-Key", local_var_param_value.to_string());
        }

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            Ok(())
        } else {
            let local_var_entity: Option<CancelJobError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Continue the given paused job.
    async fn continue_job(&self, params: ContinueJobParams) -> Result<(), Error<ContinueJobError>> {
        let ContinueJobParams {
            job_id,
            idempotency_key,
        } = params;

        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/batch/{jobId}/continue",
            local_var_configuration.base_path,
            jobId = crate::apis::urlencode(job_id)
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(local_var_param_value) = idempotency_key {
            local_var_req_builder =
                local_var_req_builder.header("Idempotency-Key", local_var_param_value.to_string());
        }

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            Ok(())
        } else {
            let local_var_entity: Option<ContinueJobError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Get an object describing the given job
    async fn get_job(&self, params: GetJobParams) -> Result<models::Job, Error<GetJobError>> {
        let GetJobParams { job_id } = params;

        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/batch/{jobId}",
            local_var_configuration.base_path,
            jobId = crate::apis::urlencode(job_id)
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content_type = local_var_resp
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("application/octet-stream");
        let local_var_content_type = super::ContentType::from(local_var_content_type);
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            match local_var_content_type {
                ContentType::Json => serde_json::from_str(&local_var_content).map_err(Error::from),
                ContentType::Text => {
                    return Err(Error::from(serde_json::Error::custom(
                        "Received `text/plain` content type response that cannot be converted to \
                         `models::Job`",
                    )));
                }
                ContentType::Unsupported(local_var_unknown_type) => {
                    return Err(Error::from(serde_json::Error::custom(format!(
                        "Received `{local_var_unknown_type}` content type response that cannot be \
                         converted to `models::Job`"
                    ))));
                }
            }
        } else {
            let local_var_entity: Option<GetJobError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Return a list of tasks for given job
    async fn get_job_tasks(
        &self,
        params: GetJobTasksParams,
    ) -> Result<Vec<models::Task>, Error<GetJobTasksError>> {
        let GetJobTasksParams { job_id } = params;

        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/batch/{jobId}/tasks",
            local_var_configuration.base_path,
            jobId = crate::apis::urlencode(job_id)
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content_type = local_var_resp
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("application/octet-stream");
        let local_var_content_type = super::ContentType::from(local_var_content_type);
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            match local_var_content_type {
                ContentType::Json => serde_json::from_str(&local_var_content).map_err(Error::from),
                ContentType::Text => {
                    return Err(Error::from(serde_json::Error::custom(
                        "Received `text/plain` content type response that cannot be converted to \
                         `Vec&lt;models::Task&gt;`",
                    )));
                }
                ContentType::Unsupported(local_var_unknown_type) => {
                    return Err(Error::from(serde_json::Error::custom(format!(
                        "Received `{local_var_unknown_type}` content type response that cannot be \
                         converted to `Vec&lt;models::Task&gt;`"
                    ))));
                }
            }
        } else {
            let local_var_entity: Option<GetJobTasksError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Get an array of objects including all active, paused, canceled, and
    /// complete jobs in a workspace.
    async fn get_jobs(
        &self,
        params: GetJobsParams,
    ) -> Result<Vec<models::Job>, Error<GetJobsError>> {
        let GetJobsParams { from_time, to_time } = params;

        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/batch/jobs", local_var_configuration.base_path);
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_str) = from_time {
            local_var_req_builder =
                local_var_req_builder.query(&[("fromTime", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = to_time {
            local_var_req_builder =
                local_var_req_builder.query(&[("toTime", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content_type = local_var_resp
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("application/octet-stream");
        let local_var_content_type = super::ContentType::from(local_var_content_type);
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            match local_var_content_type {
                ContentType::Json => serde_json::from_str(&local_var_content).map_err(Error::from),
                ContentType::Text => {
                    return Err(Error::from(serde_json::Error::custom(
                        "Received `text/plain` content type response that cannot be converted to \
                         `Vec&lt;models::Job&gt;`",
                    )));
                }
                ContentType::Unsupported(local_var_unknown_type) => {
                    return Err(Error::from(serde_json::Error::custom(format!(
                        "Received `{local_var_unknown_type}` content type response that cannot be \
                         converted to `Vec&lt;models::Job&gt;`"
                    ))));
                }
            }
        } else {
            let local_var_entity: Option<GetJobsError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Pause the given job, after the current task is done. A paused job can
    /// later be resumed by calling ‘continue’, or canceled.
    async fn pause_job(&self, params: PauseJobParams) -> Result<(), Error<PauseJobError>> {
        let PauseJobParams {
            job_id,
            idempotency_key,
        } = params;

        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/batch/{jobId}/pause",
            local_var_configuration.base_path,
            jobId = crate::apis::urlencode(job_id)
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(local_var_param_value) = idempotency_key {
            local_var_req_builder =
                local_var_req_builder.header("Idempotency-Key", local_var_param_value.to_string());
        }

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            Ok(())
        } else {
            let local_var_entity: Option<PauseJobError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }
}

/// struct for typed errors of method [`cancel_job`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CancelJobError {
    DefaultResponse(models::ErrorSchema),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`continue_job`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContinueJobError {
    DefaultResponse(models::ErrorSchema),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_job`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetJobError {
    DefaultResponse(models::ErrorSchema),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_job_tasks`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetJobTasksError {
    DefaultResponse(models::ErrorSchema),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_jobs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetJobsError {
    DefaultResponse(models::ErrorSchema),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`pause_job`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PauseJobError {
    DefaultResponse(models::ErrorSchema),
    UnknownValue(serde_json::Value),
}
