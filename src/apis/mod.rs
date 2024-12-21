use std::{error, fmt};

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    ReqwestMiddleware(reqwest_middleware::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}

impl<T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Reqwest(e) => ("reqwest", e.to_string()),
            Error::ReqwestMiddleware(e) => ("reqwest-middleware", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::Io(e) => ("IO", e.to_string()),
            Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl<T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Reqwest(e) => e,
            Error::ReqwestMiddleware(e) => e,
            Error::Serde(e) => e,
            Error::Io(e) => e,
            Error::ResponseError(_) => return None,
        })
    }
}

impl<T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl<T> From<reqwest_middleware::Error> for Error<T> {
    fn from(e: reqwest_middleware::Error) -> Self {
        Error::ReqwestMiddleware(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl<T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub fn parse_deep_object(prefix: &str, value: &serde_json::Value) -> Vec<(String, String)> {
    if let serde_json::Value::Object(object) = value {
        let mut params = vec![];

        for (key, value) in object {
            match value {
                serde_json::Value::Object(_) => params.append(&mut parse_deep_object(
                    &format!("{}[{}]", prefix, key),
                    value,
                )),
                serde_json::Value::Array(array) => {
                    for (i, value) in array.iter().enumerate() {
                        params.append(&mut parse_deep_object(
                            &format!("{}[{}][{}]", prefix, key, i),
                            value,
                        ));
                    }
                }
                serde_json::Value::String(s) => {
                    params.push((format!("{}[{}]", prefix, key), s.clone()))
                }
                _ => params.push((format!("{}[{}]", prefix, key), value.to_string())),
            }
        }

        return params;
    }

    unimplemented!("Only objects are supported with style=deepObject")
}

pub mod blockchains_assets_api;
pub mod compliance_api;
pub mod contract_interactions_api;
pub mod contract_templates_api;
pub mod cosigners_beta_api;
pub mod d_app_connections_api;
pub mod deployed_contracts_api;
pub mod exchange_accounts_api;
pub mod fiat_accounts_api;
pub mod fireblocks_network_api;
pub mod gas_station_api;
pub mod key_link_beta_api;
pub mod nfts_api;
pub mod off_exchange_api;
pub mod payments_payout_api;
pub mod policy_editor_beta_api;
pub mod smart_transfers_api;
pub mod staking_api;
pub mod tokenization_api;
pub mod transactions_api;
pub mod vaults_api;
pub mod webhooks_api;
pub mod whitelisted_contracts_api;
pub mod whitelisted_external_wallets_api;
pub mod whitelisted_internal_wallets_api;
pub mod workspace_management_api;

pub mod configuration;

use std::sync::Arc;

pub trait Api {
    fn blockchains_assets_api(&self) -> &dyn blockchains_assets_api::BlockchainsAssetsApi;
    fn compliance_api(&self) -> &dyn compliance_api::ComplianceApi;
    fn contract_interactions_api(&self) -> &dyn contract_interactions_api::ContractInteractionsApi;
    fn contract_templates_api(&self) -> &dyn contract_templates_api::ContractTemplatesApi;
    fn cosigners_beta_api(&self) -> &dyn cosigners_beta_api::CosignersBetaApi;
    fn d_app_connections_api(&self) -> &dyn d_app_connections_api::DAppConnectionsApi;
    fn deployed_contracts_api(&self) -> &dyn deployed_contracts_api::DeployedContractsApi;
    fn exchange_accounts_api(&self) -> &dyn exchange_accounts_api::ExchangeAccountsApi;
    fn fiat_accounts_api(&self) -> &dyn fiat_accounts_api::FiatAccountsApi;
    fn fireblocks_network_api(&self) -> &dyn fireblocks_network_api::FireblocksNetworkApi;
    fn gas_station_api(&self) -> &dyn gas_station_api::GasStationApi;
    fn key_link_beta_api(&self) -> &dyn key_link_beta_api::KeyLinkBetaApi;
    fn nfts_api(&self) -> &dyn nfts_api::NftsApi;
    fn off_exchange_api(&self) -> &dyn off_exchange_api::OffExchangeApi;
    fn payments_payout_api(&self) -> &dyn payments_payout_api::PaymentsPayoutApi;
    fn policy_editor_beta_api(&self) -> &dyn policy_editor_beta_api::PolicyEditorBetaApi;
    fn smart_transfers_api(&self) -> &dyn smart_transfers_api::SmartTransfersApi;
    fn staking_api(&self) -> &dyn staking_api::StakingApi;
    fn tokenization_api(&self) -> &dyn tokenization_api::TokenizationApi;
    fn transactions_api(&self) -> &dyn transactions_api::TransactionsApi;
    fn vaults_api(&self) -> &dyn vaults_api::VaultsApi;
    fn webhooks_api(&self) -> &dyn webhooks_api::WebhooksApi;
    fn whitelisted_contracts_api(&self) -> &dyn whitelisted_contracts_api::WhitelistedContractsApi;
    fn whitelisted_external_wallets_api(
        &self,
    ) -> &dyn whitelisted_external_wallets_api::WhitelistedExternalWalletsApi;
    fn whitelisted_internal_wallets_api(
        &self,
    ) -> &dyn whitelisted_internal_wallets_api::WhitelistedInternalWalletsApi;
    fn workspace_management_api(&self) -> &dyn workspace_management_api::WorkspaceManagementApi;
}

pub struct ApiClient {
    blockchains_assets_api: Box<dyn blockchains_assets_api::BlockchainsAssetsApi>,
    compliance_api: Box<dyn compliance_api::ComplianceApi>,
    contract_interactions_api: Box<dyn contract_interactions_api::ContractInteractionsApi>,
    contract_templates_api: Box<dyn contract_templates_api::ContractTemplatesApi>,
    cosigners_beta_api: Box<dyn cosigners_beta_api::CosignersBetaApi>,
    d_app_connections_api: Box<dyn d_app_connections_api::DAppConnectionsApi>,
    deployed_contracts_api: Box<dyn deployed_contracts_api::DeployedContractsApi>,
    exchange_accounts_api: Box<dyn exchange_accounts_api::ExchangeAccountsApi>,
    fiat_accounts_api: Box<dyn fiat_accounts_api::FiatAccountsApi>,
    fireblocks_network_api: Box<dyn fireblocks_network_api::FireblocksNetworkApi>,
    gas_station_api: Box<dyn gas_station_api::GasStationApi>,
    key_link_beta_api: Box<dyn key_link_beta_api::KeyLinkBetaApi>,
    nfts_api: Box<dyn nfts_api::NftsApi>,
    off_exchange_api: Box<dyn off_exchange_api::OffExchangeApi>,
    payments_payout_api: Box<dyn payments_payout_api::PaymentsPayoutApi>,
    policy_editor_beta_api: Box<dyn policy_editor_beta_api::PolicyEditorBetaApi>,
    smart_transfers_api: Box<dyn smart_transfers_api::SmartTransfersApi>,
    staking_api: Box<dyn staking_api::StakingApi>,
    tokenization_api: Box<dyn tokenization_api::TokenizationApi>,
    transactions_api: Box<dyn transactions_api::TransactionsApi>,
    vaults_api: Box<dyn vaults_api::VaultsApi>,
    webhooks_api: Box<dyn webhooks_api::WebhooksApi>,
    whitelisted_contracts_api: Box<dyn whitelisted_contracts_api::WhitelistedContractsApi>,
    whitelisted_external_wallets_api:
        Box<dyn whitelisted_external_wallets_api::WhitelistedExternalWalletsApi>,
    whitelisted_internal_wallets_api:
        Box<dyn whitelisted_internal_wallets_api::WhitelistedInternalWalletsApi>,
    workspace_management_api: Box<dyn workspace_management_api::WorkspaceManagementApi>,
}

impl ApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self {
            blockchains_assets_api: Box::new(
                blockchains_assets_api::BlockchainsAssetsApiClient::new(configuration.clone()),
            ),
            compliance_api: Box::new(compliance_api::ComplianceApiClient::new(
                configuration.clone(),
            )),
            contract_interactions_api: Box::new(
                contract_interactions_api::ContractInteractionsApiClient::new(
                    configuration.clone(),
                ),
            ),
            contract_templates_api: Box::new(
                contract_templates_api::ContractTemplatesApiClient::new(configuration.clone()),
            ),
            cosigners_beta_api: Box::new(cosigners_beta_api::CosignersBetaApiClient::new(
                configuration.clone(),
            )),
            d_app_connections_api: Box::new(d_app_connections_api::DAppConnectionsApiClient::new(
                configuration.clone(),
            )),
            deployed_contracts_api: Box::new(
                deployed_contracts_api::DeployedContractsApiClient::new(configuration.clone()),
            ),
            exchange_accounts_api: Box::new(exchange_accounts_api::ExchangeAccountsApiClient::new(
                configuration.clone(),
            )),
            fiat_accounts_api: Box::new(fiat_accounts_api::FiatAccountsApiClient::new(
                configuration.clone(),
            )),
            fireblocks_network_api: Box::new(
                fireblocks_network_api::FireblocksNetworkApiClient::new(configuration.clone()),
            ),
            gas_station_api: Box::new(gas_station_api::GasStationApiClient::new(
                configuration.clone(),
            )),
            key_link_beta_api: Box::new(key_link_beta_api::KeyLinkBetaApiClient::new(
                configuration.clone(),
            )),
            nfts_api: Box::new(nfts_api::NftsApiClient::new(configuration.clone())),
            off_exchange_api: Box::new(off_exchange_api::OffExchangeApiClient::new(
                configuration.clone(),
            )),
            payments_payout_api: Box::new(payments_payout_api::PaymentsPayoutApiClient::new(
                configuration.clone(),
            )),
            policy_editor_beta_api: Box::new(
                policy_editor_beta_api::PolicyEditorBetaApiClient::new(configuration.clone()),
            ),
            smart_transfers_api: Box::new(smart_transfers_api::SmartTransfersApiClient::new(
                configuration.clone(),
            )),
            staking_api: Box::new(staking_api::StakingApiClient::new(configuration.clone())),
            tokenization_api: Box::new(tokenization_api::TokenizationApiClient::new(
                configuration.clone(),
            )),
            transactions_api: Box::new(transactions_api::TransactionsApiClient::new(
                configuration.clone(),
            )),
            vaults_api: Box::new(vaults_api::VaultsApiClient::new(configuration.clone())),
            webhooks_api: Box::new(webhooks_api::WebhooksApiClient::new(configuration.clone())),
            whitelisted_contracts_api: Box::new(
                whitelisted_contracts_api::WhitelistedContractsApiClient::new(
                    configuration.clone(),
                ),
            ),
            whitelisted_external_wallets_api: Box::new(
                whitelisted_external_wallets_api::WhitelistedExternalWalletsApiClient::new(
                    configuration.clone(),
                ),
            ),
            whitelisted_internal_wallets_api: Box::new(
                whitelisted_internal_wallets_api::WhitelistedInternalWalletsApiClient::new(
                    configuration.clone(),
                ),
            ),
            workspace_management_api: Box::new(
                workspace_management_api::WorkspaceManagementApiClient::new(configuration.clone()),
            ),
        }
    }
}

impl Api for ApiClient {
    fn blockchains_assets_api(&self) -> &dyn blockchains_assets_api::BlockchainsAssetsApi {
        self.blockchains_assets_api.as_ref()
    }

    fn compliance_api(&self) -> &dyn compliance_api::ComplianceApi {
        self.compliance_api.as_ref()
    }

    fn contract_interactions_api(&self) -> &dyn contract_interactions_api::ContractInteractionsApi {
        self.contract_interactions_api.as_ref()
    }

    fn contract_templates_api(&self) -> &dyn contract_templates_api::ContractTemplatesApi {
        self.contract_templates_api.as_ref()
    }

    fn cosigners_beta_api(&self) -> &dyn cosigners_beta_api::CosignersBetaApi {
        self.cosigners_beta_api.as_ref()
    }

    fn d_app_connections_api(&self) -> &dyn d_app_connections_api::DAppConnectionsApi {
        self.d_app_connections_api.as_ref()
    }

    fn deployed_contracts_api(&self) -> &dyn deployed_contracts_api::DeployedContractsApi {
        self.deployed_contracts_api.as_ref()
    }

    fn exchange_accounts_api(&self) -> &dyn exchange_accounts_api::ExchangeAccountsApi {
        self.exchange_accounts_api.as_ref()
    }

    fn fiat_accounts_api(&self) -> &dyn fiat_accounts_api::FiatAccountsApi {
        self.fiat_accounts_api.as_ref()
    }

    fn fireblocks_network_api(&self) -> &dyn fireblocks_network_api::FireblocksNetworkApi {
        self.fireblocks_network_api.as_ref()
    }

    fn gas_station_api(&self) -> &dyn gas_station_api::GasStationApi {
        self.gas_station_api.as_ref()
    }

    fn key_link_beta_api(&self) -> &dyn key_link_beta_api::KeyLinkBetaApi {
        self.key_link_beta_api.as_ref()
    }

    fn nfts_api(&self) -> &dyn nfts_api::NftsApi {
        self.nfts_api.as_ref()
    }

    fn off_exchange_api(&self) -> &dyn off_exchange_api::OffExchangeApi {
        self.off_exchange_api.as_ref()
    }

    fn payments_payout_api(&self) -> &dyn payments_payout_api::PaymentsPayoutApi {
        self.payments_payout_api.as_ref()
    }

    fn policy_editor_beta_api(&self) -> &dyn policy_editor_beta_api::PolicyEditorBetaApi {
        self.policy_editor_beta_api.as_ref()
    }

    fn smart_transfers_api(&self) -> &dyn smart_transfers_api::SmartTransfersApi {
        self.smart_transfers_api.as_ref()
    }

    fn staking_api(&self) -> &dyn staking_api::StakingApi {
        self.staking_api.as_ref()
    }

    fn tokenization_api(&self) -> &dyn tokenization_api::TokenizationApi {
        self.tokenization_api.as_ref()
    }

    fn transactions_api(&self) -> &dyn transactions_api::TransactionsApi {
        self.transactions_api.as_ref()
    }

    fn vaults_api(&self) -> &dyn vaults_api::VaultsApi {
        self.vaults_api.as_ref()
    }

    fn webhooks_api(&self) -> &dyn webhooks_api::WebhooksApi {
        self.webhooks_api.as_ref()
    }

    fn whitelisted_contracts_api(&self) -> &dyn whitelisted_contracts_api::WhitelistedContractsApi {
        self.whitelisted_contracts_api.as_ref()
    }

    fn whitelisted_external_wallets_api(
        &self,
    ) -> &dyn whitelisted_external_wallets_api::WhitelistedExternalWalletsApi {
        self.whitelisted_external_wallets_api.as_ref()
    }

    fn whitelisted_internal_wallets_api(
        &self,
    ) -> &dyn whitelisted_internal_wallets_api::WhitelistedInternalWalletsApi {
        self.whitelisted_internal_wallets_api.as_ref()
    }

    fn workspace_management_api(&self) -> &dyn workspace_management_api::WorkspaceManagementApi {
        self.workspace_management_api.as_ref()
    }
}
