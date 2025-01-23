# \WhitelistedContractsApi

All URIs are relative to *https://api.fireblocks.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_contract_asset**](WhitelistedContractsApi.md#add_contract_asset) | **POST** /contracts/{contractId}/{assetId} | Add an Asset to a Whitelisted Contract
[**create_contract**](WhitelistedContractsApi.md#create_contract) | **POST** /contracts | Create a Whitelisted Contract
[**delete_contract**](WhitelistedContractsApi.md#delete_contract) | **DELETE** /contracts/{contractId} | Delete a Whitelisted Contract
[**delete_contract_asset**](WhitelistedContractsApi.md#delete_contract_asset) | **DELETE** /contracts/{contractId}/{assetId} | Delete a Whitelisted Contract's Asset
[**get_contract**](WhitelistedContractsApi.md#get_contract) | **GET** /contracts/{contractId} | Find a Specific Whitelisted Contract
[**get_contract_asset**](WhitelistedContractsApi.md#get_contract_asset) | **GET** /contracts/{contractId}/{assetId} | Find a whitelisted contract's asset
[**get_contracts**](WhitelistedContractsApi.md#get_contracts) | **GET** /contracts | List Whitelisted Contracts



## add_contract_asset

> models::ExternalWalletAsset add_contract_asset(contract_id, asset_id, idempotency_key, add_contract_asset_request)
Add an Asset to a Whitelisted Contract

Adds an asset to an existing whitelisted contract. </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_id** | **String** | The ID of the contract | [required] |
**asset_id** | **String** | The ID of the asset to add | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |
**add_contract_asset_request** | Option<[**AddContractAssetRequest**](AddContractAssetRequest.md)> |  |  |

### Return type

[**models::ExternalWalletAsset**](ExternalWalletAsset.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_contract

> models::UnmanagedWallet create_contract(idempotency_key, create_contract_request)
Create a Whitelisted Contract

Creates a new whitelisted contract.  Learn more about Whitelisted Smart Contracts [here](https://developers.fireblocks.com/docs/whitelist-addresses#contracts).  </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |
**create_contract_request** | Option<[**CreateContractRequest**](CreateContractRequest.md)> |  |  |

### Return type

[**models::UnmanagedWallet**](UnmanagedWallet.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_contract

> delete_contract(contract_id)
Delete a Whitelisted Contract

Deletes a whitelisted contract by Fireblocks Contract ID. </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor.

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


## delete_contract_asset

> delete_contract_asset(contract_id, asset_id)
Delete a Whitelisted Contract's Asset

Deletes a whitelisted contract's asset by Fireblocks Contract ID and Asset ID. </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor.

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


## get_contract

> models::UnmanagedWallet get_contract(contract_id)
Find a Specific Whitelisted Contract

Returns a whitelisted contract by Fireblocks Contract ID.  </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor.

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
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_contract_asset

> models::ExternalWalletAsset get_contract_asset(contract_id, asset_id)
Find a whitelisted contract's asset

Returns a whitelisted contract's asset by ID.  </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor.

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
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_contracts

> Vec<models::UnmanagedWallet> get_contracts()
List Whitelisted Contracts

Gets a list of whitelisted contracts. </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::UnmanagedWallet>**](UnmanagedWallet.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

