use std::error;
use std::fmt;

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

impl <T> fmt::Display for Error<T> {
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

impl <T: fmt::Debug> error::Error for Error<T> {
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

impl <T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl<T> From<reqwest_middleware::Error> for Error<T> {
    fn from(e: reqwest_middleware::Error) -> Self {
        Error::ReqwestMiddleware(e)
    }
}

impl <T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl <T> From<std::io::Error> for Error<T> {
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
                },
                serde_json::Value::String(s) => params.push((format!("{}[{}]", prefix, key), s.clone())),
                _ => params.push((format!("{}[{}]", prefix, key), value.to_string())),
            }
        }

        return params;
    }

    unimplemented!("Only objects are supported with style=deepObject")
}

pub mod audit_logs_api;
pub mod blockchains_assets_api;
pub mod contracts_api;
pub mod exchange_accounts_api;
pub mod external_wallets_api;
pub mod fiat_accounts_api;
pub mod gas_stations_api;
pub mod internal_wallets_api;
pub mod nfts_api;
pub mod network_connections_api;
pub mod off_exchanges_api;
pub mod payments_cross_border_settlement_api;
pub mod payments_payout_api;
pub mod policy_editor_beta_api;
pub mod smart_transfer_api;
pub mod staking_api;
pub mod transactions_api;
pub mod travel_rule_beta_api;
pub mod users_api;
pub mod vaults_api;
pub mod wallet_link_api;
pub mod webhooks_api;
pub mod workspace_management_api;

pub mod configuration;

use std::sync::Arc;

pub trait Api {
    fn audit_logs_api(&self) -> &dyn audit_logs_api::AuditLogsApi;
    fn blockchains_assets_api(&self) -> &dyn blockchains_assets_api::BlockchainsAssetsApi;
    fn contracts_api(&self) -> &dyn contracts_api::ContractsApi;
    fn exchange_accounts_api(&self) -> &dyn exchange_accounts_api::ExchangeAccountsApi;
    fn external_wallets_api(&self) -> &dyn external_wallets_api::ExternalWalletsApi;
    fn fiat_accounts_api(&self) -> &dyn fiat_accounts_api::FiatAccountsApi;
    fn gas_stations_api(&self) -> &dyn gas_stations_api::GasStationsApi;
    fn internal_wallets_api(&self) -> &dyn internal_wallets_api::InternalWalletsApi;
    fn nfts_api(&self) -> &dyn nfts_api::NftsApi;
    fn network_connections_api(&self) -> &dyn network_connections_api::NetworkConnectionsApi;
    fn off_exchanges_api(&self) -> &dyn off_exchanges_api::OffExchangesApi;
    fn payments_cross_border_settlement_api(&self) -> &dyn payments_cross_border_settlement_api::PaymentsCrossBorderSettlementApi;
    fn payments_payout_api(&self) -> &dyn payments_payout_api::PaymentsPayoutApi;
    fn policy_editor_beta_api(&self) -> &dyn policy_editor_beta_api::PolicyEditorBetaApi;
    fn smart_transfer_api(&self) -> &dyn smart_transfer_api::SmartTransferApi;
    fn staking_api(&self) -> &dyn staking_api::StakingApi;
    fn transactions_api(&self) -> &dyn transactions_api::TransactionsApi;
    fn travel_rule_beta_api(&self) -> &dyn travel_rule_beta_api::TravelRuleBetaApi;
    fn users_api(&self) -> &dyn users_api::UsersApi;
    fn vaults_api(&self) -> &dyn vaults_api::VaultsApi;
    fn wallet_link_api(&self) -> &dyn wallet_link_api::WalletLinkApi;
    fn webhooks_api(&self) -> &dyn webhooks_api::WebhooksApi;
    fn workspace_management_api(&self) -> &dyn workspace_management_api::WorkspaceManagementApi;
}

pub struct ApiClient {
    audit_logs_api: Box<dyn audit_logs_api::AuditLogsApi>,
    blockchains_assets_api: Box<dyn blockchains_assets_api::BlockchainsAssetsApi>,
    contracts_api: Box<dyn contracts_api::ContractsApi>,
    exchange_accounts_api: Box<dyn exchange_accounts_api::ExchangeAccountsApi>,
    external_wallets_api: Box<dyn external_wallets_api::ExternalWalletsApi>,
    fiat_accounts_api: Box<dyn fiat_accounts_api::FiatAccountsApi>,
    gas_stations_api: Box<dyn gas_stations_api::GasStationsApi>,
    internal_wallets_api: Box<dyn internal_wallets_api::InternalWalletsApi>,
    nfts_api: Box<dyn nfts_api::NftsApi>,
    network_connections_api: Box<dyn network_connections_api::NetworkConnectionsApi>,
    off_exchanges_api: Box<dyn off_exchanges_api::OffExchangesApi>,
    payments_cross_border_settlement_api: Box<dyn payments_cross_border_settlement_api::PaymentsCrossBorderSettlementApi>,
    payments_payout_api: Box<dyn payments_payout_api::PaymentsPayoutApi>,
    policy_editor_beta_api: Box<dyn policy_editor_beta_api::PolicyEditorBetaApi>,
    smart_transfer_api: Box<dyn smart_transfer_api::SmartTransferApi>,
    staking_api: Box<dyn staking_api::StakingApi>,
    transactions_api: Box<dyn transactions_api::TransactionsApi>,
    travel_rule_beta_api: Box<dyn travel_rule_beta_api::TravelRuleBetaApi>,
    users_api: Box<dyn users_api::UsersApi>,
    vaults_api: Box<dyn vaults_api::VaultsApi>,
    wallet_link_api: Box<dyn wallet_link_api::WalletLinkApi>,
    webhooks_api: Box<dyn webhooks_api::WebhooksApi>,
    workspace_management_api: Box<dyn workspace_management_api::WorkspaceManagementApi>,
}

impl ApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self {
            audit_logs_api: Box::new(audit_logs_api::AuditLogsApiClient::new(configuration.clone())),
            blockchains_assets_api: Box::new(blockchains_assets_api::BlockchainsAssetsApiClient::new(configuration.clone())),
            contracts_api: Box::new(contracts_api::ContractsApiClient::new(configuration.clone())),
            exchange_accounts_api: Box::new(exchange_accounts_api::ExchangeAccountsApiClient::new(configuration.clone())),
            external_wallets_api: Box::new(external_wallets_api::ExternalWalletsApiClient::new(configuration.clone())),
            fiat_accounts_api: Box::new(fiat_accounts_api::FiatAccountsApiClient::new(configuration.clone())),
            gas_stations_api: Box::new(gas_stations_api::GasStationsApiClient::new(configuration.clone())),
            internal_wallets_api: Box::new(internal_wallets_api::InternalWalletsApiClient::new(configuration.clone())),
            nfts_api: Box::new(nfts_api::NftsApiClient::new(configuration.clone())),
            network_connections_api: Box::new(network_connections_api::NetworkConnectionsApiClient::new(configuration.clone())),
            off_exchanges_api: Box::new(off_exchanges_api::OffExchangesApiClient::new(configuration.clone())),
            payments_cross_border_settlement_api: Box::new(payments_cross_border_settlement_api::PaymentsCrossBorderSettlementApiClient::new(configuration.clone())),
            payments_payout_api: Box::new(payments_payout_api::PaymentsPayoutApiClient::new(configuration.clone())),
            policy_editor_beta_api: Box::new(policy_editor_beta_api::PolicyEditorBetaApiClient::new(configuration.clone())),
            smart_transfer_api: Box::new(smart_transfer_api::SmartTransferApiClient::new(configuration.clone())),
            staking_api: Box::new(staking_api::StakingApiClient::new(configuration.clone())),
            transactions_api: Box::new(transactions_api::TransactionsApiClient::new(configuration.clone())),
            travel_rule_beta_api: Box::new(travel_rule_beta_api::TravelRuleBetaApiClient::new(configuration.clone())),
            users_api: Box::new(users_api::UsersApiClient::new(configuration.clone())),
            vaults_api: Box::new(vaults_api::VaultsApiClient::new(configuration.clone())),
            wallet_link_api: Box::new(wallet_link_api::WalletLinkApiClient::new(configuration.clone())),
            webhooks_api: Box::new(webhooks_api::WebhooksApiClient::new(configuration.clone())),
            workspace_management_api: Box::new(workspace_management_api::WorkspaceManagementApiClient::new(configuration.clone())),
        }
    }
}

impl Api for ApiClient {
    fn audit_logs_api(&self) -> &dyn audit_logs_api::AuditLogsApi {
        self.audit_logs_api.as_ref()
    }
    fn blockchains_assets_api(&self) -> &dyn blockchains_assets_api::BlockchainsAssetsApi {
        self.blockchains_assets_api.as_ref()
    }
    fn contracts_api(&self) -> &dyn contracts_api::ContractsApi {
        self.contracts_api.as_ref()
    }
    fn exchange_accounts_api(&self) -> &dyn exchange_accounts_api::ExchangeAccountsApi {
        self.exchange_accounts_api.as_ref()
    }
    fn external_wallets_api(&self) -> &dyn external_wallets_api::ExternalWalletsApi {
        self.external_wallets_api.as_ref()
    }
    fn fiat_accounts_api(&self) -> &dyn fiat_accounts_api::FiatAccountsApi {
        self.fiat_accounts_api.as_ref()
    }
    fn gas_stations_api(&self) -> &dyn gas_stations_api::GasStationsApi {
        self.gas_stations_api.as_ref()
    }
    fn internal_wallets_api(&self) -> &dyn internal_wallets_api::InternalWalletsApi {
        self.internal_wallets_api.as_ref()
    }
    fn nfts_api(&self) -> &dyn nfts_api::NftsApi {
        self.nfts_api.as_ref()
    }
    fn network_connections_api(&self) -> &dyn network_connections_api::NetworkConnectionsApi {
        self.network_connections_api.as_ref()
    }
    fn off_exchanges_api(&self) -> &dyn off_exchanges_api::OffExchangesApi {
        self.off_exchanges_api.as_ref()
    }
    fn payments_cross_border_settlement_api(&self) -> &dyn payments_cross_border_settlement_api::PaymentsCrossBorderSettlementApi {
        self.payments_cross_border_settlement_api.as_ref()
    }
    fn payments_payout_api(&self) -> &dyn payments_payout_api::PaymentsPayoutApi {
        self.payments_payout_api.as_ref()
    }
    fn policy_editor_beta_api(&self) -> &dyn policy_editor_beta_api::PolicyEditorBetaApi {
        self.policy_editor_beta_api.as_ref()
    }
    fn smart_transfer_api(&self) -> &dyn smart_transfer_api::SmartTransferApi {
        self.smart_transfer_api.as_ref()
    }
    fn staking_api(&self) -> &dyn staking_api::StakingApi {
        self.staking_api.as_ref()
    }
    fn transactions_api(&self) -> &dyn transactions_api::TransactionsApi {
        self.transactions_api.as_ref()
    }
    fn travel_rule_beta_api(&self) -> &dyn travel_rule_beta_api::TravelRuleBetaApi {
        self.travel_rule_beta_api.as_ref()
    }
    fn users_api(&self) -> &dyn users_api::UsersApi {
        self.users_api.as_ref()
    }
    fn vaults_api(&self) -> &dyn vaults_api::VaultsApi {
        self.vaults_api.as_ref()
    }
    fn wallet_link_api(&self) -> &dyn wallet_link_api::WalletLinkApi {
        self.wallet_link_api.as_ref()
    }
    fn webhooks_api(&self) -> &dyn webhooks_api::WebhooksApi {
        self.webhooks_api.as_ref()
    }
    fn workspace_management_api(&self) -> &dyn workspace_management_api::WorkspaceManagementApi {
        self.workspace_management_api.as_ref()
    }
}


