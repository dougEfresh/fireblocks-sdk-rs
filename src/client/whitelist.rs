use {
    super::Client,
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

impl Client {
    pub async fn wallet_create_asset(
        &self,
        wallet_type: WalletType,
        id: &str,
        asset_id: &str,
        address: &str,
    ) -> crate::Result<String> {
        let id: String = match wallet_type {
            WalletType::External => {
                let api = self.api_client.whitelisted_external_wallets_api();
                let params = AddAssetToExternalWalletParams::builder()
                    .asset_id(String::from(asset_id))
                    .wallet_id(String::from(id))
                    .add_asset_to_external_wallet_request(
                        AddAssetToExternalWalletRequest::AddAssetToExternalWalletRequestOneOf(
                            AddAssetToExternalWalletRequestOneOf {
                                address: String::from(address),
                                tag: None,
                            },
                        ),
                    )
                    .build();
                api.add_asset_to_external_wallet(params)
                    .await
                    .map_err(|e| FireblocksError::FetchWalletCreateError(e.to_string()))?
                    .id
                    .unwrap_or_default()
            }
            WalletType::Internal => {
                let api = self.api_client.whitelisted_internal_wallets_api();
                let a = CreateInternalWalletAssetRequest::new(String::from(address));
                let params = CreateInternalWalletAssetParams::builder()
                    .asset_id(String::from(asset_id))
                    .wallet_id(String::from(id))
                    .create_internal_wallet_asset_request(a)
                    .build();
                api.create_internal_wallet_asset(params)
                    .await
                    .map_err(|e| FireblocksError::FetchWalletCreateError(e.to_string()))?
                    .id
                    .unwrap_or_default()
            }
            WalletType::Contract => String::new(),
        };
        Ok(id)
    }

    pub async fn wallet_delete(&self, wallet_type: WalletType, id: &str) -> crate::Result<()> {
        match wallet_type {
            WalletType::External => {
                let api = self.api_client.whitelisted_external_wallets_api();
                let params = DeleteExternalWalletParams::builder()
                    .wallet_id(String::from(id))
                    .build();
                api.delete_external_wallet(params)
                    .await
                    .map_err(|e| FireblocksError::FetchWalletCreateError(e.to_string()))?;
            }
            WalletType::Internal => {
                let api = self.api_client.whitelisted_internal_wallets_api();
                let params = DeleteInternalWalletParams::builder()
                    .wallet_id(String::from(id))
                    .build();
                api.delete_internal_wallet(params)
                    .await
                    .map_err(|e| FireblocksError::FetchWalletCreateError(e.to_string()))?;
            }
            WalletType::Contract => {
                let api = self.api_client.whitelisted_contracts_api();
                let params = DeleteContractParams::builder()
                    .contract_id(String::from(id))
                    .build();
                api.delete_contract(params)
                    .await
                    .map_err(|e| FireblocksError::FetchWalletCreateError(e.to_string()))?;
            }
        };
        Ok(())
    }

    pub async fn wallet_create(
        &self,
        wallet_type: WalletType,
        name: &str,
    ) -> crate::Result<String> {
        let id: String = match wallet_type {
            // TransferPeerPathType::Contract => String::new(),
            WalletType::External => {
                let api = self.api_client.whitelisted_external_wallets_api();
                let params = CreateExternalWalletParams::builder()
                    .create_wallet_request(CreateWalletRequest {
                        name: Some(String::from(name)),
                        customer_ref_id: None,
                    })
                    .build();
                api.create_external_wallet(params)
                    .await
                    .map_err(|e| FireblocksError::FetchWalletCreateError(e.to_string()))?
                    .id
            }
            WalletType::Internal => {
                let api = self.api_client.whitelisted_internal_wallets_api();
                let params = CreateInternalWalletParams::builder()
                    .create_wallet_request(CreateWalletRequest {
                        name: Some(String::from(name)),
                        customer_ref_id: None,
                    })
                    .build();
                api.create_internal_wallet(params)
                    .await
                    .map_err(|e| FireblocksError::FetchWalletCreateError(e.to_string()))?
                    .id
                    .unwrap_or_default()
            }
            WalletType::Contract => {
                let api = self.api_client.whitelisted_contracts_api();
                let params = CreateContractParams::builder()
                    .create_contract_request(CreateContractRequest {
                        name: Some(String::from(name)),
                    })
                    .build();
                api.create_contract(params)
                    .await
                    .map_err(|e| FireblocksError::FetchWalletCreateError(e.to_string()))?
                    .id
                    .unwrap_or_default()
            }
        };
        Ok(id)
    }

    pub async fn wallet_name(
        &self,
        wallet_type: WalletType,
        name: &str,
    ) -> crate::Result<Option<WalletContainer>> {
        Ok(self
            .wallets(wallet_type)
            .await?
            .into_iter()
            .find(|w| w.name == name))
    }

    pub async fn wallets(&self, wallet_type: WalletType) -> crate::Result<Vec<WalletContainer>> {
        let wallets: Vec<WalletContainer> = match wallet_type {
            WalletType::Internal => {
                let api = self.api_client.whitelisted_internal_wallets_api();
                let wallets = api
                    .get_internal_wallets()
                    .await
                    .map_err(|e| FireblocksError::FetchWalletInternalError(e.to_string()))?;
                wallets.into_iter().map(WalletContainer::from).collect()
            }
            WalletType::External => {
                let api = self.api_client.whitelisted_external_wallets_api();
                let wallets = api
                    .get_external_wallets()
                    .await
                    .map_err(|e| FireblocksError::FetchWalletExternalError(e.to_string()))?;
                wallets.into_iter().map(WalletContainer::from).collect()
            }
            WalletType::Contract => {
                let api = self.api_client.whitelisted_contracts_api();
                let wallets = api
                    .get_contracts()
                    .await
                    .map_err(|e| FireblocksError::FetchWalletContractError(e.to_string()))?;
                wallets.into_iter().map(WalletContainer::from).collect()
            }
        };
        Ok(wallets)
    }
}
