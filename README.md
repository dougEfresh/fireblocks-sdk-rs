<div align="center">
  <h1><code>fireblocks-sdk</code></h1>
  <a href="https://docs.rs/fireblocks-sdk/">
    <img src="https://docs.rs/fireblocks-sdk/badge.svg">
  </a>
  <a href="https://github.com/dougEfresh/fireblocks-sdk/actions">
    <img src="https://github.com/dougEfresh/fireblocks-sdk/workflows/Continuous%20integration/badge.svg">
  </a>
  <a href="https://deps.rs/repo/github/dougEfresh/fireblocks-sdk">
    <img src="https://deps.rs/repo/github/dougEfresh/fireblocks-sdk/status.svg" >
  </a>
  <a href="https://codecov.io/gh/dougEfresh/fireblocks-sdk" > 
   <img src="https://codecov.io/gh/dougEfresh/fireblocks-sdk/graph/badge.svg?token=OI06VXUKKJ"/> 
 </a>  
  <a href="https://crates.io/crates/fireblocks-sdk">
    <img src="https://img.shields.io/crates/v/fireblocks-sdk.svg">
  </a>
</div>


# Overview

`fireblocks_sdk` is an async library for the Fireblocks [API](https://docs.fireblocks.com/api/swagger-ui/#)

# Getting Started 

See developer [portal](https://developers.fireblocks.com/docs/introduction) and sign up for a [sandbox](https://developers.fireblocks.com/docs/sandbox-quickstart) account


# Quick Start

```rust
use fireblocks_sdk::{ClientBuilder, PagingVaultRequestBuilder};
use std::time::Duration;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
  let api_key = std::env::var("FIREBLOCKS_API_KEY")?;
  let secret = std::env::var("FIREBLOCKS_SECRET")?;
  let client = ClientBuilder::new(&api_key, &secret.into_bytes())
    .with_timeout(Duration::from_secs(10))
    .with_connect_timeout(Duration::from_secs(5))
    .build()?;
  let params = PagingVaultRequestBuilder::new().limit(10).build()?;
  let (vault_accounts, request_id) = client.vaults(params).await?;
  println!("Got requestId: {request_id}");
  println!("vault accounts: {:#?}", vault_accounts.accounts);
  Ok(())
}
```

# Supported Endpoints

## Vaults


| Endpoint                                                                                  | Status  |
|-------------------------------------------------------------------------------------------|---------|
| GET /vault/accounts                                                                       | &check; |
| POST /vault/accounts                                                                      | &check; |
| GET /vault/accounts_paged                                                                 | &check; |
| GET /vault/accounts/{vaultAccountId}                                                      | &check; |
| PUT /vault/accounts/{vaultAccountId}                                                      | &check; |
| GET /vault/asset_wallets                                                                  | &check; |
| POST /vault/accounts/{vaultAccountId}/hide                                                | &cross; |
| POST /vault/accounts/{vaultAccountId}/unhide                                              | &cross; |
| POST /vault/accounts/{vaultAccountId}/{assetId}/activate                                  | &cross; |
| POST /vault/accounts/{vaultAccountId}/set_customer_ref_id                                 | &cross; |
| POST /vault/accounts/{vaultAccountId}/set_auto_fuel                                       | &cross; |
| GET /vault/accounts/{vaultAccountId}/{assetId}                                            | &check; |
| POST /vault/accounts/{vaultAccountId}/{assetId}                                           | &check; |
| POST /vault/accounts/{vaultAccountId}/{assetId}/balance                                   | &cross; |
| GET /vault/accounts/{vaultAccountId}/{assetId}/addresses                                  | &cross; |
| POST /vault/accounts/{vaultAccountId}/{assetId}/addresses                                 | &cross; |
| GET /vault/accounts/{vaultAccountId}/{assetId}/addresses_paginated                        | &cross; |
| GET /vault/accounts/{vaultAccountId}/{assetId}/max_spendable_amount                       | &cross; |
| PUT /vault/accounts/{vaultAccountId}/{assetId}/addresses/{addressId}                      | &cross; |
| POST /vault/accounts/{vaultAccountId}/{assetId}/addresses/{addressId}/set_customer_ref_id | &cross; |
| POST /vault/accounts/{vaultAccountId}/{assetId}/addresses/{addressId}/create_legacy       | &cross; |
| GET /vault/accounts/{vaultAccountId}/{assetId}/unspent_inputs                             | &cross; |
| GET /vault/public_key_info/                                                               | &cross; |
| GET /vault/accounts/{vaultAccountId}/{assetId}/{change}/{addressIndex}/public_key_info    | &cross; |
| GET /vault/assets                                                                         | &check; |
| GET /vault/assets/{assetId}                                                               | &check; |


## Staking


| Endpoint                                                   | Status  |
|------------------------------------------------------------|---------|
| GET /staking/chains                                        | &check; |
| GET /staking/chains/{chainDescriptor}/chainInfo            | &cross; |
| POST /staking/chains/{chainDescriptor}/stake               | &cross; |
| POST /staking/chains/{chainDescriptor}/unstake             | &cross; |
| POST /staking/chains/{chainDescriptor}/withdraw            | &cross; |
| POST /staking/chains/{chainDescriptor}/claimRewards        | &cross; |
| GET /staking/positions                                     | &check; |
| GET /staking/positions/summary                             | &check; |
| GET /staking/positions/summary/vaults                      | &cross; |
| GET /staking/positions/{id}                                | &cross; |
| GET /staking/providers                                     | &cross; |
| POST /staking/providers/{providerId}/approveTermsOfService | &cross; |


## Exchange accounts


| Endpoint                                                      | Status  |
|---------------------------------------------------------------|---------|
| GET /exchange_accounts                                        | &cross; |
| GET /exchange_accounts/paged                                  | &cross; |
| GET /exchange_accounts/{exchangeAccountId}                    | &cross; |
| POST /exchange_accounts/{exchangeAccountId}/internal_transfer | &cross; |
| POST /exchange_accounts/{exchangeAccountId}/convert           | &cross; |
| GET /exchange_accounts/{exchangeAccountId}/{assetId}          | &cross; |


## Fiat accounts


| Endpoint                                                | Status  |
|---------------------------------------------------------|---------|
| GET /fiat_accounts                                      | &cross; |
| GET /fiat_accounts/{accountId}                          | &cross; |
| POST /fiat_accounts/{accountId}/redeem_to_linked_dda    | &cross; |
| POST /fiat_accounts/{accountId}/deposit_from_linked_dda | &cross; |


## Network connections


| Endpoint                                                                   | Status  |
|----------------------------------------------------------------------------|---------|
| GET /network_connections                                                   | &cross; |
| POST /network_connections                                                  | &cross; |
| PATCH /network_connections/{connectionId}/set_routing_policy               | &cross; |
| GET /network_connections/{connectionId}/is_third_party_routing/{assetType} | &cross; |
| GET /network_connections/{connectionId}                                    | &cross; |
| DELETE /network_connections/{connectionId}                                 | &cross; |
| GET /network_ids                                                           | &cross; |
| POST /network_ids                                                          | &cross; |
| GET /network_ids/{networkId}                                               | &cross; |
| DELETE /network_ids/{networkId}                                            | &cross; |
| PATCH /network_ids/{networkId}/set_routing_policy                          | &cross; |
| PATCH /network_ids/{networkId}/set_discoverability                         | &cross; |
| PATCH /network_ids/{networkId}/set_name                                    | &cross; |


## Internal wallets


| Endpoint                                              | Status  |
|-------------------------------------------------------|---------|
| GET /internal_wallets                                 | &check; |
| POST /internal_wallets                                | &check; |
| GET /internal_wallets/{walletId}                      | &check; |
| DELETE /internal_wallets/{walletId}                   | &check; |
| POST /internal_wallets/{walletId}/set_customer_ref_id | &cross; |
| GET /internal_wallets/{walletId}/{assetId}            | &cross; |
| POST /internal_wallets/{walletId}/{assetId}           | &cross; |
| DELETE /internal_wallets/{walletId}/{assetId}         | &cross; |


## External wallets


| Endpoint                                              | Status  |
|-------------------------------------------------------|---------|
| GET /external_wallets                                 | &check; |
| POST /external_wallets                                | &check; |
| GET /external_wallets/{walletId}                      | &check; |
| DELETE /external_wallets/{walletId}                   | &check; |
| POST /external_wallets/{walletId}/set_customer_ref_id | &cross; |
| GET /external_wallets/{walletId}/{assetId}            | &cross; |
| POST /external_wallets/{walletId}/{assetId}           | &cross; |
| DELETE /external_wallets/{walletId}/{assetId}         | &cross; |


## Contracts


| Endpoint                                 | Status  |
|------------------------------------------|---------|
| GET /contracts                           | &cross; |
| POST /contracts                          | &cross; |
| GET /contracts/{contractId}              | &cross; |
| DELETE /contracts/{contractId}           | &cross; |
| GET /contracts/{contractId}/{assetId}    | &cross; |
| POST /contracts/{contractId}/{assetId}   | &cross; |
| DELETE /contracts/{contractId}/{assetId} | &cross; |


## Blockchains & assets


| Endpoint                                               | Status  |
|--------------------------------------------------------|---------|
| GET /supported_assets                                  | &cross; |
| POST /assets                                           | &cross; |
| GET /estimate_network_fee                              | &cross; |
| GET /transactions/validate_address/{assetId}/{address} | &cross; |


## Transactions


| Endpoint                                             | Status  |
|------------------------------------------------------|---------|
| GET /transactions                                    | &cross; |
| POST /transactions                                   | &cross; |
| POST /transactions/estimate_fee                      | &cross; |
| GET /transactions/{txId}                             | &cross; |
| GET /transactions/external_tx_id/{externalTxId}/     | &cross; |
| POST /transactions/{txId}/set_confirmation_threshold | &cross; |
| POST /transactions/{txId}/drop                       | &cross; |
| POST /transactions/{txId}/cancel                     | &cross; |
| POST /transactions/{txId}/freeze                     | &cross; |
| POST /transactions/{txId}/unfreeze                   | &cross; |
| POST /txHash/{txHash}/set_confirmation_threshold     | &cross; |


## Payments - cross-border settlement


| Endpoint                                                     | Status  |
|--------------------------------------------------------------|---------|
| POST /payments/xb-settlements/configs                        | &cross; |
| GET /payments/xb-settlements/configs                         | &cross; |
| GET /payments/xb-settlements/configs/{configId}              | &cross; |
| PUT /payments/xb-settlements/configs/{configId}              | &cross; |
| DELETE /payments/xb-settlements/configs/{configId}           | &cross; |
| POST /payments/xb-settlements/flows                          | &cross; |
| GET /payments/xb-settlements/flows/{flowId}                  | &cross; |
| POST /payments/xb-settlements/flows/{flowId}/actions/execute | &cross; |


## Payments - Payout


| Endpoint                                         | Status  |
|--------------------------------------------------|---------|
| POST /payments/payout                            | &cross; |
| POST /payments/payout/{payoutId}/actions/execute | &cross; |
| GET /payments/payout/{payoutId}                  | &cross; |


## Payments - Flows


| Endpoint                                                                | Status  |
|-------------------------------------------------------------------------|---------|
| POST /payments/workflow_config                                          | &cross; |
| GET /payments/workflow_config/{configId}                                | &cross; |
| DELETE /payments/workflow_config/{configId}                             | &cross; |
| POST /payments/workflow_execution                                       | &cross; |
| GET /payments/workflow_execution/{workflowExecutionId}                  | &cross; |
| POST /payments/workflow_execution/{workflowExecutionId}/actions/execute | &cross; |


## Gas stations


| Endpoint                                 | Status  |
|------------------------------------------|---------|
| GET /gas_station                         | &cross; |
| GET /gas_station/{assetId}               | &cross; |
| PUT /gas_station/configuration           | &cross; |
| PUT /gas_station/configuration/{assetId} | &cross; |


## Workspace Management


| Endpoint                                                  | Status  |
|-----------------------------------------------------------|---------|
| GET /management/user_groups                               | &cross; |
| POST /management/user_groups                              | &cross; |
| GET /management/user_groups/{groupId}                     | &cross; |
| PUT /management/user_groups/{groupId}                     | &cross; |
| DELETE /management/user_groups/{groupId}                  | &cross; |
| GET /management/audit_logs                                | &cross; |
| POST /management/ota                                      | &cross; |
| GET /management/ota                                       | &cross; |
| GET /management/workspace_status                          | &cross; |
| GET /management/users                                     | &cross; |
| POST /management/users                                    | &cross; |
| GET /management/api_users                                 | &cross; |
| POST /management/api_users                                | &cross; |
| POST /management/users/{id}/reset_device                  | &cross; |
| GET /management/api_users/{userId}/whitelist_ip_addresses | &cross; |


## Users


| Endpoint   | Status  |
|------------|---------|
| GET /users | &cross; |


## Audit Logs


| Endpoint    | Status  |
|-------------|---------|
| GET /audits | &cross; |


## Off exchanges


| Endpoint                                                      | Status  |
|---------------------------------------------------------------|---------|
| POST /off_exchange/add                                        | &cross; |
| POST /off_exchange/remove                                     | &cross; |
| POST /off_exchange/settlements/trader                         | &cross; |
| GET /off_exchange/settlements/transactions                    | &cross; |
| GET /off_exchange/collateral_accounts/{mainExchangeAccountId} | &cross; |


## Webhooks


| Endpoint                     | Status  |
|------------------------------|---------|
| POST /webhooks/resend        | &cross; |
| POST /webhooks/resend/{txId} | &cross; |


## NFTs


| Endpoint                               | Status  |
|----------------------------------------|---------|
| PUT /nfts/ownership/tokens             | &cross; |
| GET /nfts/ownership/tokens             | &cross; |
| GET /nfts/ownership/assets             | &cross; |
| GET /nfts/ownership/collections        | &cross; |
| PUT /nfts/tokens/{id}                  | &cross; |
| GET /nfts/tokens/{id}                  | &cross; |
| GET /nfts/tokens                       | &cross; |
| PUT /nfts/ownership/tokens/{id}/status | &cross; |
| PUT /nfts/ownership/tokens/status      | &cross; |
| PUT /api/v1/nfts/ownership/tokens/spam | &cross; |


## WalletLink


| Endpoint                    | Status  |
|-----------------------------|---------|
| GET /connections            | &cross; |
| POST /connections/wc        | &cross; |
| PUT /connections/wc/{id}    | &cross; |
| DELETE /connections/wc/{id} | &cross; |


## Travel Rule (Beta)


| Endpoint                                              | Status  |
|-------------------------------------------------------|---------|
| POST /screening/travel_rule/transaction/validate      | &cross; |
| POST /screening/travel_rule/transaction/validate/full | &cross; |
| GET /screening/travel_rule/vasp/{did}                 | &cross; |
| GET /screening/travel_rule/vasp                       | &cross; |
| PUT /screeening/travel_rule/vasp/update               | &cross; |


## Policy Editor (Beta)


| Endpoint               | Status  |
|------------------------|---------|
| GET /tap/active_policy | &cross; |
| GET /tap/draft         | &cross; |
| PUT /tap/draft         | &cross; |
| POST /tap/draft        | &cross; |
| POST /tap/publish      | &cross; |


## Smart Transfer


| Endpoint                                                     | Status  |
|--------------------------------------------------------------|---------|
| POST /smart-transfers                                        | &cross; |
| GET /smart-transfers                                         | &cross; |
| GET /smart-transfers/{ticketId}                              | &cross; |
| PUT /smart-transfers/{ticketId}/expires-in                   | &cross; |
| PUT /smart-transfers/{ticketId}/external-id                  | &cross; |
| PUT /smart-transfers/{ticketId}/submit                       | &cross; |
| PUT /smart-transfers/{ticketId}/fulfill                      | &cross; |
| PUT /smart-transfers/{ticketId}/cancel                       | &cross; |
| POST /smart-transfers/{ticketId}/terms                       | &cross; |
| GET /smart-transfers/{ticketId}/terms/{termId}               | &cross; |
| PUT /smart-transfers/{ticketId}/terms/{termId}               | &cross; |
| DELETE /smart-transfers/{ticketId}/terms/{termId}            | &cross; |
| PUT /smart-transfers/{ticketId}/terms/{termId}/fund          | &cross; |
| PUT /smart-transfers/{ticketId}/terms/{termId}/manually-fund | &cross; |
| POST /smart-transfers/settings/user-groups                   | &cross; |
| GET /smart-transfers/settings/user-groups                    | &cross; |


