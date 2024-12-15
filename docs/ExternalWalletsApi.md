# \ExternalWalletsApi

All URIs are relative to *https://api.fireblocks.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**external_wallets_get**](ExternalWalletsApi.md#external_wallets_get) | **GET** /external_wallets | List external wallets
[**external_wallets_post**](ExternalWalletsApi.md#external_wallets_post) | **POST** /external_wallets | Create an external wallet
[**external_wallets_wallet_id_asset_id_delete**](ExternalWalletsApi.md#external_wallets_wallet_id_asset_id_delete) | **DELETE** /external_wallets/{walletId}/{assetId} | Delete an asset from an external wallet
[**external_wallets_wallet_id_asset_id_get**](ExternalWalletsApi.md#external_wallets_wallet_id_asset_id_get) | **GET** /external_wallets/{walletId}/{assetId} | Get an asset from an external wallet
[**external_wallets_wallet_id_asset_id_post**](ExternalWalletsApi.md#external_wallets_wallet_id_asset_id_post) | **POST** /external_wallets/{walletId}/{assetId} | Add an asset to an external wallet.
[**external_wallets_wallet_id_delete**](ExternalWalletsApi.md#external_wallets_wallet_id_delete) | **DELETE** /external_wallets/{walletId} | Delete an external wallet
[**external_wallets_wallet_id_get**](ExternalWalletsApi.md#external_wallets_wallet_id_get) | **GET** /external_wallets/{walletId} | Find an external wallet
[**external_wallets_wallet_id_set_customer_ref_id_post**](ExternalWalletsApi.md#external_wallets_wallet_id_set_customer_ref_id_post) | **POST** /external_wallets/{walletId}/set_customer_ref_id | Set an AML customer reference ID for an external wallet



## external_wallets_get

> Vec<models::UnmanagedWallet> external_wallets_get()
List external wallets

Gets a list of external wallets under the workspace.

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


## external_wallets_post

> models::UnmanagedWallet external_wallets_post(internal_wallets_post_request)
Create an external wallet

Creates a new external wallet with the requested name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**internal_wallets_post_request** | Option<[**InternalWalletsPostRequest**](InternalWalletsPostRequest.md)> |  |  |

### Return type

[**models::UnmanagedWallet**](UnmanagedWallet.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## external_wallets_wallet_id_asset_id_delete

> external_wallets_wallet_id_asset_id_delete(wallet_id, asset_id)
Delete an asset from an external wallet

Deletes an external wallet asset by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wallet_id** | **String** | The ID of the wallet | [required] |
**asset_id** | **String** | The ID of the asset to delete | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## external_wallets_wallet_id_asset_id_get

> models::ExternalWalletAsset external_wallets_wallet_id_asset_id_get(wallet_id, asset_id)
Get an asset from an external wallet

Returns an external wallet by wallet ID and asset ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wallet_id** | **String** | The ID of the wallet | [required] |
**asset_id** | **String** | The ID of the asset to return | [required] |

### Return type

[**models::ExternalWalletAsset**](ExternalWalletAsset.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## external_wallets_wallet_id_asset_id_post

> models::ExternalWalletAsset external_wallets_wallet_id_asset_id_post(wallet_id, asset_id, external_wallets_wallet_id_asset_id_post_request)
Add an asset to an external wallet.

Adds an asset to an existing external wallet.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wallet_id** | **String** | The ID of the wallet | [required] |
**asset_id** | **String** | The ID of the asset to add | [required] |
**external_wallets_wallet_id_asset_id_post_request** | Option<[**ExternalWalletsWalletIdAssetIdPostRequest**](ExternalWalletsWalletIdAssetIdPostRequest.md)> |  |  |

### Return type

[**models::ExternalWalletAsset**](ExternalWalletAsset.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## external_wallets_wallet_id_delete

> external_wallets_wallet_id_delete(wallet_id)
Delete an external wallet

Deletes an external wallet by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wallet_id** | **String** | The ID of the wallet to delete | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## external_wallets_wallet_id_get

> models::UnmanagedWallet external_wallets_wallet_id_get(wallet_id)
Find an external wallet

Returns an external wallet by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wallet_id** | **String** | The ID of the wallet to return | [required] |

### Return type

[**models::UnmanagedWallet**](UnmanagedWallet.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## external_wallets_wallet_id_set_customer_ref_id_post

> external_wallets_wallet_id_set_customer_ref_id_post(wallet_id, vault_accounts_vault_account_id_set_customer_ref_id_post_request)
Set an AML customer reference ID for an external wallet

Sets an AML/KYT customer reference ID for the specific external wallet.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wallet_id** | **String** | The wallet ID | [required] |
**vault_accounts_vault_account_id_set_customer_ref_id_post_request** | [**VaultAccountsVaultAccountIdSetCustomerRefIdPostRequest**](VaultAccountsVaultAccountIdSetCustomerRefIdPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

