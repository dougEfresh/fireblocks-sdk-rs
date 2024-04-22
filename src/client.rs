use crate::error::FireblocksError;
use crate::jwt::Signer;
use crate::{error, FIREBLOCKS_API, FIREBLOCKS_SANDBOX_API};
use jsonwebtoken::EncodingKey;
use reqwest::{Method, RequestBuilder, StatusCode};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::borrow::Borrow;
use std::fmt::Debug;
use std::sync::Arc;
use std::time::Duration;
use tracing::debug;
use url::Url;

#[derive(Clone)]
pub struct Client {
  signer: Arc<Signer>,
  client: reqwest::Client,
  host: String,
}

pub struct ClientBuilder {
  api_key: String,
  client: Option<reqwest::Client>,
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
    Self { api_key: String::from(api_key), secret: Vec::from(secret), ..Default::default() }
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
  pub fn with_client(mut self, client: reqwest::Client) -> Self {
    self.client = Some(client);
    self
  }

  pub fn build(&self) -> Result<Client, error::ClientError> {
    let c = match self.client.as_ref() {
      None => reqwest::ClientBuilder::new()
        .timeout(self.timeout)
        .connect_timeout(self.connect_timeout)
        .user_agent(String::from(&self.user_agent))
        .build()?,
      Some(cl) => cl.clone(),
    };
    let key = EncodingKey::from_rsa_pem(&self.secret[..])?;
    let signer = Signer::new(key, &self.api_key);
    Ok(Client::new_with_url(signer, &self.url, c))
  }
}

impl Client {
  fn new_with_url(signer: Signer, url: &str, client: reqwest::Client) -> Self {
    Self { signer: Arc::new(signer), client, host: url.to_owned() }
  }
}

// This impl block contains the underlying GET/POST helpers for authing to fireblocks
impl Client {
  #[allow(clippy::option_if_let_else)]
  #[tracing::instrument(skip(self, url, body), fields(path))]
  pub(crate) async fn send<T, S>(&self, method: Method, url: Url, body: Option<&S>) -> crate::Result<T>
  where
    T: DeserializeOwned + Default,
    S: Serialize + ?Sized + Debug + Send + Sync,
  {
    let mut path = String::from(url.path());
    if let Some(q) = url.query() {
      path = format!("{path}?{q}");
    }
    tracing::Span::current().record("path", &path);
    debug!("sending request {method} {path}");

    #[cfg(debug_assertions)]
    match body {
      None => debug!("sending request {method} {path}"),
      Some(b) => {
        if let Ok(s) = serde_json::to_string(b) {
          debug!("sending request {method} {path} {:#?}", s);
        }
      },
    }

    let req = match method {
      Method::GET => self.client.get(url),
      Method::POST => self.client.post(url),
      Method::DELETE => self.client.delete(url),
      Method::PATCH => self.client.patch(url),
      Method::PUT => self.client.put(url),
      _ => todo!(),
    };
    let mut req = self.authed(&path, req, body)?.0;
    if let Some(b) = body {
      req = req.json(b);
    }

    let resp = req.send().await?;
    let status = resp.status();
    let request_id =
      resp.headers().get("x-request-id").and_then(|value| value.to_str().ok()).unwrap_or_default().to_string();
    let json_response = resp
      .headers()
      .get("content-type")
      .and_then(|value| value.to_str().ok())
      .unwrap_or_default()
      .to_string()
      .contains("json");
    #[cfg(debug_assertions)]
    debug!("got response with x-request-id={}", request_id);
    let text = resp.text().await?;

    let r: crate::Result<T> = match status {
      StatusCode::OK | StatusCode::ACCEPTED | StatusCode::CREATED => {
        if text.is_empty() || !json_response {
          Ok((T::default(), request_id))
        } else {
          //debug!("body: {text}");
          match serde_json::from_str::<T>(&text) {
            Ok(deserialized) => Ok((deserialized, request_id)),
            Err(err) => Err(FireblocksError::SerdeJson { request_id, err, text }),
          }
        }
      },
      StatusCode::NOT_FOUND => Err(FireblocksError::NotFound { request_id, path }),
      StatusCode::BAD_REQUEST => Err(FireblocksError::BadRequest { request_id, path, text }),
      StatusCode::UNAUTHORIZED => Err(FireblocksError::Unauthorized { request_id, path, text }),
      StatusCode::SERVICE_UNAVAILABLE | StatusCode::GATEWAY_TIMEOUT | StatusCode::INTERNAL_SERVER_ERROR => {
        Err(FireblocksError::InternalError { request_id, code: status.as_u16(), text })
      },
      _ => Err(FireblocksError::Unknown { request_id, code: status.as_u16(), text }),
    };
    r
  }

  pub(crate) fn build_url(&self, path: &str) -> crate::Result<Url> {
    self.build_url_params::<Vec<(&str, &str)>, &str, &str>(path, None)
  }

  pub(crate) fn build_url_params<I, K, V>(&self, path: &str, params: Option<I>) -> crate::Result<Url>
  where
    I: IntoIterator,
    I::Item: Borrow<(K, V)>,
    K: AsRef<str>,
    V: AsRef<str>,
  {
    let mut url = Url::parse(&format!("{}/{path}", self.host))?;

    match params {
      None => Ok((url, String::new())),
      Some(p) => {
        url.query_pairs_mut().extend_pairs(p);
        Ok((url, String::new()))
      },
    }
  }

  pub(crate) async fn get<R: DeserializeOwned + Default>(&self, url: Url) -> crate::Result<R> {
    self.send(Method::GET, url, None as Option<&()>).await
  }

  pub(crate) async fn delete<R: DeserializeOwned + Default>(&self, url: Url) -> crate::Result<R> {
    self.send(Method::DELETE, url, None as Option<&()>).await
  }

  pub(crate) async fn post<R, S>(&self, url: Url, body: Option<&S>) -> crate::Result<R>
  where
    R: DeserializeOwned + Default,
    S: Serialize + ?Sized + Debug + Send + Sync,
  {
    self.send(Method::POST, url, body).await
  }

  pub(crate) async fn put<R, S>(&self, url: Url, body: Option<&S>) -> crate::Result<R>
  where
    R: DeserializeOwned + Default,
    S: Serialize + ?Sized + Debug + Send + Sync,
  {
    self.send(Method::PUT, url, body).await
  }

  pub(crate) fn authed<S>(&self, url: &str, req: RequestBuilder, body: Option<&S>) -> crate::Result<RequestBuilder>
  where
    S: Serialize + ?Sized + Debug + Send + Sync,
  {
    let jwt = self.signer.sign(url, body)?;
    Ok((req.header("X-API-Key", self.signer.api_key()).bearer_auth(jwt), String::new()))
  }
}
