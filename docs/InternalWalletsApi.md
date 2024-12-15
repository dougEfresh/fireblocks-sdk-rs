# \InternalWalletsApi

All URIs are relative to *https://api.fireblocks.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**internal_wallets_get**](InternalWalletsApi.md#internal_wallets_get) | **GET** /internal_wallets | List internal wallets
[**internal_wallets_post**](InternalWalletsApi.md#internal_wallets_post) | **POST** /internal_wallets | Create an internal wallet
[**internal_wallets_wallet_id_asset_id_delete**](InternalWalletsApi.md#internal_wallets_wallet_id_asset_id_delete) | **DELETE** /internal_wallets/{walletId}/{assetId} | Delete a whitelisted address from an internal wallet
[**internal_wallets_wallet_id_asset_id_get**](InternalWalletsApi.md#internal_wallets_wallet_id_asset_id_get) | **GET** /internal_wallets/{walletId}/{assetId} | Get an asset from an internal wallet
[**internal_wallets_wallet_id_asset_id_post**](InternalWalletsApi.md#internal_wallets_wallet_id_asset_id_post) | **POST** /internal_wallets/{walletId}/{assetId} | Add an asset to an internal wallet
[**internal_wallets_wallet_id_delete**](InternalWalletsApi.md#internal_wallets_wallet_id_delete) | **DELETE** /internal_wallets/{walletId} | Delete an internal wallet
[**internal_wallets_wallet_id_get**](InternalWalletsApi.md#internal_wallets_wallet_id_get) | **GET** /internal_wallets/{walletId} | Get assets for internal wallet
[**internal_wallets_wallet_id_set_customer_ref_id_post**](InternalWalletsApi.md#internal_wallets_wallet_id_set_customer_ref_id_post) | **POST** /internal_wallets/{walletId}/set_customer_ref_id | Set an AML/KYT customer reference ID for an internal wallet



## internal_wallets_get

> Vec<models::UnmanagedWallet> internal_wallets_get()
List internal wallets

Gets a list of internal wallets.  **Note**: BTC-based assets belonging to whitelisted addresses cannot be retrieved between 00:00 UTC and 00:01 UTC daily due to third-party provider, Blockchair, being unavailable for this 60 second period. Please wait until the next minute to retrieve BTC-based assets. 

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


## internal_wallets_post

> models::UnmanagedWallet internal_wallets_post(internal_wallets_post_request)
Create an internal wallet

Creates a new internal wallet with the requested name.

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


## internal_wallets_wallet_id_asset_id_delete

> internal_wallets_wallet_id_asset_id_delete(wallet_id, asset_id)
Delete a whitelisted address from an internal wallet

Deletes a whitelisted address (for an asset) from an internal wallet.

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


## internal_wallets_wallet_id_asset_id_get

> models::WalletAsset internal_wallets_wallet_id_asset_id_get(wallet_id, asset_id)
Get an asset from an internal wallet

Returns information for an asset in an internal wallet.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wallet_id** | **String** | The ID of the wallet | [required] |
**asset_id** | **String** | The ID of the asset to return | [required] |

### Return type

[**models::WalletAsset**](WalletAsset.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## internal_wallets_wallet_id_asset_id_post

> models::WalletAsset internal_wallets_wallet_id_asset_id_post(wallet_id, asset_id, internal_wallets_wallet_id_asset_id_post_request)
Add an asset to an internal wallet

Adds an asset to an existing internal wallet.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wallet_id** | **String** | The ID of the wallet | [required] |
**asset_id** | **String** | The ID of the asset to add | [required] |
**internal_wallets_wallet_id_asset_id_post_request** | Option<[**InternalWalletsWalletIdAssetIdPostRequest**](InternalWalletsWalletIdAssetIdPostRequest.md)> |  |  |

### Return type

[**models::WalletAsset**](WalletAsset.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## internal_wallets_wallet_id_delete

> internal_wallets_wallet_id_delete(wallet_id)
Delete an internal wallet

Deletes an internal wallet by ID.

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


## internal_wallets_wallet_id_get

> models::UnmanagedWallet internal_wallets_wallet_id_get(wallet_id)
Get assets for internal wallet

Returns all assets in an internal wallet by ID.

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


## internal_wallets_wallet_id_set_customer_ref_id_post

> internal_wallets_wallet_id_set_customer_ref_id_post(wallet_id, vault_accounts_vault_account_id_set_customer_ref_id_post_request)
Set an AML/KYT customer reference ID for an internal wallet

Sets an AML/KYT customer reference ID for the specific internal wallet.

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

