use {
    super::Client,
    crate::{
        apis::{
            whitelisted_contracts_api::{
                AddContractAssetParams,
                CreateContractParams,
                DeleteContractParams,
                GetContractParams,
            },
            whitelisted_external_wallets_api::{
                AddAssetToExternalWalletParams,
                CreateExternalWalletParams,
                DeleteExternalWalletParams,
                GetExternalWalletParams,
            },
            whitelisted_internal_wallets_api::{
                CreateInternalWalletAssetParams,
                CreateInternalWalletParams,
                DeleteInternalWalletParams,
                GetInternalWalletParams,
            },
            Api,
        },
        error::FireblocksError,
        models::{
            AddAssetToExternalWalletRequest,
            AddAssetToExternalWalletRequestOneOf,
            CreateContractRequest,
            CreateInternalWalletAssetRequest,
            CreateWalletRequest,
        },
        WalletContainer,
        WalletType,
    },
};

impl Client {
    pub async fn wallet_create_asset(
        &self,
        wallet_type: WalletType,
        id: &str,
        asset_id: impl Into<String>,
        address: &str,
    ) -> crate::Result<String> {
        let asset_id = asset_id.into();
        let id: String = match wallet_type {
            WalletType::External => {
                let api = self.api_client.whitelisted_external_wallets_api();
                let params = AddAssetToExternalWalletParams::builder()
                    .asset_id(asset_id)
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
            }
            WalletType::Internal => {
                let api = self.api_client.whitelisted_internal_wallets_api();
                let a = CreateInternalWalletAssetRequest::new(String::from(address));
                let params = CreateInternalWalletAssetParams::builder()
                    .asset_id(asset_id)
                    .wallet_id(String::from(id))
                    .create_internal_wallet_asset_request(a)
                    .build();
                api.create_internal_wallet_asset(params)
                    .await
                    .map_err(|e| FireblocksError::FetchWalletCreateError(e.to_string()))?
                    .id
            }
            WalletType::Contract => {
                let api = self.api_client.whitelisted_contracts_api();
                let params = AddContractAssetParams::builder()
                    .asset_id(asset_id)
                    .contract_id(String::from(address))
                    .build();
                api.add_contract_asset(params)
                    .await
                    .map_err(|e| FireblocksError::FetchWalletCreateError(e.to_string()))?
                    .id
                    .unwrap_or_default()
            }
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

    pub async fn wallet_by_name(
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

    pub async fn wallet_by_id(
        &self,
        wallet_type: WalletType,
        id: &str,
    ) -> crate::Result<WalletContainer> {
        let w: WalletContainer = match wallet_type {
            WalletType::External => {
                let w = self
                    .wallet_external_api()
                    .get_external_wallet(GetExternalWalletParams {
                        wallet_id: String::from(id),
                    })
                    .await
                    .map_err(|e| FireblocksError::FetchWalletError(e.to_string()))?;
                WalletContainer::from(w)
            }
            WalletType::Internal => {
                let w = self
                    .wallet_internal_api()
                    .get_internal_wallet(GetInternalWalletParams {
                        wallet_id: String::from(id),
                    })
                    .await
                    .map_err(|e| FireblocksError::FetchWalletError(e.to_string()))?;
                WalletContainer::from(w)
            }
            WalletType::Contract => {
                let w = self
                    .wallet_contract_api()
                    .get_contract(GetContractParams {
                        contract_id: String::from(id),
                    })
                    .await
                    .map_err(|e| FireblocksError::FetchWalletError(e.to_string()))?;
                WalletContainer::from(w)
            }
        };
        Ok(w)
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
