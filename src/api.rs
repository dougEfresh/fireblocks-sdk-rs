use std::{fmt::Debug, sync::Arc};

use bigdecimal::BigDecimal;
use log::debug;
use reqwest::Method;
#[cfg(not(target_arch = "wasm32"))]
use reqwest::{Client, RequestBuilder, StatusCode, Url};
use serde::{de::DeserializeOwned, Serialize};

use crate::{
  error::FireblocksError,
  jwt::Signer,
  types::{
    address::{Address, AddressContainer, CreateAddressResponse},
    asset::SupportedAsset,
    connect::PagedWalletConnectResponse,
    fee::EstimateFee,
    staking::{StakingPosition, StakingPositionsSummary},
    transaction::{CreateTransactionResponse, Transaction, TransactionArguments, TransactionListOptions},
    vault::{Account, CreateAccount, VaultAccounts},
    wallet::{WalletContainer, WalletCreate, WalletCreateAsset, WalletCreateAssetResponse},
    PaginatedAssetWallet, PagingVaultRequest,
  },
  FireblocksFactory, Result,
};

pub const FIREBLOCKS_API: &str = "https://api.fireblocks.io/v1";
pub const FIREBLOCKS_SANDBOX_API: &str = "https://sandbox-api.fireblocks.io/v1";

#[derive(Clone)]
pub struct FireblocksHttpClient {
  signer: Arc<Signer>,
  client: Client,
  host: String,
}

impl FireblocksHttpClient {
  pub fn new(signer: Signer, client: Client) -> Self {
    Self::new_with_url(signer, FIREBLOCKS_API, client)
  }

  pub fn sandbox(signer: Signer, client: Client) -> Self {
    Self::new_with_url(signer, FIREBLOCKS_SANDBOX_API, client)
  }

  pub fn new_with_url(signer: Signer, url: &str, client: Client) -> Self {
    Self { signer: Arc::new(signer), client, host: url.to_owned() }
  }
}

// This impl block contains the underlying GET/POST helpers for authing to fireblocks
impl FireblocksHttpClient {
  fn build_uri(&self, path: &str, page: Option<&PagingVaultRequest>) -> Result<Url> {
    let mut url = Url::parse(&format!("{}/{path}", self.host))?;

    match page {
      None => Ok((url, String::new())),
      Some(paging) => {
        url.query_pairs_mut().extend_pairs(paging.params());
        Ok((url, String::new()))
      },
    }
  }

  async fn get<R: DeserializeOwned + Default>(&self, url: Url) -> Result<R> {
    self.send_no_body(url, Method::GET).await
  }

  async fn delete<R: DeserializeOwned + Default>(&self, url: Url) -> Result<R> {
    self.send_no_body(url, Method::DELETE).await
  }

  async fn post<R: DeserializeOwned + Default>(&self, url: Url) -> Result<R> {
    self.send_no_body(url, Method::POST).await
  }

  async fn post_body<S, R>(&self, url: Url, body: S) -> Result<R>
  where
    S: Serialize + Debug,
    R: DeserializeOwned + Send + Default,
  {
    let mut path = String::from(url.path());
    if let Some(q) = url.query() {
      path = format!("{path}?{q}");
    }
    let req = self.client.post(url).json(&body);
    self.send(&path, req, body).await
  }

  async fn send_no_body<R: DeserializeOwned + Default>(&self, url: Url, method: Method) -> Result<R> {
    let mut path = String::from(url.path());
    if let Some(q) = url.query() {
      path = format!("{path}?{q}");
    }
    let req = match method {
      Method::GET => self.client.get(url),
      Method::POST => self.client.post(url),
      Method::DELETE => self.client.delete(url),
      Method::PATCH => self.client.patch(url),
      _ => todo!(),
    };
    self.send(&path, req, ()).await
  }

  async fn send<S, R>(&self, path: &str, req: RequestBuilder, body: S) -> Result<R>
  where
    S: Serialize + Debug,
    R: DeserializeOwned + Default,
  {
    debug!("sending request {}", path);
    let (req, _) = self.authed(path, req, &body)?;
    let response = req.send().await?;
    let status = response.status();
    let request_id =
      response.headers().get("x-request-id").and_then(|value| value.to_str().ok()).unwrap_or_default().to_string();
    debug!("got response with x-request-id={}", request_id);

    let text = response.text().await?;

    // debug!("body response {}", text.clone());
    let r: Result<R> = match status {
      StatusCode::OK | StatusCode::ACCEPTED | StatusCode::CREATED => {
        if text.is_empty() {
          Ok((R::default(), request_id))
        } else {
          match serde_json::from_str::<R>(&text) {
            Ok(deserialized) => Ok((deserialized, request_id)),
            Err(err) => Err(FireblocksError::SerdeJson { request_id, err, text }),
          }
        }
      },
      StatusCode::NOT_FOUND => Err(FireblocksError::NotFound { request_id, path: String::from(path) }),
      StatusCode::BAD_REQUEST => Err(FireblocksError::BadRequest { request_id, path: String::from(path), text }),
      StatusCode::UNAUTHORIZED => Err(FireblocksError::Unauthorized { request_id, path: String::from(path), text }),
      StatusCode::SERVICE_UNAVAILABLE | StatusCode::GATEWAY_TIMEOUT | StatusCode::INTERNAL_SERVER_ERROR => {
        Err(FireblocksError::InternalError { request_id, code: status.as_u16(), text })
      },
      _ => Err(FireblocksError::Unknown { request_id, code: status.as_u16(), text }),
    };
    r
  }

  fn authed<S: Serialize + Debug>(&self, url: &str, req: RequestBuilder, body: &S) -> Result<RequestBuilder> {
    let jwt = self.signer.sign(url, body)?;
    Ok((req.header("X-API-Key", self.signer.api_key()).bearer_auth(jwt), String::new()))
  }
}

impl FireblocksFactory for FireblocksHttpClient {
  async fn create_address(&self, vault_id: i32, asset_id: &str) -> Result<CreateAddressResponse> {
    let p = format!("vault/accounts/{vault_id}/{asset_id}");
    let (u, _) = self.build_uri(&p, None)?;
    self.post(u).await
  }

  async fn addresses(&self, vault_id: i32, asset_id: &str) -> Result<Vec<Address>> {
    let p = format!("vault/accounts/{vault_id}/{asset_id}/addresses");
    let (u, _) = self.build_uri(&p, None)?;
    self.get(u).await
  }

  async fn addresses_paginated(
    &self,
    vault_id: i32,
    asset_id: &str,
    paging: Option<&PagingVaultRequest>,
  ) -> Result<AddressContainer> {
    let p = format!("vault/accounts/{vault_id}/{asset_id}/addresses_paginated");
    let (u, _) = self.build_uri(&p, paging)?;
    self.get(u).await
  }

  async fn vault(&self, vault_id: i32) -> Result<Account> {
    let p = format!("vault/accounts/{vault_id}");
    let (u, _) = self.build_uri(&p, None)?;
    self.get(u).await
  }

  async fn vaults(
    &self,
    page: Option<&PagingVaultRequest>,
    min_threshold: Option<&BigDecimal>,
  ) -> Result<VaultAccounts> {
    let (mut u, _) = self.build_uri("vault/accounts_paged", page)?;
    if let Some(min) = min_threshold {
      u.query_pairs_mut().append_pair("minAmountThreshold", &format!("{min}"));
    }
    self.get(u).await
  }

  async fn create_vault(&self, account: &CreateAccount) -> Result<Account> {
    let (u, _) = self.build_uri("vault/accounts", None)?;
    self.post_body(u, account).await
  }

  async fn assets(&self, page: Option<&PagingVaultRequest>, greater_than: Option<f64>) -> Result<PaginatedAssetWallet> {
    let (mut u, _) = self.build_uri("vault/asset_wallets?limit=1000", page)?;
    if let Some(greater) = greater_than {
      u.query_pairs_mut().append_pair("totalAmountLargerThan", &format!("{greater}"));
    }
    self.get(u).await
  }

  async fn external_wallets(&self) -> Result<Vec<WalletContainer>> {
    let (u, _) = self.build_uri("external_wallets", None)?;
    self.get(u).await
  }

  async fn external_wallet(&self, id: &str) -> Result<WalletContainer> {
    let (u, _) = self.build_uri(&format!("external_wallets/{id}"), None)?;
    self.get(u).await
  }

  async fn external_wallet_asset(&self, id: &str, asset: &str, address: &str) -> Result<WalletCreateAssetResponse> {
    let (u, _) = self.build_uri(&format!("external_wallets/{id}/{asset}"), None)?;
    let w = WalletCreateAsset { address: String::from(address), tag: "fireblocks-sdk-rs".to_string() };
    self.post_body(u, w).await
  }

  async fn external_wallet_create(&self, name: &str) -> Result<WalletContainer> {
    let (u, _) = self.build_uri("external_wallets", None)?;
    let w = WalletCreate { name: String::from(name) };
    self.post_body(u, w).await
  }

  async fn external_wallet_delete(&self, id: &str) -> Result<()> {
    let (u, _) = self.build_uri(&format!("external_wallets/{id}"), None)?;
    self.delete(u).await
  }

  async fn contracts(&self) -> Result<Vec<WalletContainer>> {
    let (u, _) = self.build_uri("contracts", None)?;
    self.get(u).await
  }

  async fn contract(&self, id: &str) -> Result<WalletContainer> {
    let (u, _) = self.build_uri(&format!("contracts/{id}"), None)?;
    self.get(u).await
  }

  async fn contract_asset(&self, id: &str, asset: &str, address: &str) -> Result<WalletCreateAssetResponse> {
    let (u, _) = self.build_uri(&format!("contracts/{id}/{asset}"), None)?;
    let w = WalletCreateAsset { address: String::from(address), tag: "fireblocks-sdk-rs".to_string() };
    self.post_body(u, w).await
  }

  async fn contract_create(&self, name: &str) -> Result<WalletContainer> {
    let (u, _) = self.build_uri("contracts", None)?;
    let w = WalletCreate { name: String::from(name) };
    self.post_body(u, w).await
  }

  async fn contract_delete(&self, id: &str) -> Result<()> {
    let (u, _) = self.build_uri(&format!("contracts/{id}"), None)?;
    self.delete(u).await
  }

  async fn internal_wallets(&self) -> Result<Vec<WalletContainer>> {
    let (u, _) = self.build_uri("internal_wallets", None)?;
    self.get(u).await
  }

  async fn supported_assets(&self) -> Result<Vec<SupportedAsset>> {
    let (u, _) = self.build_uri("supported_assets", None)?;
    self.get(u).await
  }

  async fn staking_chains(&self) -> Result<Vec<String>> {
    let (u, _) = self.build_uri("staking/chains", None)?;
    self.get(u).await
  }

  async fn staking_positions(&self) -> Result<Vec<StakingPosition>> {
    let (u, _) = self.build_uri("staking/positions", None)?;
    self.get(u).await
  }

  async fn staking_positions_summary(&self) -> Result<StakingPositionsSummary> {
    let (u, _) = self.build_uri("staking/positions/summary", None)?;
    self.get(u).await
  }

  async fn estimate_fee(&self, asset: &str) -> Result<EstimateFee> {
    let (u, _) = self.build_uri(&format!("estimate_network_fee?assetId={asset}"), None)?;
    self.get(u).await
  }

  async fn transactions(&self, options: &TransactionListOptions) -> Result<Vec<Transaction>> {
    let (mut u, _) = self.build_uri("transactions", None)?;
    u.query_pairs_mut().extend_pairs(options.params());
    self.get(u).await
  }

  async fn create_transaction(&self, args: &TransactionArguments) -> Result<CreateTransactionResponse> {
    let (u, _) = self.build_uri("transactions", None)?;
    self.post_body(u, args).await
  }

  async fn get_transaction(&self, id: &str) -> Result<Transaction> {
    let (u, _) = self.build_uri(&format!("transactions/{id}"), None)?;
    self.get(u).await
  }

  async fn wallet_connections(&self) -> Result<PagedWalletConnectResponse> {
    let (u, _) = self.build_uri("connections", None)?;
    self.get(u).await
  }

  async fn wallet_connection_delete(&self, id: &str) -> Result<()> {
    let u = self.build_uri(&format!("connections/wc/{id}"), None)?.0;
    self.delete(u).await
  }
}
