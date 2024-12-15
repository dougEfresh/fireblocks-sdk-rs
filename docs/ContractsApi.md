# \ContractsApi

All URIs are relative to *https://api.fireblocks.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**contracts_contract_id_asset_id_delete**](ContractsApi.md#contracts_contract_id_asset_id_delete) | **DELETE** /contracts/{contractId}/{assetId} | Delete a contract asset
[**contracts_contract_id_asset_id_get**](ContractsApi.md#contracts_contract_id_asset_id_get) | **GET** /contracts/{contractId}/{assetId} | Find a contract asset
[**contracts_contract_id_asset_id_post**](ContractsApi.md#contracts_contract_id_asset_id_post) | **POST** /contracts/{contractId}/{assetId} | Add an asset to a contract
[**contracts_contract_id_delete**](ContractsApi.md#contracts_contract_id_delete) | **DELETE** /contracts/{contractId} | Delete a contract
[**contracts_contract_id_get**](ContractsApi.md#contracts_contract_id_get) | **GET** /contracts/{contractId} | Find a specific contract
[**contracts_get**](ContractsApi.md#contracts_get) | **GET** /contracts | List contracts
[**contracts_post**](ContractsApi.md#contracts_post) | **POST** /contracts | Create a contract



## contracts_contract_id_asset_id_delete

> contracts_contract_id_asset_id_delete(contract_id, asset_id)
Delete a contract asset

Deletes a contract asset by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_id** | **String** | The ID of the contract | [required] |
**asset_id** | **String** | The ID of the asset to delete | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contracts_contract_id_asset_id_get

> models::ExternalWalletAsset contracts_contract_id_asset_id_get(contract_id, asset_id)
Find a contract asset

Returns a contract asset by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_id** | **String** | The ID of the contract | [required] |
**asset_id** | **String** | The ID of the asset to return | [required] |

### Return type

[**models::ExternalWalletAsset**](ExternalWalletAsset.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contracts_contract_id_asset_id_post

> models::ExternalWalletAsset contracts_contract_id_asset_id_post(contract_id, asset_id, contracts_contract_id_asset_id_post_request)
Add an asset to a contract

Adds an asset to an existing contract.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_id** | **String** | The ID of the contract | [required] |
**asset_id** | **String** | The ID of the asset to add | [required] |
**contracts_contract_id_asset_id_post_request** | Option<[**ContractsContractIdAssetIdPostRequest**](ContractsContractIdAssetIdPostRequest.md)> |  |  |

### Return type

[**models::ExternalWalletAsset**](ExternalWalletAsset.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contracts_contract_id_delete

> contracts_contract_id_delete(contract_id)
Delete a contract

Deletes a contract by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_id** | **String** | The ID of the contract to delete | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contracts_contract_id_get

> models::UnmanagedWallet contracts_contract_id_get(contract_id)
Find a specific contract

Returns a contract by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_id** | **String** | The ID of the contract to return | [required] |

### Return type

[**models::UnmanagedWallet**](UnmanagedWallet.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contracts_get

> Vec<models::UnmanagedWallet> contracts_get()
List contracts

Gets a list of contracts.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::UnmanagedWallet>**](UnmanagedWallet.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contracts_post

> models::UnmanagedWallet contracts_post(contracts_post_request)
Create a contract

Creates a new contract.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contracts_post_request** | Option<[**ContractsPostRequest**](ContractsPostRequest.md)> |  |  |

### Return type

[**models::UnmanagedWallet**](UnmanagedWallet.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

