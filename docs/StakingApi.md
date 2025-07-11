# \StakingApi

All URIs are relative to *https://api.fireblocks.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**approve_terms_of_service_by_provider_id**](StakingApi.md#approve_terms_of_service_by_provider_id) | **POST** /staking/providers/{providerId}/approveTermsOfService | Approve staking terms of service
[**claim_rewards**](StakingApi.md#claim_rewards) | **POST** /staking/chains/{chainDescriptor}/claim_rewards | Execute a Claim Rewards operation
[**get_all_delegations**](StakingApi.md#get_all_delegations) | **GET** /staking/positions | List staking positions details
[**get_chain_info**](StakingApi.md#get_chain_info) | **GET** /staking/chains/{chainDescriptor}/chainInfo | Get chain-specific staking summary
[**get_chains**](StakingApi.md#get_chains) | **GET** /staking/chains | List supported chains for Fireblocks Staking
[**get_delegation_by_id**](StakingApi.md#get_delegation_by_id) | **GET** /staking/positions/{id} | Get staking position details
[**get_providers**](StakingApi.md#get_providers) | **GET** /staking/providers | List staking providers details
[**get_summary**](StakingApi.md#get_summary) | **GET** /staking/positions/summary | Get staking summary details
[**get_summary_by_vault**](StakingApi.md#get_summary_by_vault) | **GET** /staking/positions/summary/vaults | Get staking summary details by vault
[**merge_stake_accounts**](StakingApi.md#merge_stake_accounts) | **POST** /staking/chains/{chainDescriptor}/merge | Merge operation on Solana stake accounts
[**split**](StakingApi.md#split) | **POST** /staking/chains/{chainDescriptor}/split | Execute a Split operation
[**stake**](StakingApi.md#stake) | **POST** /staking/chains/{chainDescriptor}/stake | Initiate Stake Operation
[**unstake**](StakingApi.md#unstake) | **POST** /staking/chains/{chainDescriptor}/unstake | Execute an Unstake operation
[**withdraw**](StakingApi.md#withdraw) | **POST** /staking/chains/{chainDescriptor}/withdraw | Execute a Withdraw operation



## approve_terms_of_service_by_provider_id

> approve_terms_of_service_by_provider_id(provider_id, idempotency_key)
Approve staking terms of service

Approve the terms of service of the staking provider. This must be called before performing a staking action for the first time with this provider. </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**provider_id** | [**StakingProvider**](.md) | The unique identifier of the staking provider | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## claim_rewards

> claim_rewards(chain_descriptor, claim_rewards_request, idempotency_key)
Execute a Claim Rewards operation

Perform a chain-specific Claim Rewards.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chain_descriptor** | **String** | The protocol identifier (e.g. \"MATIC\"/\"SOL\") to use | [required] |
**claim_rewards_request** | [**ClaimRewardsRequest**](ClaimRewardsRequest.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_delegations

> Vec<models::Delegation> get_all_delegations(chain_descriptor)
List staking positions details

Return detailed information on all staking positions, including the staked amount, rewards, status and more. </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chain_descriptor** | Option<[**ChainDescriptor**](.md)> | Use \"ETH\" / \"SOL\" / \"MATIC\" / \"STETH_ETH\" in order to obtain information related to the specific blockchain network or retrieve information about all chains that have data available by providing no argument. |  |

### Return type

[**Vec<models::Delegation>**](Delegation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_chain_info

> models::ChainInfoResponse get_chain_info(chain_descriptor)
Get chain-specific staking summary

Return chain-specific, staking-related information summary (e.g. epoch details, lockup durations, estimated rewards, etc.). </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chain_descriptor** | [**ChainDescriptor**](.md) | The protocol identifier (e.g. \"ETH\"/\"SOL\"/\"MATIC\"/\"STETH_ETH\") to use | [required] |

### Return type

[**models::ChainInfoResponse**](ChainInfoResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_chains

> Vec<models::ChainDescriptor> get_chains()
List supported chains for Fireblocks Staking

Return an alphabetical list of supported chains. </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ChainDescriptor>**](ChainDescriptor.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_delegation_by_id

> models::Delegation get_delegation_by_id(id)
Get staking position details

Return detailed information on a staking position, including the staked amount, rewards, status and more. </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The unique identifier of the staking position | [required] |

### Return type

[**models::Delegation**](Delegation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_providers

> Vec<models::Provider> get_providers()
List staking providers details

Return information on all the available staking providers. </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::Provider>**](Provider.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_summary

> models::DelegationSummary get_summary()
Get staking summary details

Return a summary of all vaults, categorized by their status (active, inactive), the total amounts staked and total rewards per-chain. </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::DelegationSummary**](DelegationSummary.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_summary_by_vault

> std::collections::HashMap<String, models::DelegationSummary> get_summary_by_vault()
Get staking summary details by vault

Return a summary for each vault, categorized by their status (active, inactive), the total amounts staked and total rewards per-chain. </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor.

### Parameters

This endpoint does not need any parameter.

### Return type

[**std::collections::HashMap<String, models::DelegationSummary>**](DelegationSummary.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## merge_stake_accounts

> models::MergeStakeAccountsResponse merge_stake_accounts(chain_descriptor, merge_stake_accounts_request, idempotency_key)
Merge operation on Solana stake accounts

Perform a Solana Merge of two active stake accounts into one. Endpoint permissions: Admin, Non-Signing Admin, Signer, Approver, Editor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chain_descriptor** | **String** | The protocol identifier (e.g. \"SOL\"/\"SOL_TEST\") to use | [required] |
**merge_stake_accounts_request** | [**MergeStakeAccountsRequest**](MergeStakeAccountsRequest.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::MergeStakeAccountsResponse**](MergeStakeAccountsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## split

> models::SplitResponse split(chain_descriptor, split_request, idempotency_key)
Execute a Split operation

Perform a SOL/SOL_TEST Split on a stake account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chain_descriptor** | **String** | The protocol identifier (e.g. \"SOL\"/\"SOL_TEST\") to use | [required] |
**split_request** | [**SplitRequest**](SplitRequest.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::SplitResponse**](SplitResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stake

> models::StakeResponse stake(chain_descriptor, stake_request, idempotency_key)
Initiate Stake Operation

Perform a chain-specific Stake.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chain_descriptor** | [**ChainDescriptor**](.md) | The protocol identifier (e.g. \"ETH\"/\"SOL\"/\"MATIC\") to use | [required] |
**stake_request** | [**StakeRequest**](StakeRequest.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::StakeResponse**](StakeResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unstake

> unstake(chain_descriptor, unstake_request, idempotency_key)
Execute an Unstake operation

Execute an Unstake operation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chain_descriptor** | [**ChainDescriptor**](.md) | The protocol identifier (e.g. \"ETH\"/\"SOL\"/\"MATIC\") to use | [required] |
**unstake_request** | [**UnstakeRequest**](UnstakeRequest.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## withdraw

> withdraw(chain_descriptor, withdraw_request, idempotency_key)
Execute a Withdraw operation

Perform a chain-specific Withdraw.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chain_descriptor** | [**ChainDescriptor**](.md) | The protocol identifier (e.g. \"ETH\"/\"SOL\"/\"MATIC\") to use | [required] |
**withdraw_request** | [**WithdrawRequest**](WithdrawRequest.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

