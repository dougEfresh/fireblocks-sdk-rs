# \TransactionsApi

All URIs are relative to *https://api.fireblocks.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**transactions_estimate_fee_post**](TransactionsApi.md#transactions_estimate_fee_post) | **POST** /transactions/estimate_fee | Estimate transaction fee
[**transactions_external_tx_id_external_tx_id_get**](TransactionsApi.md#transactions_external_tx_id_external_tx_id_get) | **GET** /transactions/external_tx_id/{externalTxId}/ | Find a specific transaction by external transaction ID
[**transactions_get**](TransactionsApi.md#transactions_get) | **GET** /transactions | List transaction history
[**transactions_post**](TransactionsApi.md#transactions_post) | **POST** /transactions | Create a new transaction
[**transactions_tx_id_cancel_post**](TransactionsApi.md#transactions_tx_id_cancel_post) | **POST** /transactions/{txId}/cancel | Cancel a transaction
[**transactions_tx_id_drop_post**](TransactionsApi.md#transactions_tx_id_drop_post) | **POST** /transactions/{txId}/drop | Drop ETH transaction by ID
[**transactions_tx_id_freeze_post**](TransactionsApi.md#transactions_tx_id_freeze_post) | **POST** /transactions/{txId}/freeze | Freeze a transaction
[**transactions_tx_id_get**](TransactionsApi.md#transactions_tx_id_get) | **GET** /transactions/{txId} | Find a specific transaction by Fireblocks transaction ID
[**transactions_tx_id_set_confirmation_threshold_post**](TransactionsApi.md#transactions_tx_id_set_confirmation_threshold_post) | **POST** /transactions/{txId}/set_confirmation_threshold | Set confirmation threshold by transaction ID
[**transactions_tx_id_unfreeze_post**](TransactionsApi.md#transactions_tx_id_unfreeze_post) | **POST** /transactions/{txId}/unfreeze | Unfreeze a transaction
[**tx_hash_tx_hash_set_confirmation_threshold_post**](TransactionsApi.md#tx_hash_tx_hash_set_confirmation_threshold_post) | **POST** /txHash/{txHash}/set_confirmation_threshold | Set confirmation threshold by transaction hash



## transactions_estimate_fee_post

> models::EstimatedTransactionFeeResponse transactions_estimate_fee_post(estimated_fee_request)
Estimate transaction fee

Estimates the transaction fee for a transaction request.  ***Note:***  -Supports all Fireblocks assets except ZCash (ZEC).  -For XEM and Chiliz assets, 0 will be returned for all three fee levels, as there is no gas or tip price on these blockchains. You can use the POST /v1/transactions/estimate_fee to see what the exact fee will be for a transaction on this network. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**estimated_fee_request** | Option<[**EstimatedFeeRequest**](EstimatedFeeRequest.md)> |  |  |

### Return type

[**models::EstimatedTransactionFeeResponse**](EstimatedTransactionFeeResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transactions_external_tx_id_external_tx_id_get

> models::TransactionResponse transactions_external_tx_id_external_tx_id_get(external_tx_id)
Find a specific transaction by external transaction ID

Returns transaction by external transaction ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_tx_id** | **String** | The external ID of the transaction to return | [required] |

### Return type

[**models::TransactionResponse**](TransactionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transactions_get

> Vec<models::TransactionResponse> transactions_get(before, after, status, order_by, sort, limit, source_type, source_id, dest_type, dest_id, assets, tx_hash, source_wallet_id, dest_wallet_id)
List transaction history

Lists the transaction history for your workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**before** | Option<**String**> | Unix timestamp in milliseconds. Returns only transactions created before the specified date |  |
**after** | Option<**String**> | Unix timestamp in milliseconds. Returns only transactions created after the specified date |  |
**status** | Option<**String**> | You can filter by one of the statuses. |  |
**order_by** | Option<**String**> | The field to order the results by  **Note**: Ordering by a field that is not createdAt may result with transactions that receive updates as you request the next or previous pages of results, resulting with missing those transactions. |  |
**sort** | Option<**String**> | The direction to order the results by |  |
**limit** | Option<**i32**> | Limits the number of results. If not provided, a limit of 200 will be used. The maximum allowed limit is 500 |  |[default to 200]
**source_type** | Option<**String**> | The source type of the transaction |  |
**source_id** | Option<**String**> | The source ID of the transaction |  |
**dest_type** | Option<**String**> | The destination type of the transaction |  |
**dest_id** | Option<**String**> | The destination ID of the transaction |  |
**assets** | Option<**String**> | A list of assets to filter by, seperated by commas |  |
**tx_hash** | Option<**String**> | Returns only results with a specified txHash |  |
**source_wallet_id** | Option<**String**> | Returns only results where the source is a specific end user wallet |  |
**dest_wallet_id** | Option<**String**> | Returns only results where the destination is a specific end user wallet |  |

### Return type

[**Vec<models::TransactionResponse>**](TransactionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transactions_post

> models::CreateTransactionResponse transactions_post(x_end_user_wallet_id, transaction_request)
Create a new transaction

Creates a new transaction. This endpoint can be used for regular Transfers, Contract Calls, Raw & Typed message signing. - For Transfers, the required parameters are: `assetId`, `source`, `destination` and `amount`.   This endpoint is also used for native NFT transfers. For more details, please see the [Transferring NFT](https://developers.fireblocks.com/docs/transferring-nfts) guide.  - For Contract Calls, the required parameters are: `operation.CONTRACT_CALL`, `assetId` (Base Asset), `source`, `destination`, `amount` (usually 0) and `extraParameters` object with `contractCallData` string.  - For RAW and Typed messages signing, the required parameters are: `operation.RAW/TYPED_MESSAGE`, `assetId` or `derivationPath`, `source` or `derivationPath`, `extraParameters` with [rawMessageData object](https://developers.fireblocks.com/reference/raw-signing-objects).  - Typed Message Signing is supported for the following asset IDs: 'ETH', 'BTC' and 'TRX'. [Typed Message Signing Guide](https://developers.fireblocks.com/docs/typed-message-signing-overview). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_end_user_wallet_id** | Option<**uuid::Uuid**> | Unique ID of the End-User wallet to the API request. Required for end-user wallet operations. |  |
**transaction_request** | Option<[**TransactionRequest**](TransactionRequest.md)> |  |  |

### Return type

[**models::CreateTransactionResponse**](CreateTransactionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transactions_tx_id_cancel_post

> models::CancelTransactionResponse transactions_tx_id_cancel_post(tx_id, x_end_user_wallet_id)
Cancel a transaction

Cancels a transaction by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tx_id** | **String** | The ID of the transaction to cancel | [required] |
**x_end_user_wallet_id** | Option<**uuid::Uuid**> | Unique ID of the End-User wallet to the API request. Required for end-user wallet operations. |  |

### Return type

[**models::CancelTransactionResponse**](CancelTransactionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transactions_tx_id_drop_post

> models::DropTransactionResponse transactions_tx_id_drop_post(tx_id, x_end_user_wallet_id, drop_transaction_request)
Drop ETH transaction by ID

Drops a stuck ETH transaction and creates a replacement transaction.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tx_id** | **String** | The ID of the transaction | [required] |
**x_end_user_wallet_id** | Option<**uuid::Uuid**> | Unique ID of the End-User wallet to the API request. Required for end-user wallet operations. |  |
**drop_transaction_request** | Option<[**DropTransactionRequest**](DropTransactionRequest.md)> |  |  |

### Return type

[**models::DropTransactionResponse**](DropTransactionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transactions_tx_id_freeze_post

> models::FreezeTransactionResponse transactions_tx_id_freeze_post(tx_id, x_end_user_wallet_id)
Freeze a transaction

Freezes a transaction by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tx_id** | **String** | The ID of the transaction to freeze | [required] |
**x_end_user_wallet_id** | Option<**uuid::Uuid**> | Unique ID of the End-User wallet to the API request. Required for end-user wallet operations. |  |

### Return type

[**models::FreezeTransactionResponse**](FreezeTransactionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transactions_tx_id_get

> models::TransactionResponse transactions_tx_id_get(tx_id)
Find a specific transaction by Fireblocks transaction ID

Returns a transaction by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tx_id** | **String** | The ID of the transaction to return | [required] |

### Return type

[**models::TransactionResponse**](TransactionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transactions_tx_id_set_confirmation_threshold_post

> models::SetConfirmationsThresholdResponse transactions_tx_id_set_confirmation_threshold_post(tx_id, set_confirmations_threshold_request)
Set confirmation threshold by transaction ID

Overrides the required number of confirmations for transaction completion by transaction ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tx_id** | **String** | The ID of the transaction | [required] |
**set_confirmations_threshold_request** | Option<[**SetConfirmationsThresholdRequest**](SetConfirmationsThresholdRequest.md)> |  |  |

### Return type

[**models::SetConfirmationsThresholdResponse**](SetConfirmationsThresholdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transactions_tx_id_unfreeze_post

> models::UnfreezeTransactionResponse transactions_tx_id_unfreeze_post(tx_id, x_end_user_wallet_id)
Unfreeze a transaction

Unfreezes a transaction by ID and makes the transaction available again.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tx_id** | **String** | The ID of the transaction to unfreeze | [required] |
**x_end_user_wallet_id** | Option<**uuid::Uuid**> | Unique ID of the End-User wallet to the API request. Required for end-user wallet operations. |  |

### Return type

[**models::UnfreezeTransactionResponse**](UnfreezeTransactionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tx_hash_tx_hash_set_confirmation_threshold_post

> models::SetConfirmationsThresholdResponse tx_hash_tx_hash_set_confirmation_threshold_post(tx_hash, set_confirmations_threshold_request)
Set confirmation threshold by transaction hash

Overrides the required number of confirmations for transaction completion by transaction hash.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tx_hash** | **String** | The TxHash | [required] |
**set_confirmations_threshold_request** | Option<[**SetConfirmationsThresholdRequest**](SetConfirmationsThresholdRequest.md)> |  |  |

### Return type

[**models::SetConfirmationsThresholdResponse**](SetConfirmationsThresholdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

