use crate::apis::vaults_api::VaultsApi;
use crate::apis::Api;
use crate::error::FireblocksError;
use crate::jwt::{JwtSigningMiddleware, Signer};
use crate::{error, ApiClient, Configuration, FIREBLOCKS_API, FIREBLOCKS_SANDBOX_API};
use jsonwebtoken::EncodingKey;
use reqwest::{Method, RequestBuilder, StatusCode};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::borrow::Borrow;
use std::fmt::{Debug, Display};
use std::sync::Arc;
use std::time::Duration;
use tracing::debug;
use url::Url;

#[derive(Clone)]
pub struct Client {
    api_client: Arc<ApiClient>,
}

pub struct ClientBuilder {
    api_key: String,
    client: Option<reqwest_middleware::ClientBuilder>,
    timeout: Duration,
    connect_timeout: Duration,
    user_agent: String,
    secret: Vec<u8>,
    url: String,
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            client: None,
            timeout: Duration::from_secs(15),
            connect_timeout: Duration::from_secs(5),
            user_agent: format!("fireblocks-sdk-rs {}", env!["CARGO_PKG_VERSION"]),
            secret: vec![],
            url: String::from(FIREBLOCKS_API),
        }
    }
}

impl ClientBuilder {
    pub fn new(api_key: &str, secret: &[u8]) -> Self {
        Self {
            api_key: String::from(api_key),
            secret: Vec::from(secret),
            ..Default::default()
        }
    }

    #[allow(unused_mut, clippy::return_self_not_must_use)]
    pub fn use_sandbox(mut self) -> Self {
        self.with_url(FIREBLOCKS_SANDBOX_API)
    }

    #[allow(unused_mut, clippy::return_self_not_must_use)]
    pub fn with_sandbox(mut self) -> Self {
        self.with_url(FIREBLOCKS_SANDBOX_API)
    }

    #[allow(clippy::return_self_not_must_use)]
    pub fn with_url(mut self, url: &str) -> Self {
        self.url = String::from(url);
        self
    }

    #[allow(clippy::return_self_not_must_use)]
    pub const fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    #[allow(clippy::return_self_not_must_use)]
    pub const fn with_connect_timeout(mut self, timeout: Duration) -> Self {
        self.connect_timeout = timeout;
        self
    }

    #[allow(clippy::return_self_not_must_use)]
    pub fn with_user_agent(mut self, ua: &str) -> Self {
        self.user_agent = String::from(ua);
        self
    }

    #[allow(clippy::return_self_not_must_use)]
    pub fn with_client(mut self, client: reqwest_middleware::ClientBuilder) -> Self {
        self.client = Some(client);
        self
    }

    pub fn build(self) -> Result<Client, error::FireblocksError> {
        let key = EncodingKey::from_rsa_pem(&self.secret[..])?;
        let signer = Signer::new(key, &self.api_key);
        let jwt_handler = JwtSigningMiddleware::new(signer);
        let c = self.client.unwrap_or_else(|| {
            let r = reqwest::ClientBuilder::new()
                .timeout(self.timeout)
                .connect_timeout(self.connect_timeout)
                .user_agent(String::from(&self.user_agent))
                .build()
                .unwrap_or_default();
            reqwest_middleware::ClientBuilder::new(r)
                .with(crate::log::LoggingMiddleware)
                .with(jwt_handler)
        });
        let c = c.build();
        Ok(Client::new_with_url(&self.url, c, Some(self.user_agent)))
    }
}

impl Client {
    fn new_with_url(
        url: &str,
        client: reqwest_middleware::ClientWithMiddleware,
        user_agent: Option<String>,
    ) -> Self {
        let cfg = Configuration {
            base_path: String::from(url),
            user_agent,
            client,
            basic_auth: None,
            oauth_access_token: None,
            bearer_access_token: None,
            api_key: None,
        };
        let api_client = Arc::new(ApiClient::new(Arc::new(cfg)));
        Self { api_client }
    }
}

impl Client {
    pub fn vaults_api(&self) -> &dyn VaultsApi {
        self.api_client.vaults_api()
    }
}
