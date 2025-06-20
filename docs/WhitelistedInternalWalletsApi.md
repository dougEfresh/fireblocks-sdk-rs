# \WhitelistedInternalWalletsApi

All URIs are relative to *https://api.fireblocks.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_internal_wallet**](WhitelistedInternalWalletsApi.md#create_internal_wallet) | **POST** /internal_wallets | Create an internal wallet
[**create_internal_wallet_asset**](WhitelistedInternalWalletsApi.md#create_internal_wallet_asset) | **POST** /internal_wallets/{walletId}/{assetId} | Add an asset to an internal wallet
[**delete_internal_wallet**](WhitelistedInternalWalletsApi.md#delete_internal_wallet) | **DELETE** /internal_wallets/{walletId} | Delete an internal wallet
[**delete_internal_wallet_asset**](WhitelistedInternalWalletsApi.md#delete_internal_wallet_asset) | **DELETE** /internal_wallets/{walletId}/{assetId} | Delete a whitelisted address
[**get_internal_wallet**](WhitelistedInternalWalletsApi.md#get_internal_wallet) | **GET** /internal_wallets/{walletId} | Get assets for internal wallet
[**get_internal_wallet_asset**](WhitelistedInternalWalletsApi.md#get_internal_wallet_asset) | **GET** /internal_wallets/{walletId}/{assetId} | Get an asset from an internal wallet
[**get_internal_wallet_assets_paginated**](WhitelistedInternalWalletsApi.md#get_internal_wallet_assets_paginated) | **GET** /internal_wallets/{walletId}/assets | List assets in an internal wallet (Paginated)
[**get_internal_wallets**](WhitelistedInternalWalletsApi.md#get_internal_wallets) | **GET** /internal_wallets | List internal wallets
[**set_customer_ref_id_for_internal_wallet**](WhitelistedInternalWalletsApi.md#set_customer_ref_id_for_internal_wallet) | **POST** /internal_wallets/{walletId}/set_customer_ref_id | Set an AML/KYT customer reference ID for internal wallet



## create_internal_wallet

> models::UnmanagedWallet create_internal_wallet(idempotency_key, create_wallet_request)
Create an internal wallet

Creates a new internal wallet with the requested name. Learn more about Whitelisted Internal Addresses [here](https://developers.fireblocks.com/docs/whitelist-addresses#internal-wallets) </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor.

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


## create_internal_wallet_asset

> models::WalletAsset create_internal_wallet_asset(wallet_id, asset_id, idempotency_key, create_internal_wallet_asset_request)
Add an asset to an internal wallet

Adds an asset to an existing internal wallet.  Internal Wallets are whitelisted wallets that belong to you outside of Fireblocks.    - You can see the balance of the Internal Wallet via Fireblocks   - You cannot initiate transactions from Internal Wallets through Fireblocks    Learn more about Whitelisted Internal Addresses [here](https://developers.fireblocks.com/docs/whitelist-addresses#internal-wallets) </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wallet_id** | **String** | The ID of the wallet | [required] |
**asset_id** | **String** | The ID of the asset to add | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |
**create_internal_wallet_asset_request** | Option<[**CreateInternalWalletAssetRequest**](CreateInternalWalletAssetRequest.md)> |  |  |

### Return type

[**models::WalletAsset**](WalletAsset.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_internal_wallet

> delete_internal_wallet(wallet_id)
Delete an internal wallet

Deletes an internal wallet by ID.  Internal Wallets are whitelisted wallets that belong to you outside of Fireblocks.    - You can see the balance of the Internal Wallet via Fireblocks   - You cannot initiate transactions from Internal Wallets through Fireblocks  </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor.

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


## delete_internal_wallet_asset

> delete_internal_wallet_asset(wallet_id, asset_id)
Delete a whitelisted address

Deletes a whitelisted address (for an asset) from an internal wallet.  Internal Wallets are whitelisted wallets that belong to you outside of Fireblocks.    - You can see the balance of the Internal Wallet via Fireblocks   - You cannot initiate transactions from Internal Wallets through Fireblocks  </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor.

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


## get_internal_wallet

> models::UnmanagedWallet get_internal_wallet(wallet_id)
Get assets for internal wallet

Returns information for an asset in an internal wallet. This endpoint will be deprecated after 6 months. As part of the depreciation process this endpoint will no longer return balances, only addresses. Until it is deprecated, this endpoint will behave the same way. Internal Wallets are whitelisted wallets that belong to you outside of Fireblocks.    - You can see the balance of the Internal Wallet via Fireblocks   - You cannot initiate transactions from Internal Wallets through Fireblocks  </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor, Viewer.

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


## get_internal_wallet_asset

> models::WalletAsset get_internal_wallet_asset(wallet_id, asset_id)
Get an asset from an internal wallet

Returns information for an asset in an internal wallet.  Internal Wallets are whitelisted wallets that belong to you outside of Fireblocks.    - You can see the balance of the Internal Wallet via Fireblocks   - You cannot initiate transactions from Internal Wallets through Fireblocks  </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor, Viewer.

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
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_internal_wallet_assets_paginated

> models::PaginatedAssetsResponse get_internal_wallet_assets_paginated(wallet_id, page_size, page_cursor)
List assets in an internal wallet (Paginated)

Returns a paginated response of assets in an internal wallet.  This is a new paginated endpoint that gets all the assets from the wallet container with balances. </br>This endpoint returns a limited amount of results with a quick response time.  Internal Wallets are whitelisted wallets that belong to you outside of Fireblocks.    - You can see the balance of the Internal Wallet via Fireblocks   - You cannot initiate transactions from Internal Wallets through Fireblocks  Learn more about Whitelisted Internal Addresses [here](https://developers.fireblocks.com/docs/whitelist-addresses#internal-wallets)  Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor, Viewer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wallet_id** | **String** | The ID of the internal wallet to return assets for | [required] |
**page_size** | Option<**f64**> |  |  |[default to 50]
**page_cursor** | Option<**String**> |  |  |

### Return type

[**models::PaginatedAssetsResponse**](PaginatedAssetsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_internal_wallets

> Vec<models::UnmanagedWallet> get_internal_wallets()
List internal wallets

Gets a list of internal wallets. **Note**:  - BTC-based assets belonging to whitelisted addresses cannot be retrieved between 00:00 UTC and 00:01 UTC daily due to third-party provider, Blockchain, being unavailable for this 60 second period.  Please wait until the next minute to retrieve BTC-based assets. - The list of assets returned will NOT include the balances anymore. Internal Wallets are whitelisted wallets that belong to you outside of Fireblocks. - You can see the balance of the Internal Wallet via Fireblocks - You cannot initiate transactions from Internal Wallets through Fireblocks Learn more about Whitelisted Internal Addresses [here](https://developers.fireblocks.com/docs/whitelist-addresses#internal-wallets) </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor, Viewer. 

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


## set_customer_ref_id_for_internal_wallet

> set_customer_ref_id_for_internal_wallet(wallet_id, set_customer_ref_id_request, idempotency_key)
Set an AML/KYT customer reference ID for internal wallet

Sets an AML/KYT customer reference ID for the specific internal wallet.  Internal Wallets are whitelisted wallets that belong to you outside of Fireblocks.    - You can see the balance of the Internal Wallet via Fireblocks   - You cannot initiate transactions from Internal Wallets through Fireblocks    Learn more about Whitelisted Internal Addresses [here](https://developers.fireblocks.com/docs/whitelist-addresses#internal-wallets) </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor.

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

