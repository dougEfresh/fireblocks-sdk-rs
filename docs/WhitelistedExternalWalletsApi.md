# \WhitelistedExternalWalletsApi

All URIs are relative to *https://api.fireblocks.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_asset_to_external_wallet**](WhitelistedExternalWalletsApi.md#add_asset_to_external_wallet) | **POST** /external_wallets/{walletId}/{assetId} | Add an asset to an external wallet.
[**create_external_wallet**](WhitelistedExternalWalletsApi.md#create_external_wallet) | **POST** /external_wallets | Create an external wallet
[**delete_external_wallet**](WhitelistedExternalWalletsApi.md#delete_external_wallet) | **DELETE** /external_wallets/{walletId} | Delete an external wallet
[**get_external_wallet**](WhitelistedExternalWalletsApi.md#get_external_wallet) | **GET** /external_wallets/{walletId} | Find an external wallet
[**get_external_wallet_asset**](WhitelistedExternalWalletsApi.md#get_external_wallet_asset) | **GET** /external_wallets/{walletId}/{assetId} | Get an asset from an external wallet
[**get_external_wallets**](WhitelistedExternalWalletsApi.md#get_external_wallets) | **GET** /external_wallets | List external wallets
[**remove_asset_from_external_wallet**](WhitelistedExternalWalletsApi.md#remove_asset_from_external_wallet) | **DELETE** /external_wallets/{walletId}/{assetId} | Delete an asset from an external wallet
[**set_external_wallet_customer_ref_id**](WhitelistedExternalWalletsApi.md#set_external_wallet_customer_ref_id) | **POST** /external_wallets/{walletId}/set_customer_ref_id | Set an AML customer reference ID for an external wallet



## add_asset_to_external_wallet

> models::ExternalWalletAsset add_asset_to_external_wallet(wallet_id, asset_id, idempotency_key, add_asset_to_external_wallet_request)
Add an asset to an external wallet.

Adds an asset to an existing external wallet. </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wallet_id** | **String** | The ID of the wallet | [required] |
**asset_id** | **String** | The ID of the asset to add | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |
**add_asset_to_external_wallet_request** | Option<[**AddAssetToExternalWalletRequest**](AddAssetToExternalWalletRequest.md)> |  |  |

### Return type

[**models::ExternalWalletAsset**](ExternalWalletAsset.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_external_wallet

> models::UnmanagedWallet create_external_wallet(idempotency_key, create_wallet_request)
Create an external wallet

Creates a new external wallet with the requested name.  External Wallet is a whitelisted address of a wallet that belongs to your users/counterparties.  - You cannot see the balance of the external wallet. - You cannot initiate transactions from an external wallet as the source via Fireblocks. Learn more about Whitelisted External Wallet Addresses [here](https://developers.fireblocks.com/docs/whitelist-addresses#external-wallets). </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |
**create_wallet_request** | Option<[**CreateWalletRequest**](CreateWalletRequest.md)> |  |  |

### Return type

[**models::UnmanagedWallet**](UnmanagedWallet.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_external_wallet

> delete_external_wallet(wallet_id)
Delete an external wallet

Deletes an external wallet by ID.  External Wallet is a whitelisted address of a wallet that belongs to your users/counterparties.  - You cannot see the balance of the external wallet. - You cannot initiate transactions from an external wallet as the source via Fireblocks. </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor.

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


## get_external_wallet

> models::UnmanagedWallet get_external_wallet(wallet_id)
Find an external wallet

Returns an external wallet by ID.  External Wallet is a whitelisted address of a wallet that belongs to your users/counterparties.  - You cannot see the balance of the external wallet. - You cannot initiate transactions from an external wallet as the source via Fireblocks. </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor, Viewer.

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
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_external_wallet_asset

> models::ExternalWalletAsset get_external_wallet_asset(wallet_id, asset_id)
Get an asset from an external wallet

Returns an external wallet by wallet ID and asset ID.  External Wallet is a whitelisted address of a wallet that belongs to your users/counterparties.  - You cannot see the balance of the external wallet. - You cannot initiate transactions from an external wallet as the source via Fireblocks. </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor,   Viewer.

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
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_external_wallets

> Vec<models::UnmanagedWallet> get_external_wallets()
List external wallets

Gets a list of external wallets under the workspace.  External Wallet is a whitelisted address of a wallet that belongs to your users/counterparties.  - You cannot see the balance of the external wallet. - You cannot initiate transactions from an external wallet as the source via Fireblocks. </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor, Viewer.

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


## remove_asset_from_external_wallet

> remove_asset_from_external_wallet(wallet_id, asset_id)
Delete an asset from an external wallet

Deletes an external wallet asset by ID. </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor.

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


## set_external_wallet_customer_ref_id

> set_external_wallet_customer_ref_id(wallet_id, set_customer_ref_id_request, idempotency_key)
Set an AML customer reference ID for an external wallet

Sets an AML/KYT customer reference ID for the specific external wallet.  External Wallet is a whitelisted address of a wallet that belongs to your users/counterparties.  - You cannot see the balance of the external wallet. - You cannot initiate transactions from an external wallet as the source via Fireblocks. Learn more about Whitelisted External Wallet Addresses [here](https://developers.fireblocks.com/docs/whitelist-addresses#external-wallets). </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wallet_id** | **String** | The wallet ID | [required] |
**set_customer_ref_id_request** | [**SetCustomerRefIdRequest**](SetCustomerRefIdRequest.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

