use {
    super::Client,
    crate::{
        apis::{transactions_api::CreateTransactionParams, Api},
        error::FireblocksError,
        models::{
            transaction_request::FeeLevel,
            CreateTransactionResponse,
            DestinationTransferPeerPath,
            SourceTransferPeerPath,
            TransactionRequest,
            TransferPeerPathType,
        },
        WalletType,
    },
};

impl Client {
    pub async fn vault_whitelist_transfer(
        &self,
        vault_id: &str,
        asset_id: impl Into<String>,
        amount: impl Into<String>,
        dest_type: WalletType,
        dest_id: &str,
        fee_level: Option<FeeLevel>,
    ) -> crate::Result<CreateTransactionResponse> {
        let api = self.api_client.transactions_api();
        let mut req = TransactionRequest::default();
        let peer: TransferPeerPathType = match dest_type {
            WalletType::Internal => TransferPeerPathType::InternalWallet,
            WalletType::External => TransferPeerPathType::ExternalWallet,
            WalletType::Contract => TransferPeerPathType::Contract,
        };
        if fee_level.is_some() {
            req.fee_level = req.fee_level
        }
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
            id: Some(uuid.to_string()),
            name: None,
            wallet_id: None,
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
