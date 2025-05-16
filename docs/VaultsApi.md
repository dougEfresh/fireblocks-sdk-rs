# \VaultsApi

All URIs are relative to *https://api.fireblocks.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**activate_asset_for_vault_account**](VaultsApi.md#activate_asset_for_vault_account) | **POST** /vault/accounts/{vaultAccountId}/{assetId}/activate | Activate a wallet in a vault account
[**create_legacy_address**](VaultsApi.md#create_legacy_address) | **POST** /vault/accounts/{vaultAccountId}/{assetId}/addresses/{addressId}/create_legacy | Convert a segwit address to legacy format
[**create_multiple_deposit_addresses**](VaultsApi.md#create_multiple_deposit_addresses) | **POST** /vault/accounts/addresses/bulk | Bulk creation of new deposit addresses
[**create_vault_account**](VaultsApi.md#create_vault_account) | **POST** /vault/accounts | Create a new vault account
[**create_vault_account_asset**](VaultsApi.md#create_vault_account_asset) | **POST** /vault/accounts/{vaultAccountId}/{assetId} | Create a new vault wallet
[**create_vault_account_asset_address**](VaultsApi.md#create_vault_account_asset_address) | **POST** /vault/accounts/{vaultAccountId}/{assetId}/addresses | Create new asset deposit address
[**get_asset_wallets**](VaultsApi.md#get_asset_wallets) | **GET** /vault/asset_wallets | Get vault wallets (Paginated)
[**get_create_multiple_deposit_addresses_job_status**](VaultsApi.md#get_create_multiple_deposit_addresses_job_status) | **GET** /vault/accounts/addresses/bulk/{jobId} | Get the job status of the bulk deposit address creation
[**get_max_spendable_amount**](VaultsApi.md#get_max_spendable_amount) | **GET** /vault/accounts/{vaultAccountId}/{assetId}/max_spendable_amount | Get the max spendable amount in a transaction.
[**get_paged_vault_accounts**](VaultsApi.md#get_paged_vault_accounts) | **GET** /vault/accounts_paged | Get vault accounts (Paginated)
[**get_public_key_info**](VaultsApi.md#get_public_key_info) | **GET** /vault/public_key_info | Get the public key for a derivation path
[**get_public_key_info_for_address**](VaultsApi.md#get_public_key_info_for_address) | **GET** /vault/accounts/{vaultAccountId}/{assetId}/{change}/{addressIndex}/public_key_info | Get an asset's public key
[**get_unspent_inputs**](VaultsApi.md#get_unspent_inputs) | **GET** /vault/accounts/{vaultAccountId}/{assetId}/unspent_inputs | Get UTXO unspent inputs information
[**get_vault_account**](VaultsApi.md#get_vault_account) | **GET** /vault/accounts/{vaultAccountId} | Get a vault account by ID
[**get_vault_account_asset**](VaultsApi.md#get_vault_account_asset) | **GET** /vault/accounts/{vaultAccountId}/{assetId} | Get the asset balance for a vault account
[**get_vault_account_asset_addresses**](VaultsApi.md#get_vault_account_asset_addresses) | **GET** /vault/accounts/{vaultAccountId}/{assetId}/addresses | Get asset addresses
[**get_vault_account_asset_addresses_paginated**](VaultsApi.md#get_vault_account_asset_addresses_paginated) | **GET** /vault/accounts/{vaultAccountId}/{assetId}/addresses_paginated | Get addresses (Paginated)
[**get_vault_assets**](VaultsApi.md#get_vault_assets) | **GET** /vault/assets | Get asset balance for chosen assets
[**get_vault_balance_by_asset**](VaultsApi.md#get_vault_balance_by_asset) | **GET** /vault/assets/{assetId} | Get vault balance by an asset
[**hide_vault_account**](VaultsApi.md#hide_vault_account) | **POST** /vault/accounts/{vaultAccountId}/hide | Hide a vault account in the console
[**set_customer_ref_id_for_address**](VaultsApi.md#set_customer_ref_id_for_address) | **POST** /vault/accounts/{vaultAccountId}/{assetId}/addresses/{addressId}/set_customer_ref_id | Assign AML customer reference ID
[**set_vault_account_auto_fuel**](VaultsApi.md#set_vault_account_auto_fuel) | **POST** /vault/accounts/{vaultAccountId}/set_auto_fuel | Set auto fueling to on or off
[**set_vault_account_customer_ref_id**](VaultsApi.md#set_vault_account_customer_ref_id) | **POST** /vault/accounts/{vaultAccountId}/set_customer_ref_id | Set an AML/KYT ID for a vault account
[**unhide_vault_account**](VaultsApi.md#unhide_vault_account) | **POST** /vault/accounts/{vaultAccountId}/unhide | Unhide a vault account in the console
[**update_vault_account**](VaultsApi.md#update_vault_account) | **PUT** /vault/accounts/{vaultAccountId} | Rename a vault account
[**update_vault_account_asset_address**](VaultsApi.md#update_vault_account_asset_address) | **PUT** /vault/accounts/{vaultAccountId}/{assetId}/addresses/{addressId} | Update address description
[**update_vault_account_asset_balance**](VaultsApi.md#update_vault_account_asset_balance) | **POST** /vault/accounts/{vaultAccountId}/{assetId}/balance | Refresh asset balance data



## activate_asset_for_vault_account

> models::CreateVaultAssetResponse activate_asset_for_vault_account(vault_account_id, asset_id, idempotency_key)
Activate a wallet in a vault account

Initiates activation for a wallet in a vault account.  Activation is required for tokens that need an on-chain transaction for creation (XLM tokens, SOL tokens etc). </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vault_account_id** | **String** | The ID of the vault account | [required] |
**asset_id** | **String** | The ID of the asset | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::CreateVaultAssetResponse**](CreateVaultAssetResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_legacy_address

> models::CreateAddressResponse create_legacy_address(vault_account_id, asset_id, address_id, idempotency_key)
Convert a segwit address to legacy format

Converts an existing segwit address to the legacy format. </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vault_account_id** | **String** | The ID of the vault account | [required] |
**asset_id** | **String** | The ID of the asset | [required] |
**address_id** | **String** | The segwit address to translate | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::CreateAddressResponse**](CreateAddressResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_multiple_deposit_addresses

> models::JobCreated create_multiple_deposit_addresses(create_multiple_deposit_addresses_request, idempotency_key)
Bulk creation of new deposit addresses

Create multiple deposit address by running an async job. </br> **Note**: - We limit accounts to 10k per operation. - The target Vault Account should already have the asset wallet created, or the deposit addresses will fail. - This endpoint should be used for UTXO blockchains. - This endpoint is currently in Early Availability. Please contact CSM to get access to this endpoint.   Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_multiple_deposit_addresses_request** | [**CreateMultipleDepositAddressesRequest**](CreateMultipleDepositAddressesRequest.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::JobCreated**](JobCreated.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_vault_account

> models::VaultAccount create_vault_account(create_vault_account_request, idempotency_key)
Create a new vault account

Creates a new vault account with the requested name. **Note: ** Vault account names should consist of ASCII characters only. Learn more about Fireblocks Vault Accounts in the following [guide](https://developers.fireblocks.com/reference/create-vault-account). </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_vault_account_request** | [**CreateVaultAccountRequest**](CreateVaultAccountRequest.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::VaultAccount**](VaultAccount.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_vault_account_asset

> models::CreateVaultAssetResponse create_vault_account_asset(vault_account_id, asset_id, idempotency_key, create_assets_request)
Create a new vault wallet

Creates a wallet for a specific asset in a vault account. Learn more about Fireblocks Vault Wallets in the following [guide](https://developers.fireblocks.com/reference/create-vault-wallet). </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vault_account_id** | **String** | The ID of the vault account to return, or 'default' for the default vault account | [required] |
**asset_id** | **String** | The ID of the asset | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |
**create_assets_request** | Option<[**CreateAssetsRequest**](CreateAssetsRequest.md)> |  |  |

### Return type

[**models::CreateVaultAssetResponse**](CreateVaultAssetResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_vault_account_asset_address

> models::CreateAddressResponse create_vault_account_asset_address(vault_account_id, asset_id, idempotency_key, create_address_request)
Create new asset deposit address

Creates a new deposit address for an asset of a vault account. Should be used for UTXO or Tag/Memo based assets ONLY.  Requests with account based assets will fail.  </br>Endpoint Permission: Admin, Non-Signing Admin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vault_account_id** | **String** | The ID of the vault account to return | [required] |
**asset_id** | **String** | The ID of the asset | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |
**create_address_request** | Option<[**CreateAddressRequest**](CreateAddressRequest.md)> |  |  |

### Return type

[**models::CreateAddressResponse**](CreateAddressResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_asset_wallets

> models::PaginatedAssetWalletResponse get_asset_wallets(total_amount_larger_than, asset_id, order_by, before, after, limit)
Get vault wallets (Paginated)

Get all vault wallets of the vault accounts in your workspace.  A vault wallet is an asset in a vault account.   This method allows fast traversal of all account balances. </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor, Viewer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**total_amount_larger_than** | Option<**f64**> | When specified, only vault wallets with total balance greater than this amount are returned. |  |
**asset_id** | Option<**String**> | When specified, only vault wallets with the specified ID are returned. |  |
**order_by** | Option<**String**> |  |  |[default to DESC]
**before** | Option<**String**> | Fetches the next paginated response before this element.  This element is a cursor and is returned at the response of the previous page.  |  |
**after** | Option<**String**> | Fetches the next paginated response after this element. This element is a cursor and is returned at the response of the previous page. |  |
**limit** | Option<**f64**> | The maximum number of vault wallets in a single response.   The default is 200 and the maximum is 1000.  |  |[default to 200]

### Return type

[**models::PaginatedAssetWalletResponse**](PaginatedAssetWalletResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_create_multiple_deposit_addresses_job_status

> models::CreateMultipleDepositAddressesJobStatus get_create_multiple_deposit_addresses_job_status(job_id)
Get the job status of the bulk deposit address creation

Returns the status of the bulk creation of new deposit addresses job, and the result or error. **Note**: - The target Vault Account should already have the asset wallet created, or the deposit addresses will fail. - This endpoint is currently in Early Availability. Please contact CSM to get access to this endpoint. Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | The ID of the job to create addresses | [required] |

### Return type

[**models::CreateMultipleDepositAddressesJobStatus**](CreateMultipleDepositAddressesJobStatus.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_max_spendable_amount

> models::GetMaxSpendableAmountResponse get_max_spendable_amount(vault_account_id, asset_id, manual_signging)
Get the max spendable amount in a transaction.

Get the maximum amount of a particular asset that can be spent in a single transaction from a specified vault account (UTXO assets only). </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor, Viewer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vault_account_id** | **String** | The ID of the vault account, or 'default' for the default vault account | [required] |
**asset_id** | **String** | The ID of the asset | [required] |
**manual_signging** | Option<**bool**> | False by default. The maximum number of inputs depends if the transaction will be signed by an automated co-signer server or on a mobile device. |  |

### Return type

[**models::GetMaxSpendableAmountResponse**](GetMaxSpendableAmountResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_paged_vault_accounts

> models::VaultAccountsPagedResponse get_paged_vault_accounts(name_prefix, name_suffix, min_amount_threshold, asset_id, order_by, before, after, limit)
Get vault accounts (Paginated)

Gets all vault accounts in your workspace. This endpoint returns a limited amount of results with a quick response time. </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor, Viewer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name_prefix** | Option<**String**> |  |  |
**name_suffix** | Option<**String**> |  |  |
**min_amount_threshold** | Option<**f64**> | Specifying minAmountThreshold will filter accounts with balances greater than this value, otherwise, it will return all accounts. The amount set in this parameter is the native asset amount and not its USD value. |  |
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
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_public_key_info

> models::PublicKeyInformation get_public_key_info(derivation_path, algorithm, compressed)
Get the public key for a derivation path

Gets the public key information based on derivation path and signing algorithm. </br>Endpoint Permission: Admin, Non-Signing Admin.

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
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_public_key_info_for_address

> models::PublicKeyInformation get_public_key_info_for_address(vault_account_id, asset_id, change, address_index, compressed)
Get an asset's public key

Get the public key information for a specific asset in a vault account. </br>Endpoint Permission: Admin, Non-Signing Admin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vault_account_id** | **String** |  | [required] |
**asset_id** | **String** |  | [required] |
**change** | **f64** | BIP44 derivation path - change value | [required] |
**address_index** | **f64** | BIP44 derivation path - index value | [required] |
**compressed** | Option<**bool**> | Compressed/Uncompressed public key format |  |

### Return type

[**models::PublicKeyInformation**](PublicKeyInformation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_unspent_inputs

> Vec<models::UnspentInputsResponse> get_unspent_inputs(vault_account_id, asset_id)
Get UTXO unspent inputs information

Returns unspent inputs information of an UTXO asset in a vault account.  </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor, Viewer.

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
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_vault_account

> models::VaultAccount get_vault_account(vault_account_id)
Get a vault account by ID

Get a vault account by its unique ID. </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor, Viewer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vault_account_id** | **String** | The ID of the vault account | [required] |

### Return type

[**models::VaultAccount**](VaultAccount.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_vault_account_asset

> models::VaultAsset get_vault_account_asset(vault_account_id, asset_id)
Get the asset balance for a vault account

Returns a specific vault wallet balance information for a specific asset.  </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor,   Viewer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vault_account_id** | **String** | The ID of the vault account | [required] |
**asset_id** | **String** | The ID of the asset | [required] |

### Return type

[**models::VaultAsset**](VaultAsset.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_vault_account_asset_addresses

> Vec<models::VaultWalletAddress> get_vault_account_asset_addresses(vault_account_id, asset_id)
Get asset addresses

DEPRECATED!  - If your application logic or scripts rely on the deprecated endpoint, you should update to account for GET/V1/vault/accounts/{vaultAccountId}/{assetId}/addresses_paginated before Mar 31,2024. - All workspaces created after Mar 31,2024. will have it disabled. If it is disabled for your workspace and you attempt to use it, you will receive the following error message: \"This endpoint is unavailable. - Please use the GET /v1/vault/accounts/{vaultAccountId}/{assetId}/addresses_paginated endpoint to return all the wallet addresses associated with the specified vault account and asset in a paginated list.  </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor, Viewer.

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
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_vault_account_asset_addresses_paginated

> models::PaginatedAddressResponse get_vault_account_asset_addresses_paginated(vault_account_id, asset_id, limit, before, after)
Get addresses (Paginated)

Returns a paginated response of the addresses for a given vault account and asset. </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor, Viewer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vault_account_id** | **String** | The ID of the vault account to return | [required] |
**asset_id** | **String** | The ID of the asset | [required] |
**limit** | Option<**f64**> | Limit the number of results per page |  |
**before** | Option<**String**> | Cursor string for the previous page |  |
**after** | Option<**String**> | Cursor string for the next page |  |

### Return type

[**models::PaginatedAddressResponse**](PaginatedAddressResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_vault_assets

> Vec<models::VaultAsset> get_vault_assets(account_name_prefix, account_name_suffix)
Get asset balance for chosen assets

Gets the assets amount summary for all accounts or filtered accounts.  </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor, Viewer.

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
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_vault_balance_by_asset

> models::VaultAsset get_vault_balance_by_asset(asset_id)
Get vault balance by an asset

Get the total balance of an asset across all the vault accounts.  </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor, Viewer.

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
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## hide_vault_account

> models::VaultActionStatus hide_vault_account(vault_account_id, idempotency_key)
Hide a vault account in the console

Hides the requested vault account from the web console view. This operation is required when creating thousands of vault accounts to serve your end-users. Used for preventing the web console to be swamped with too much vault accounts. Learn more in the following [guide](https://developers.fireblocks.com/docs/create-direct-custody-wallets#hiding-vault-accounts). NOTE: Hiding the vault account from the web console will also hide all the related transactions to/from this vault. </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vault_account_id** | **String** | The vault account to hide | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::VaultActionStatus**](VaultActionStatus.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_customer_ref_id_for_address

> models::VaultActionStatus set_customer_ref_id_for_address(vault_account_id, asset_id, address_id, set_customer_ref_id_for_address_request, idempotency_key)
Assign AML customer reference ID

Sets an AML/KYT customer reference ID for a specific address. </br>Endpoint Permission: Admin, Non-Signing Admin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vault_account_id** | **String** | The ID of the vault account | [required] |
**asset_id** | **String** | The ID of the asset | [required] |
**address_id** | **String** | The address for which to add a description. For XRP, use <address>:<tag>, for all other assets, use only the address | [required] |
**set_customer_ref_id_for_address_request** | [**SetCustomerRefIdForAddressRequest**](SetCustomerRefIdForAddressRequest.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::VaultActionStatus**](VaultActionStatus.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_vault_account_auto_fuel

> models::VaultActionStatus set_vault_account_auto_fuel(vault_account_id, set_auto_fuel_request, idempotency_key)
Set auto fueling to on or off

Toggles the auto fueling property of the vault account to enabled or disabled. Vault Accounts with 'autoFuel=true' are monitored and auto fueled by the Fireblocks Gas Station. Learn more about the Fireblocks Gas Station in the following [guide](https://developers.fireblocks.com/docs/work-with-gas-station). </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vault_account_id** | **String** | The vault account ID | [required] |
**set_auto_fuel_request** | [**SetAutoFuelRequest**](SetAutoFuelRequest.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::VaultActionStatus**](VaultActionStatus.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_vault_account_customer_ref_id

> models::VaultActionStatus set_vault_account_customer_ref_id(vault_account_id, set_customer_ref_id_request, idempotency_key)
Set an AML/KYT ID for a vault account

Assigns an AML/KYT customer reference ID for the vault account. Learn more about Fireblocks AML management in the following [guide](https://developers.fireblocks.com/docs/define-aml-policies). </br>Endpoint Permission: Admin, Non-Signing Admin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vault_account_id** | **String** | The vault account ID | [required] |
**set_customer_ref_id_request** | [**SetCustomerRefIdRequest**](SetCustomerRefIdRequest.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::VaultActionStatus**](VaultActionStatus.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unhide_vault_account

> models::VaultActionStatus unhide_vault_account(vault_account_id, idempotency_key)
Unhide a vault account in the console

Makes a hidden vault account visible in web console view. </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vault_account_id** | **String** | The vault account to unhide | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::VaultActionStatus**](VaultActionStatus.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_vault_account

> models::RenameVaultAccountResponse update_vault_account(vault_account_id, update_vault_account_request, idempotency_key)
Rename a vault account

Renames the requested vault account. </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vault_account_id** | **String** | The ID of the vault account | [required] |
**update_vault_account_request** | [**UpdateVaultAccountRequest**](UpdateVaultAccountRequest.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::RenameVaultAccountResponse**](RenameVaultAccountResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_vault_account_asset_address

> models::VaultActionStatus update_vault_account_asset_address(vault_account_id, asset_id, address_id, idempotency_key, update_vault_account_asset_address_request)
Update address description

Updates the description of an existing address of an asset in a vault account. </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vault_account_id** | **String** | The ID of the vault account | [required] |
**asset_id** | **String** | The ID of the asset | [required] |
**address_id** | **String** | The address for which to add a description. For XRP, use <address>:<tag>, for all other assets, use only the address | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |
**update_vault_account_asset_address_request** | Option<[**UpdateVaultAccountAssetAddressRequest**](UpdateVaultAccountAssetAddressRequest.md)> |  |  |

### Return type

[**models::VaultActionStatus**](VaultActionStatus.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_vault_account_asset_balance

> models::VaultAsset update_vault_account_asset_balance(vault_account_id, asset_id, idempotency_key)
Refresh asset balance data

Updates the balance of a specific asset in a vault account.  This API endpoint is subject to a strict rate limit. Should be used by clients in very specific scenarios.  </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vault_account_id** | **String** | The ID of the vault account to return | [required] |
**asset_id** | **String** | The ID of the asset | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::VaultAsset**](VaultAsset.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

