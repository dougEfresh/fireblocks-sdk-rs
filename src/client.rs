use {
    crate::{
        apis::{
            d_app_connections_api::DAppConnectionsApi,
            transactions_api::{GetTransactionParams, TransactionsApi},
            vaults_api::{
                CreateVaultAccountAssetAddressParams,
                GetVaultAccountAssetAddressesPaginatedParams,
                GetVaultAccountParams,
                VaultsApi,
            },
            whitelisted_contracts_api::WhitelistedContractsApi,
            whitelisted_external_wallets_api::WhitelistedExternalWalletsApi,
            whitelisted_internal_wallets_api::WhitelistedInternalWalletsApi,
            Api,
        },
        error::{self},
        jwt::{JwtSigningMiddleware, Signer},
        models::{AssetTypeResponse, TransactionResponse, VaultAccount, VaultWalletAddress},
        ApiClient,
        Configuration,
        FIREBLOCKS_API,
        FIREBLOCKS_SANDBOX_API,
    },
    jsonwebtoken::EncodingKey,
    std::{sync::Arc, time::Duration},
};

#[derive(Clone)]
pub struct Client {
    api_client: Arc<ApiClient>,
}

mod poll;
mod transfer;
mod whitelist;
pub struct ClientBuilder {
    api_key: String,
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

    pub fn build(self) -> Result<Client, error::FireblocksError> {
        let key = EncodingKey::from_rsa_pem(&self.secret[..])?;
        let signer = Signer::new(key, &self.api_key);
        let jwt_handler = JwtSigningMiddleware::new(signer);
        let r = reqwest::ClientBuilder::new()
            .timeout(self.timeout)
            .connect_timeout(self.connect_timeout)
            .user_agent(String::from(&self.user_agent))
            .build()
            .unwrap_or_default();
        let client = reqwest_middleware::ClientBuilder::new(r)
            .with(crate::log::LoggingMiddleware)
            .with(jwt_handler);
        Ok(Client::new_with_url(
            &self.url,
            client.build(),
            Some(self.user_agent),
        ))
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
        asset_id: impl Into<String>,
    ) -> crate::Result<Vec<VaultWalletAddress>> {
        let vault_api = self.api_client.vaults_api();
        let params = GetVaultAccountAssetAddressesPaginatedParams::builder()
            .vault_account_id(String::from(vault_id))
            .asset_id(asset_id.into())
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
}
