# \StakingApi

All URIs are relative to *https://api.fireblocks.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**approve_terms_of_service_by_provider_id**](StakingApi.md#approve_terms_of_service_by_provider_id) | **POST** /staking/providers/{providerId}/approveTermsOfService | Approve staking terms of service
[**execute_action**](StakingApi.md#execute_action) | **POST** /staking/chains/{chainDescriptor}/{actionId} | Execute a staking action
[**get_all_delegations**](StakingApi.md#get_all_delegations) | **GET** /staking/positions | List staking positions details
[**get_chain_info**](StakingApi.md#get_chain_info) | **GET** /staking/chains/{chainDescriptor}/chainInfo | Get chain-specific staking summary
[**get_chains**](StakingApi.md#get_chains) | **GET** /staking/chains | List supported chains for Fireblocks Staking
[**get_delegation_by_id**](StakingApi.md#get_delegation_by_id) | **GET** /staking/positions/{id} | Get staking position details
[**get_providers**](StakingApi.md#get_providers) | **GET** /staking/providers | List staking providers details
[**get_summary**](StakingApi.md#get_summary) | **GET** /staking/positions/summary | Get staking summary details
[**get_summary_by_vault**](StakingApi.md#get_summary_by_vault) | **GET** /staking/positions/summary/vaults | Get staking summary details by vault



## approve_terms_of_service_by_provider_id

> serde_json::Value approve_terms_of_service_by_provider_id(provider_id, idempotency_key)
Approve staking terms of service

Approve the terms of service of the staking provider. This must be called before performing a staking action for the first time with this provider.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**provider_id** | **String** | The unique identifier of the staking provider | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## execute_action

> models::ExecuteActionResponse execute_action(chain_descriptor, action_id, execute_action_request, idempotency_key)
Execute a staking action

Perform a chain-specific staking action (e.g. stake, unstake, withdraw).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chain_descriptor** | **String** | The protocol identifier (e.g. \"ETH\"/\"SOL\") to use | [required] |
**action_id** | **String** | The operation that can be executed on a vault/position | [required] |
**execute_action_request** | [**ExecuteActionRequest**](ExecuteActionRequest.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::ExecuteActionResponse**](ExecuteActionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_delegations

> Vec<models::DelegationDto> get_all_delegations(chain_descriptor)
List staking positions details

Return detailed information on all staking positions, including the staked amount, rewards, status and more.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chain_descriptor** | Option<**String**> | Use \"ETH\" / \"SOL\"/ \"MATIC\" in order to obtain information related to the specific blockchain network or retrieve information about all chains that have data available by providing no argument. |  |

### Return type

[**Vec<models::DelegationDto>**](DelegationDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_chain_info

> models::ChainInfoResponseDto get_chain_info(chain_descriptor)
Get chain-specific staking summary

Return chain-specific, staking-related information summary (e.g. epoch details, lockup durations, estimated rewards, etc.).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chain_descriptor** | **String** | The protocol identifier (e.g. \"ETH\"/\"SOL\"/\"MATIC\") to use | [required] |

### Return type

[**models::ChainInfoResponseDto**](ChainInfoResponseDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_chains

> Vec<String> get_chains()
List supported chains for Fireblocks Staking

Return an alphabetical list of supported chains.

### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_delegation_by_id

> models::DelegationDto get_delegation_by_id(id)
Get staking position details

Return detailed information on a staking position, including the staked amount, rewards, status and more.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The unique identifier of the staking position | [required] |

### Return type

[**models::DelegationDto**](DelegationDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_providers

> Vec<models::ProviderDto> get_providers()
List staking providers details

Return information on all the available staking providers.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ProviderDto>**](ProviderDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_summary

> models::DelegationSummaryDto get_summary()
Get staking summary details

Return a summary of all vaults, categorized by their status (active, inactive), the total amounts staked and total rewards per-chain.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::DelegationSummaryDto**](DelegationSummaryDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_summary_by_vault

> std::collections::HashMap<String, models::DelegationSummaryDto> get_summary_by_vault()
Get staking summary details by vault

Return a summary for each vault, categorized by their status (active, inactive), the total amounts staked and total rewards per-chain.

### Parameters

This endpoint does not need any parameter.

### Return type

[**std::collections::HashMap<String, models::DelegationSummaryDto>**](DelegationSummaryDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

