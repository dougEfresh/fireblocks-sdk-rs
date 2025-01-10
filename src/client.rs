use {
    crate::{
        apis::{
            d_app_connections_api::DAppConnectionsApi,
            transactions_api::{CreateTransactionParams, GetTransactionParams, TransactionsApi},
            vaults_api::{
                CreateVaultAccountAssetAddressParams,
                CreateVaultAccountAssetParams,
                GetVaultAccountAssetAddressesPaginatedParams,
                GetVaultAccountParams,
                VaultsApi,
            },
            whitelisted_contracts_api::{
                CreateContractParams,
                DeleteContractParams,
                WhitelistedContractsApi,
            },
            whitelisted_external_wallets_api::{
                AddAssetToExternalWalletParams,
                CreateExternalWalletParams,
                DeleteExternalWalletParams,
                WhitelistedExternalWalletsApi,
            },
            whitelisted_internal_wallets_api::{
                CreateInternalWalletAssetParams,
                CreateInternalWalletParams,
                DeleteInternalWalletParams,
                WhitelistedInternalWalletsApi,
            },
            Api,
        },
        error::{self, FireblocksError},
        jwt::{JwtSigningMiddleware, Signer},
        models::{
            AddAssetToExternalWalletRequest,
            AddAssetToExternalWalletRequestOneOf,
            AssetTypeResponse,
            CreateContractRequest,
            CreateInternalWalletAssetRequest,
            CreateTransactionResponse,
            CreateWalletRequest,
            DestinationTransferPeerPath,
            SourceTransferPeerPath,
            Transaction,
            TransactionRequest,
            TransactionResponse,
            TransactionStatus,
            TransferPeerPathType,
            VaultAccount,
            VaultWalletAddress,
        },
        ApiClient,
        Configuration,
        WalletContainer,
        WalletType,
        FIREBLOCKS_API,
        FIREBLOCKS_SANDBOX_API,
    },
    jsonwebtoken::EncodingKey,
    reqwest::{Method, RequestBuilder, StatusCode},
    serde::{de::DeserializeOwned, Serialize},
    std::{
        borrow::Borrow,
        fmt::{Debug, Display},
        ops::Add,
        sync::Arc,
        time::Duration,
    },
    tokio::time,
    tracing::debug,
    url::Url,
};

#[derive(Clone)]
pub struct Client {
    api_client: Arc<ApiClient>,
}

mod transfer;
mod whitelist;

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
    pub async fn get_transaction(&self, id: &str) -> crate::Result<TransactionResponse> {
        let api = self.api_client.transactions_api();
        api.get_transaction(
            GetTransactionParams::builder()
                .tx_id(String::from(id))
                .build(),
        )
        .await
        .map_err(crate::FireblocksError::FetchTransactionError)
    }

    pub async fn create_asset(&self, vault_id: &str, asset_id: &str) -> crate::Result<String> {
        let api = self.api_client.vaults_api();
        let params = CreateVaultAccountAssetAddressParams::builder()
            .asset_id(String::from(asset_id))
            .vault_account_id(String::from(vault_id))
            .build();
        api.create_vault_account_asset_address(params)
            .await
            .map_err(crate::FireblocksError::FetchCreateAssetError)
            .map(|r| r.address.unwrap_or_default())
    }

    pub async fn supported_assets(&self) -> crate::Result<Vec<AssetTypeResponse>> {
        let api = self.api_client.blockchains_assets_api();
        api.get_supported_assets()
            .await
            .map_err(crate::FireblocksError::FetchSupportedAssetsError)
    }

    pub async fn addresses(
        &self,
        vault_id: &str,
        asset_id: &str,
    ) -> crate::Result<Vec<VaultWalletAddress>> {
        let vault_api = self.api_client.vaults_api();
        let params = GetVaultAccountAssetAddressesPaginatedParams::builder()
            .vault_account_id(String::from(vault_id))
            .asset_id(String::from(asset_id))
            .build();

        vault_api
            .get_vault_account_asset_addresses_paginated(params)
            .await
            .map_err(crate::FireblocksError::FetchAddressesError)
            .map(|r| r.addresses)
    }

    pub async fn vault(&self, id: &str) -> crate::Result<VaultAccount> {
        let vault_api = self.api_client.vaults_api();
        vault_api
            .get_vault_account(
                GetVaultAccountParams::builder()
                    .vault_account_id(String::from(id))
                    .build(),
            )
            .await
            .map_err(crate::FireblocksError::FetchVaultAccountError)
    }

    pub fn transactions_api(&self) -> &dyn TransactionsApi {
        self.api_client.transactions_api()
    }

    pub fn vaults_api(&self) -> &dyn VaultsApi {
        self.api_client.vaults_api()
    }

    pub fn wallet_connect_api(&self) -> &dyn DAppConnectionsApi {
        self.api_client.d_app_connections_api()
    }

    pub fn wallet_internal_api(&self) -> &dyn WhitelistedInternalWalletsApi {
        self.api_client.whitelisted_internal_wallets_api()
    }

    pub fn wallet_external_api(&self) -> &dyn WhitelistedExternalWalletsApi {
        self.api_client.whitelisted_external_wallets_api()
    }

    pub fn wallet_contract_api(&self) -> &dyn WhitelistedContractsApi {
        self.api_client.whitelisted_contracts_api()
    }

    pub fn apis(&self) -> Arc<ApiClient> {
        self.api_client.clone()
    }

    /// Pool transaction until
    /// * [`TransactionStatus::Failed`]
    /// * [`TransactionStatus::Completed`]
    /// * [`TransactionStatus::Blocked`]
    /// * [`TransactionStatus::Rejected`]
    /// * [`TransactionStatus::Cancelling`]
    /// * [`TransactionStatus::Cancelled`]
    ///
    /// [getTransaction](https://docs.fireblocks.com/api/swagger-ui/#/Transactions/getTransaction)
    #[tracing::instrument(level = "debug", skip(self, callback))]
    pub async fn poll_transaction(
        &self,
        id: &str,
        timeout: time::Duration,
        interval: time::Duration,
        callback: impl Fn(&TransactionResponse) + Send + Sync,
    ) -> crate::Result<TransactionResponse> {
        let mut total_time = time::Duration::from_millis(0);
        loop {
            if let Ok(result) = self.get_transaction(id).await {
                let status = &result.status;
                debug!("status {:#?}", status);
                #[allow(clippy::match_same_arms)]
                match status {
                    TransactionStatus::Blocked => break,
                    TransactionStatus::Cancelled => break,
                    TransactionStatus::Cancelling => break,
                    TransactionStatus::Completed => break,
                    TransactionStatus::Confirming => break,
                    TransactionStatus::Failed => break,
                    TransactionStatus::Rejected => break,
                    _ => {
                        callback(&result);
                    }
                }
            }
            time::sleep(interval).await;
            total_time = total_time.add(interval);
            if total_time > timeout {
                break;
            }
        }
        self.get_transaction(id).await
    }
}
