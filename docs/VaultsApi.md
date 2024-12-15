# \VaultsApi

All URIs are relative to *https://api.fireblocks.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_vault_account_asset_address**](VaultsApi.md#create_vault_account_asset_address) | **POST** /vault/accounts/{vaultAccountId}/{assetId}/addresses | Create new asset deposit address
[**vault_accounts_get**](VaultsApi.md#vault_accounts_get) | **GET** /vault/accounts | List vault accounts
[**vault_accounts_paged_get**](VaultsApi.md#vault_accounts_paged_get) | **GET** /vault/accounts_paged | List vault accounts (Paginated)
[**vault_accounts_post**](VaultsApi.md#vault_accounts_post) | **POST** /vault/accounts | Create a new vault account
[**vault_accounts_vault_account_id_asset_id_activate_post**](VaultsApi.md#vault_accounts_vault_account_id_asset_id_activate_post) | **POST** /vault/accounts/{vaultAccountId}/{assetId}/activate | Activate a wallet in a vault account.
[**vault_accounts_vault_account_id_asset_id_addresses_address_id_create_legacy_post**](VaultsApi.md#vault_accounts_vault_account_id_asset_id_addresses_address_id_create_legacy_post) | **POST** /vault/accounts/{vaultAccountId}/{assetId}/addresses/{addressId}/create_legacy | Convert a segwit address to legacy format
[**vault_accounts_vault_account_id_asset_id_addresses_address_id_put**](VaultsApi.md#vault_accounts_vault_account_id_asset_id_addresses_address_id_put) | **PUT** /vault/accounts/{vaultAccountId}/{assetId}/addresses/{addressId} | Update address description
[**vault_accounts_vault_account_id_asset_id_addresses_address_id_set_customer_ref_id_post**](VaultsApi.md#vault_accounts_vault_account_id_asset_id_addresses_address_id_set_customer_ref_id_post) | **POST** /vault/accounts/{vaultAccountId}/{assetId}/addresses/{addressId}/set_customer_ref_id | Assign AML customer reference ID
[**vault_accounts_vault_account_id_asset_id_addresses_get**](VaultsApi.md#vault_accounts_vault_account_id_asset_id_addresses_get) | **GET** /vault/accounts/{vaultAccountId}/{assetId}/addresses | Get asset addresses
[**vault_accounts_vault_account_id_asset_id_addresses_paginated_get**](VaultsApi.md#vault_accounts_vault_account_id_asset_id_addresses_paginated_get) | **GET** /vault/accounts/{vaultAccountId}/{assetId}/addresses_paginated | List addresses (Paginated)
[**vault_accounts_vault_account_id_asset_id_balance_post**](VaultsApi.md#vault_accounts_vault_account_id_asset_id_balance_post) | **POST** /vault/accounts/{vaultAccountId}/{assetId}/balance | Refresh asset balance data
[**vault_accounts_vault_account_id_asset_id_change_address_index_public_key_info_get**](VaultsApi.md#vault_accounts_vault_account_id_asset_id_change_address_index_public_key_info_get) | **GET** /vault/accounts/{vaultAccountId}/{assetId}/{change}/{addressIndex}/public_key_info | Get the public key for a vault account
[**vault_accounts_vault_account_id_asset_id_get**](VaultsApi.md#vault_accounts_vault_account_id_asset_id_get) | **GET** /vault/accounts/{vaultAccountId}/{assetId} | Get the asset balance for a vault account
[**vault_accounts_vault_account_id_asset_id_max_spendable_amount_get**](VaultsApi.md#vault_accounts_vault_account_id_asset_id_max_spendable_amount_get) | **GET** /vault/accounts/{vaultAccountId}/{assetId}/max_spendable_amount | Get the maximum spendable amount in a single transaction.
[**vault_accounts_vault_account_id_asset_id_post**](VaultsApi.md#vault_accounts_vault_account_id_asset_id_post) | **POST** /vault/accounts/{vaultAccountId}/{assetId} | Create a new wallet
[**vault_accounts_vault_account_id_asset_id_unspent_inputs_get**](VaultsApi.md#vault_accounts_vault_account_id_asset_id_unspent_inputs_get) | **GET** /vault/accounts/{vaultAccountId}/{assetId}/unspent_inputs | Get UTXO unspent inputs information
[**vault_accounts_vault_account_id_get**](VaultsApi.md#vault_accounts_vault_account_id_get) | **GET** /vault/accounts/{vaultAccountId} | Find a vault account by ID
[**vault_accounts_vault_account_id_hide_post**](VaultsApi.md#vault_accounts_vault_account_id_hide_post) | **POST** /vault/accounts/{vaultAccountId}/hide | Hide a vault account in the console
[**vault_accounts_vault_account_id_put**](VaultsApi.md#vault_accounts_vault_account_id_put) | **PUT** /vault/accounts/{vaultAccountId} | Rename a vault account
[**vault_accounts_vault_account_id_set_auto_fuel_post**](VaultsApi.md#vault_accounts_vault_account_id_set_auto_fuel_post) | **POST** /vault/accounts/{vaultAccountId}/set_auto_fuel | Turn autofueling on or off
[**vault_accounts_vault_account_id_set_customer_ref_id_post**](VaultsApi.md#vault_accounts_vault_account_id_set_customer_ref_id_post) | **POST** /vault/accounts/{vaultAccountId}/set_customer_ref_id | Set an AML/KYT customer reference ID for a vault account
[**vault_accounts_vault_account_id_unhide_post**](VaultsApi.md#vault_accounts_vault_account_id_unhide_post) | **POST** /vault/accounts/{vaultAccountId}/unhide | Unhide a vault account in the console
[**vault_asset_wallets_get**](VaultsApi.md#vault_asset_wallets_get) | **GET** /vault/asset_wallets | List asset wallets (Paginated)
[**vault_assets_asset_id_get**](VaultsApi.md#vault_assets_asset_id_get) | **GET** /vault/assets/{assetId} | Get vault balance by asset
[**vault_assets_get**](VaultsApi.md#vault_assets_get) | **GET** /vault/assets | Get asset balance for chosen assets
[**vault_public_key_info_get**](VaultsApi.md#vault_public_key_info_get) | **GET** /vault/public_key_info/ | Get the public key information



## create_vault_account_asset_address

> models::CreateAddressResponse create_vault_account_asset_address(vault_account_id, asset_id, create_vault_account_asset_address_request)
Create new asset deposit address

Creates a new deposit address for an asset of a vault account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vault_account_id** | **String** | The ID of the vault account to return | [required] |
**asset_id** | **String** | The ID of the asset | [required] |
**create_vault_account_asset_address_request** | Option<[**CreateVaultAccountAssetAddressRequest**](CreateVaultAccountAssetAddressRequest.md)> |  |  |

### Return type

[**models::CreateAddressResponse**](CreateAddressResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vault_accounts_get

> Vec<models::VaultAccount> vault_accounts_get(name_prefix, name_suffix, min_amount_threshold, asset_id)
List vault accounts

Gets all vault accounts in your workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name_prefix** | Option<**String**> |  |  |
**name_suffix** | Option<**String**> |  |  |
**min_amount_threshold** | Option<**f64**> |  |  |
**asset_id** | Option<**String**> |  |  |

### Return type

[**Vec<models::VaultAccount>**](VaultAccount.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vault_accounts_paged_get

> models::VaultAccountsPagedResponse vault_accounts_paged_get(name_prefix, name_suffix, min_amount_threshold, asset_id, order_by, before, after, limit)
List vault accounts (Paginated)

Gets all vault accounts in your workspace. This endpoint returns a limited amount of results with a quick response time.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name_prefix** | Option<**String**> |  |  |
**name_suffix** | Option<**String**> |  |  |
**min_amount_threshold** | Option<**f64**> | Specifying minAmountThreshold will filter accounts with balances greater than this value, otherwise, it will return all accounts. |  |
**asset_id** | Option<**String**> |  |  |
**order_by** | Option<**String**> |  |  |[default to DESC]
**before** | Option<**String**> |  |  |
**after** | Option<**String**> |  |  |
**limit** | Option<**f64**> |  |  |[default to 200]

### Return type

[**models::VaultAccountsPagedResponse**](VaultAccountsPagedResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vault_accounts_post

> models::VaultAccount vault_accounts_post(vault_accounts_post_request)
Create a new vault account

Creates a new vault account with the requested name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vault_accounts_post_request** | [**VaultAccountsPostRequest**](VaultAccountsPostRequest.md) |  | [required] |

### Return type

[**models::VaultAccount**](VaultAccount.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vault_accounts_vault_account_id_asset_id_activate_post

> models::CreateVaultAssetResponse vault_accounts_vault_account_id_asset_id_activate_post(vault_account_id, asset_id)
Activate a wallet in a vault account.

Initiates activation for a wallet in a vault account. Activation is required for tokens that need an on-chain transaction for creation (XLM tokens, SOL tokens etc).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vault_account_id** | **String** | The ID of the vault account to return, or 'default' for the default vault account | [required] |
**asset_id** | **String** | The ID of the asset | [required] |

### Return type

[**models::CreateVaultAssetResponse**](CreateVaultAssetResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vault_accounts_vault_account_id_asset_id_addresses_address_id_create_legacy_post

> models::CreateAddressResponse vault_accounts_vault_account_id_asset_id_addresses_address_id_create_legacy_post(vault_account_id, asset_id, address_id)
Convert a segwit address to legacy format

Converts an existing segwit address to the legacy format.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vault_account_id** | **String** | The ID of the vault account | [required] |
**asset_id** | **String** | The ID of the asset | [required] |
**address_id** | **String** | The segwit address to translate | [required] |

### Return type

[**models::CreateAddressResponse**](CreateAddressResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vault_accounts_vault_account_id_asset_id_addresses_address_id_put

> vault_accounts_vault_account_id_asset_id_addresses_address_id_put(vault_account_id, asset_id, address_id, vault_accounts_vault_account_id_asset_id_addresses_address_id_put_request)
Update address description

Updates the description of an existing address of an asset in a vault account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vault_account_id** | **String** | The ID of the vault account | [required] |
**asset_id** | **String** | The ID of the asset | [required] |
**address_id** | **String** | The address for which to add a description. For XRP, use <address>:<tag>, for all other assets, use only the address | [required] |
**vault_accounts_vault_account_id_asset_id_addresses_address_id_put_request** | Option<[**VaultAccountsVaultAccountIdAssetIdAddressesAddressIdPutRequest**](VaultAccountsVaultAccountIdAssetIdAddressesAddressIdPutRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vault_accounts_vault_account_id_asset_id_addresses_address_id_set_customer_ref_id_post

> vault_accounts_vault_account_id_asset_id_addresses_address_id_set_customer_ref_id_post(vault_account_id, asset_id, address_id, vault_accounts_vault_account_id_set_customer_ref_id_post_request)
Assign AML customer reference ID

Sets an AML/KYT customer reference ID for a specific address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vault_account_id** | **String** | The ID of the vault account | [required] |
**asset_id** | **String** | The ID of the asset | [required] |
**address_id** | **String** | The address for which to add a description. For XRP, use <address>:<tag>, for all other assets, use only the address | [required] |
**vault_accounts_vault_account_id_set_customer_ref_id_post_request** | [**VaultAccountsVaultAccountIdSetCustomerRefIdPostRequest**](VaultAccountsVaultAccountIdSetCustomerRefIdPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vault_accounts_vault_account_id_asset_id_addresses_get

> Vec<models::VaultWalletAddress> vault_accounts_vault_account_id_asset_id_addresses_get(vault_account_id, asset_id)
Get asset addresses

Lists all addresses for specific asset of vault account. </br> - **This endpoint will be deprecated on Mar 31, 2024.** - If your application logic or scripts rely on the deprecated endpoint, you should update to account for `GET/V1/vault/accounts/{vaultAccountId}/{assetId}/addresses_paginated` before Mar 31, 2024. - All workspaces created after Mar 31, 2024. will have it disabled. If it is disabled for your workspace and you attempt to use it, you will receive the following error message: 'This endpoint is unavailable.' - Please use the `GET/V1/vault/accounts/{vaultAccountId}/{assetId}/addresses_paginated endpoint` to return all the wallet addresses associated with the specified vault account and asset in a paginated list. - This API call is subject to rate limits.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vault_account_id** | **String** | The ID of the vault account to return | [required] |
**asset_id** | **String** | The ID of the asset | [required] |

### Return type

[**Vec<models::VaultWalletAddress>**](VaultWalletAddress.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vault_accounts_vault_account_id_asset_id_addresses_paginated_get

> models::AddressesResponse vault_accounts_vault_account_id_asset_id_addresses_paginated_get(vault_account_id, asset_id, limit, before, after)
List addresses (Paginated)

Returns a paginated response of the addresses for a given vault account and asset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vault_account_id** | **String** |  | [required] |
**asset_id** | **String** |  | [required] |
**limit** | Option<**f64**> |  |  |
**before** | Option<**String**> |  |  |
**after** | Option<**String**> |  |  |

### Return type

[**models::AddressesResponse**](AddressesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vault_accounts_vault_account_id_asset_id_balance_post

> models::VaultAsset vault_accounts_vault_account_id_asset_id_balance_post(vault_account_id, asset_id, body)
Refresh asset balance data

Updates the balance of a specific asset in a vault account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vault_account_id** | **String** | The ID of the vault account to return | [required] |
**asset_id** | **String** | The ID of the asset | [required] |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

[**models::VaultAsset**](VaultAsset.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vault_accounts_vault_account_id_asset_id_change_address_index_public_key_info_get

> models::PublicKeyInformation vault_accounts_vault_account_id_asset_id_change_address_index_public_key_info_get(vault_account_id, asset_id, change, address_index, compressed)
Get the public key for a vault account

Gets the public key information for the vault account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vault_account_id** | **String** | The ID of the vault account | [required] |
**asset_id** | **String** | The ID of the asset. | [required] |
**change** | **f64** | Whether the address should be derived internal (change) or not. | [required] |
**address_index** | **f64** | The index of the address for the derivation path. | [required] |
**compressed** | Option<**bool**> |  |  |

### Return type

[**models::PublicKeyInformation**](PublicKeyInformation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vault_accounts_vault_account_id_asset_id_get

> models::VaultAsset vault_accounts_vault_account_id_asset_id_get(vault_account_id, asset_id)
Get the asset balance for a vault account

Returns a wallet for a specific asset of a vault account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vault_account_id** | **String** | The ID of the vault account to return | [required] |
**asset_id** | **String** | The ID of the asset | [required] |

### Return type

[**models::VaultAsset**](VaultAsset.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vault_accounts_vault_account_id_asset_id_max_spendable_amount_get

> vault_accounts_vault_account_id_asset_id_max_spendable_amount_get(vault_account_id, asset_id, manual_signging)
Get the maximum spendable amount in a single transaction.

Get the maximum amount of a particular asset that can be spent in a single transaction from a specified vault account (UTXO assets only, with a limitation on number of inputs embedded). Send several transactions if you want to spend more than the maximum spendable amount.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vault_account_id** | **String** | The ID of the vault account, or 'default' for the default vault account | [required] |
**asset_id** | **String** | The ID of the asset | [required] |
**manual_signging** | Option<**bool**> | False by default. The maximum number of inputs depends if the transaction will be signed by an automated co-signer server or on a mobile device. |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vault_accounts_vault_account_id_asset_id_post

> models::CreateVaultAssetResponse vault_accounts_vault_account_id_asset_id_post(vault_account_id, asset_id, vault_accounts_vault_account_id_asset_id_post_request)
Create a new wallet

Creates a wallet for a specific asset in a vault account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vault_account_id** | **String** | The ID of the vault account to return, or 'default' for the default vault account | [required] |
**asset_id** | **String** | The ID of the asset | [required] |
**vault_accounts_vault_account_id_asset_id_post_request** | Option<[**VaultAccountsVaultAccountIdAssetIdPostRequest**](VaultAccountsVaultAccountIdAssetIdPostRequest.md)> |  |  |

### Return type

[**models::CreateVaultAssetResponse**](CreateVaultAssetResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vault_accounts_vault_account_id_asset_id_unspent_inputs_get

> Vec<models::UnspentInputsResponse> vault_accounts_vault_account_id_asset_id_unspent_inputs_get(vault_account_id, asset_id)
Get UTXO unspent inputs information

Returns unspent inputs information of an asset in a vault account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vault_account_id** | **String** | The ID of the vault account | [required] |
**asset_id** | **String** | The ID of the asset | [required] |

### Return type

[**Vec<models::UnspentInputsResponse>**](UnspentInputsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vault_accounts_vault_account_id_get

> models::VaultAccount vault_accounts_vault_account_id_get(vault_account_id)
Find a vault account by ID

Returns the requested vault account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vault_account_id** | **String** | The ID of the vault account to return type: string | [required] |

### Return type

[**models::VaultAccount**](VaultAccount.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vault_accounts_vault_account_id_hide_post

> vault_accounts_vault_account_id_hide_post(vault_account_id)
Hide a vault account in the console

Hides the requested vault account from the web console view.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vault_account_id** | **String** | The vault account to hide | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vault_accounts_vault_account_id_put

> vault_accounts_vault_account_id_put(vault_account_id, vault_accounts_vault_account_id_put_request)
Rename a vault account

Renames the requested vault account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vault_account_id** | **String** | The ID of the vault account to edit | [required] |
**vault_accounts_vault_account_id_put_request** | [**VaultAccountsVaultAccountIdPutRequest**](VaultAccountsVaultAccountIdPutRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vault_accounts_vault_account_id_set_auto_fuel_post

> vault_accounts_vault_account_id_set_auto_fuel_post(vault_account_id, vault_accounts_vault_account_id_set_auto_fuel_post_request)
Turn autofueling on or off

Sets the autofueling property of the vault account to enabled or disabled.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vault_account_id** | **String** | The vault account ID | [required] |
**vault_accounts_vault_account_id_set_auto_fuel_post_request** | [**VaultAccountsVaultAccountIdSetAutoFuelPostRequest**](VaultAccountsVaultAccountIdSetAutoFuelPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vault_accounts_vault_account_id_set_customer_ref_id_post

> vault_accounts_vault_account_id_set_customer_ref_id_post(vault_account_id, vault_accounts_vault_account_id_set_customer_ref_id_post_request)
Set an AML/KYT customer reference ID for a vault account

Assigns an AML/KYT customer reference ID for the vault account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vault_account_id** | **String** | The vault account ID | [required] |
**vault_accounts_vault_account_id_set_customer_ref_id_post_request** | [**VaultAccountsVaultAccountIdSetCustomerRefIdPostRequest**](VaultAccountsVaultAccountIdSetCustomerRefIdPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vault_accounts_vault_account_id_unhide_post

> vault_accounts_vault_account_id_unhide_post(vault_account_id)
Unhide a vault account in the console

Makes a hidden vault account visible in web console view.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vault_account_id** | **String** | The vault account to unhide | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vault_asset_wallets_get

> models::PaginatedAssetWalletResponse vault_asset_wallets_get(total_amount_larger_than, asset_id, order_by, before, after, limit)
List asset wallets (Paginated)

Gets all asset wallets at all of the vault accounts in your workspace. An asset wallet is an asset at a vault account. This method allows fast traversal of all account balances. **Note:**   - This API call is subject to [rate limits](https://developers.fireblocks.com/reference/rate-limiting). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**total_amount_larger_than** | Option<**f64**> | When specified, only asset wallets with total balance larger than this amount are returned. |  |
**asset_id** | Option<**String**> | When specified, only asset wallets cross vault accounts that have this asset ID are returned. |  |
**order_by** | Option<**String**> |  |  |[default to DESC]
**before** | Option<**String**> | Fetches the next paginated response before this element. This element is a cursor and is returned at the response of the previous page. |  |
**after** | Option<**String**> | Fetches the next paginated response after this element. This element is a cursor and is returned at the response of the previous page. |  |
**limit** | Option<**f64**> | The maximum number of asset wallets in a single response. The default is 200 and the maximum is 1000. |  |[default to 200]

### Return type

[**models::PaginatedAssetWalletResponse**](PaginatedAssetWalletResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vault_assets_asset_id_get

> models::VaultAsset vault_assets_asset_id_get(asset_id)
Get vault balance by asset

Gets the vault balance summary for an asset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_id** | **String** |  | [required] |

### Return type

[**models::VaultAsset**](VaultAsset.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vault_assets_get

> Vec<models::VaultAsset> vault_assets_get(account_name_prefix, account_name_suffix)
Get asset balance for chosen assets

Gets the assets amount summary for all accounts or filtered accounts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_name_prefix** | Option<**String**> |  |  |
**account_name_suffix** | Option<**String**> |  |  |

### Return type

[**Vec<models::VaultAsset>**](VaultAsset.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vault_public_key_info_get

> models::PublicKeyInformation vault_public_key_info_get(derivation_path, algorithm, compressed)
Get the public key information

Gets the public key information based on derivation path and signing algorithm.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**derivation_path** | [**Vec<i32>**](i32.md) | An array of integers representing the full BIP44 derivation path of the requested public key.  The first element must always be 44.  | [required] |
**algorithm** | **String** |  | [required] |
**compressed** | Option<**bool**> |  |  |

### Return type

[**models::PublicKeyInformation**](PublicKeyInformation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

