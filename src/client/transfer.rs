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
    pub async fn vault_whitelist_transfer(
        &self,
        vault_id: &str,
        asset_id: impl Into<String>,
        amount: impl Into<String>,
        dest_type: WalletType,
        dest_id: &str,
    ) -> crate::Result<CreateTransactionResponse> {
        let api = self.api_client.transactions_api();
        let mut req = TransactionRequest::default();
        let peer: TransferPeerPathType = match dest_type {
            WalletType::Internal => TransferPeerPathType::InternalWallet,
            WalletType::External => TransferPeerPathType::ExternalWallet,
            WalletType::Contract => TransferPeerPathType::Contract,
        };
        req.source = Some(SourceTransferPeerPath {
            r#type: TransferPeerPathType::VaultAccount,
            sub_type: None,
            id: Some(String::from(vault_id)),
            name: None,
            wallet_id: None,
        });
        let uuid = uuid::Uuid::try_from(dest_id)?;
        req.destination = Some(DestinationTransferPeerPath {
            r#type: peer,
            sub_type: None,
            id: None,
            name: None,
            wallet_id: Some(uuid),
            one_time_address: None,
        });
        req.asset_id = Some(asset_id.into());
        req.amount = Some(crate::models::TransactionRequestAmount::String(
            amount.into(),
        ));
        let params = CreateTransactionParams::builder()
            .transaction_request(req)
            .build();
        let response = api
            .create_transaction(params)
            .await
            .map_err(|e| FireblocksError::FetchCreateTransactionError(e.to_string()))?;

        Ok(response)
    }
}
