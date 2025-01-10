use {
    super::Client,
    crate::{
        apis::{
            vaults_api::{
                CreateVaultAccountAssetAddressParams,
                CreateVaultAccountParams,
                GetVaultAccountParams,
            },
            Api,
        },
        models::{CreateVaultAccountRequest, VaultAccount},
        FireblocksError,
    },
};

impl Client {
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

    pub async fn create_vault(
        &self,
        params: CreateVaultAccountRequest,
    ) -> crate::Result<VaultAccount> {
        let vault_api = self.api_client.vaults_api();
        let params = CreateVaultAccountParams::builder()
            .create_vault_account_request(params)
            .build();
        vault_api
            .create_vault_account(params)
            .await
            .map_err(|e| FireblocksError::FetchVaultCreateError(e.to_string()))
    }

    pub async fn create_asset(
        &self,
        vault_id: &str,
        asset_id: impl Into<String>,
    ) -> crate::Result<String> {
        let api = self.api_client.vaults_api();
        let params = CreateVaultAccountAssetAddressParams::builder()
            .asset_id(asset_id.into())
            .vault_account_id(String::from(vault_id))
            .build();
        api.create_vault_account_asset_address(params)
            .await
            .map_err(crate::FireblocksError::FetchCreateAssetError)
            .map(|r| r.address.unwrap_or_default())
    }
}
