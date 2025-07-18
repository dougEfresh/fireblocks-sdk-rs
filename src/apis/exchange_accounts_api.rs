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
pub trait ExchangeAccountsApi: Send + Sync {
    /// POST /exchange_accounts
    ///
    /// Add an exchange account to exchanges.   Note: This endpoint currently only supports the following exchanges `INDEPENDENT_RESERVE`,`BIT`, `BITHUMB`, `BITSO`, `CRYPTOCOM`, `BYBIT_V2`, `WHITEBIT`, `HITBTC`, `GEMINI`, `HUOBI`, `GATEIO`, `COINHAKO`, `BULLISH`, `BITGET`, and `LUNO`  To add an exchange account, please use the following [guide](https://developers.fireblocks.com/docs/add-an-exchange-account).
    async fn add_exchange_account(
        &self,
        params: AddExchangeAccountParams,
    ) -> Result<models::AddExchangeAccountResponse, Error<AddExchangeAccountError>>;

    /// POST /exchange_accounts/{exchangeAccountId}/convert
    ///
    /// Convert exchange account funds from the source asset to the destination asset. Coinbase (USD to USDC, USDC to USD) and Bitso (MXN to USD) are supported conversions. Learn more about Fireblocks Exchange Connectivity in the following [guide](https://developers.fireblocks.com/docs/connect-to-exchanges-and-fiat-providers). </br>Endpoint Permission: Admin, Non-Signing Admin.
    async fn convert_assets(
        &self,
        params: ConvertAssetsParams,
    ) -> Result<models::ConvertAssetsResponse, Error<ConvertAssetsError>>;

    /// GET /exchange_accounts/{exchangeAccountId}
    ///
    /// Returns an exchange account by ID. </br>Endpoint Permission: Admin,
    /// Non-Signing Admin.
    async fn get_exchange_account(
        &self,
        params: GetExchangeAccountParams,
    ) -> Result<models::ExchangeAccount, Error<GetExchangeAccountError>>;

    /// GET /exchange_accounts/{exchangeAccountId}/{assetId}
    ///
    /// Returns an asset for an exchange account. </br>Endpoint Permission:
    /// Admin, Non-Signing Admin.
    async fn get_exchange_account_asset(
        &self,
        params: GetExchangeAccountAssetParams,
    ) -> Result<models::ExchangeAsset, Error<GetExchangeAccountAssetError>>;

    /// GET /exchange_accounts/paged
    ///
    /// Returns a list of the connected exchange accounts in your workspace.
    /// </br>Endpoint Permission: Admin, Non-Signing Admin.
    async fn get_paged_exchange_accounts(
        &self,
        params: GetPagedExchangeAccountsParams,
    ) -> Result<Vec<models::ExchangeAccountsPaged>, Error<GetPagedExchangeAccountsError>>;

    /// POST /exchange_accounts/{exchangeAccountId}/internal_transfer
    ///
    /// Transfers funds between trading accounts under the same exchange account. Learn more about Fireblocks Exchange Connectivity in the following [guide](https://developers.fireblocks.com/docs/connect-to-exchanges-and-fiat-providers). </br>Endpoint Permission: Admin, Non-Signing Admin.
    async fn internal_transfer(
        &self,
        params: InternalTransferParams,
    ) -> Result<models::InternalTransferResponse, Error<InternalTransferError>>;
}

pub struct ExchangeAccountsApiClient {
    configuration: Arc<configuration::Configuration>,
}

impl ExchangeAccountsApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self { configuration }
    }
}

/// struct for passing parameters to the method [`add_exchange_account`]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct AddExchangeAccountParams {
    pub add_exchange_account_request: models::AddExchangeAccountRequest,
    /// A unique identifier for the request. If the request is sent multiple
    /// times with the same idempotency key, the server will return the same
    /// response as the first request. The idempotency key is valid for 24
    /// hours.
    pub idempotency_key: Option<String>,
}

/// struct for passing parameters to the method [`convert_assets`]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct ConvertAssetsParams {
    /// The ID of the exchange account. Please make sure the exchange supports
    /// conversions. To find the ID of your exchange account, use
    /// GET/exchange_accounts.
    pub exchange_account_id: String,
    /// A unique identifier for the request. If the request is sent multiple
    /// times with the same idempotency key, the server will return the same
    /// response as the first request. The idempotency key is valid for 24
    /// hours.
    pub idempotency_key: Option<String>,
    pub convert_assets_request: Option<models::ConvertAssetsRequest>,
}

/// struct for passing parameters to the method [`get_exchange_account`]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct GetExchangeAccountParams {
    /// The ID of the exchange account to return
    pub exchange_account_id: String,
}

/// struct for passing parameters to the method [`get_exchange_account_asset`]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct GetExchangeAccountAssetParams {
    /// The ID of the exchange account to return
    pub exchange_account_id: String,
    /// The ID of the asset to return
    pub asset_id: String,
}

/// struct for passing parameters to the method [`get_paged_exchange_accounts`]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct GetPagedExchangeAccountsParams {
    /// number of exchanges per page
    pub limit: f64,
    pub before: Option<String>,
    pub after: Option<String>,
}

/// struct for passing parameters to the method [`internal_transfer`]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct InternalTransferParams {
    /// The ID of the exchange account to return
    pub exchange_account_id: String,
    /// A unique identifier for the request. If the request is sent multiple
    /// times with the same idempotency key, the server will return the same
    /// response as the first request. The idempotency key is valid for 24
    /// hours.
    pub idempotency_key: Option<String>,
    pub create_internal_transfer_request: Option<models::CreateInternalTransferRequest>,
}

#[async_trait]
impl ExchangeAccountsApi for ExchangeAccountsApiClient {
    /// Add an exchange account to exchanges.   Note: This endpoint currently only supports the following exchanges `INDEPENDENT_RESERVE`,`BIT`, `BITHUMB`, `BITSO`, `CRYPTOCOM`, `BYBIT_V2`, `WHITEBIT`, `HITBTC`, `GEMINI`, `HUOBI`, `GATEIO`, `COINHAKO`, `BULLISH`, `BITGET`, and `LUNO`  To add an exchange account, please use the following [guide](https://developers.fireblocks.com/docs/add-an-exchange-account).
    async fn add_exchange_account(
        &self,
        params: AddExchangeAccountParams,
    ) -> Result<models::AddExchangeAccountResponse, Error<AddExchangeAccountError>> {
        let AddExchangeAccountParams {
            add_exchange_account_request,
            idempotency_key,
        } = params;

        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/exchange_accounts", local_var_configuration.base_path);
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
        local_var_req_builder = local_var_req_builder.json(&add_exchange_account_request);

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
                         `models::AddExchangeAccountResponse`",
                    )));
                }
                ContentType::Unsupported(local_var_unknown_type) => {
                    return Err(Error::from(serde_json::Error::custom(format!(
                        "Received `{local_var_unknown_type}` content type response that cannot be \
                         converted to `models::AddExchangeAccountResponse`"
                    ))));
                }
            }
        } else {
            let local_var_entity: Option<AddExchangeAccountError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Convert exchange account funds from the source asset to the destination asset. Coinbase (USD to USDC, USDC to USD) and Bitso (MXN to USD) are supported conversions. Learn more about Fireblocks Exchange Connectivity in the following [guide](https://developers.fireblocks.com/docs/connect-to-exchanges-and-fiat-providers). </br>Endpoint Permission: Admin, Non-Signing Admin.
    async fn convert_assets(
        &self,
        params: ConvertAssetsParams,
    ) -> Result<models::ConvertAssetsResponse, Error<ConvertAssetsError>> {
        let ConvertAssetsParams {
            exchange_account_id,
            idempotency_key,
            convert_assets_request,
        } = params;

        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/exchange_accounts/{exchangeAccountId}/convert",
            local_var_configuration.base_path,
            exchangeAccountId = crate::apis::urlencode(exchange_account_id)
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
        local_var_req_builder = local_var_req_builder.json(&convert_assets_request);

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
                         `models::ConvertAssetsResponse`",
                    )));
                }
                ContentType::Unsupported(local_var_unknown_type) => {
                    return Err(Error::from(serde_json::Error::custom(format!(
                        "Received `{local_var_unknown_type}` content type response that cannot be \
                         converted to `models::ConvertAssetsResponse`"
                    ))));
                }
            }
        } else {
            let local_var_entity: Option<ConvertAssetsError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Returns an exchange account by ID. </br>Endpoint Permission: Admin,
    /// Non-Signing Admin.
    async fn get_exchange_account(
        &self,
        params: GetExchangeAccountParams,
    ) -> Result<models::ExchangeAccount, Error<GetExchangeAccountError>> {
        let GetExchangeAccountParams {
            exchange_account_id,
        } = params;

        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/exchange_accounts/{exchangeAccountId}",
            local_var_configuration.base_path,
            exchangeAccountId = crate::apis::urlencode(exchange_account_id)
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
                         `models::ExchangeAccount`",
                    )));
                }
                ContentType::Unsupported(local_var_unknown_type) => {
                    return Err(Error::from(serde_json::Error::custom(format!(
                        "Received `{local_var_unknown_type}` content type response that cannot be \
                         converted to `models::ExchangeAccount`"
                    ))));
                }
            }
        } else {
            let local_var_entity: Option<GetExchangeAccountError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Returns an asset for an exchange account. </br>Endpoint Permission:
    /// Admin, Non-Signing Admin.
    async fn get_exchange_account_asset(
        &self,
        params: GetExchangeAccountAssetParams,
    ) -> Result<models::ExchangeAsset, Error<GetExchangeAccountAssetError>> {
        let GetExchangeAccountAssetParams {
            exchange_account_id,
            asset_id,
        } = params;

        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/exchange_accounts/{exchangeAccountId}/{assetId}",
            local_var_configuration.base_path,
            exchangeAccountId = crate::apis::urlencode(exchange_account_id),
            assetId = crate::apis::urlencode(asset_id)
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
                         `models::ExchangeAsset`",
                    )));
                }
                ContentType::Unsupported(local_var_unknown_type) => {
                    return Err(Error::from(serde_json::Error::custom(format!(
                        "Received `{local_var_unknown_type}` content type response that cannot be \
                         converted to `models::ExchangeAsset`"
                    ))));
                }
            }
        } else {
            let local_var_entity: Option<GetExchangeAccountAssetError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Returns a list of the connected exchange accounts in your workspace.
    /// </br>Endpoint Permission: Admin, Non-Signing Admin.
    async fn get_paged_exchange_accounts(
        &self,
        params: GetPagedExchangeAccountsParams,
    ) -> Result<Vec<models::ExchangeAccountsPaged>, Error<GetPagedExchangeAccountsError>> {
        let GetPagedExchangeAccountsParams {
            limit,
            before,
            after,
        } = params;

        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/exchange_accounts/paged",
            local_var_configuration.base_path
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_str) = before {
            local_var_req_builder =
                local_var_req_builder.query(&[("before", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = after {
            local_var_req_builder =
                local_var_req_builder.query(&[("after", &local_var_str.to_string())]);
        }
        local_var_req_builder = local_var_req_builder.query(&[("limit", &limit.to_string())]);
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
                         `Vec&lt;models::ExchangeAccountsPaged&gt;`",
                    )));
                }
                ContentType::Unsupported(local_var_unknown_type) => {
                    return Err(Error::from(serde_json::Error::custom(format!(
                        "Received `{local_var_unknown_type}` content type response that cannot be \
                         converted to `Vec&lt;models::ExchangeAccountsPaged&gt;`"
                    ))));
                }
            }
        } else {
            let local_var_entity: Option<GetPagedExchangeAccountsError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Transfers funds between trading accounts under the same exchange account. Learn more about Fireblocks Exchange Connectivity in the following [guide](https://developers.fireblocks.com/docs/connect-to-exchanges-and-fiat-providers). </br>Endpoint Permission: Admin, Non-Signing Admin.
    async fn internal_transfer(
        &self,
        params: InternalTransferParams,
    ) -> Result<models::InternalTransferResponse, Error<InternalTransferError>> {
        let InternalTransferParams {
            exchange_account_id,
            idempotency_key,
            create_internal_transfer_request,
        } = params;

        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/exchange_accounts/{exchangeAccountId}/internal_transfer",
            local_var_configuration.base_path,
            exchangeAccountId = crate::apis::urlencode(exchange_account_id)
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
        local_var_req_builder = local_var_req_builder.json(&create_internal_transfer_request);

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
                         `models::InternalTransferResponse`",
                    )));
                }
                ContentType::Unsupported(local_var_unknown_type) => {
                    return Err(Error::from(serde_json::Error::custom(format!(
                        "Received `{local_var_unknown_type}` content type response that cannot be \
                         converted to `models::InternalTransferResponse`"
                    ))));
                }
            }
        } else {
            let local_var_entity: Option<InternalTransferError> =
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

/// struct for typed errors of method [`add_exchange_account`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddExchangeAccountError {
    DefaultResponse(models::ErrorSchema),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`convert_assets`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConvertAssetsError {
    DefaultResponse(models::ErrorSchema),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_exchange_account`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetExchangeAccountError {
    DefaultResponse(models::ErrorSchema),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_exchange_account_asset`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetExchangeAccountAssetError {
    DefaultResponse(models::ErrorSchema),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_paged_exchange_accounts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPagedExchangeAccountsError {
    DefaultResponse(models::ErrorSchema),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`internal_transfer`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InternalTransferError {
    DefaultResponse(models::ErrorSchema),
    UnknownValue(serde_json::Value),
}
