# \TransactionsApi

All URIs are relative to *https://api.fireblocks.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel_transaction**](TransactionsApi.md#cancel_transaction) | **POST** /transactions/{txId}/cancel | Cancel a transaction
[**create_transaction**](TransactionsApi.md#create_transaction) | **POST** /transactions | Create a new transaction
[**drop_transaction**](TransactionsApi.md#drop_transaction) | **POST** /transactions/{txId}/drop | Drop ETH (EVM) transaction by ID
[**estimate_transaction_fee**](TransactionsApi.md#estimate_transaction_fee) | **POST** /transactions/estimate_fee | Estimate transaction fee
[**freeze_transaction**](TransactionsApi.md#freeze_transaction) | **POST** /transactions/{txId}/freeze | Freeze a transaction
[**get_transaction**](TransactionsApi.md#get_transaction) | **GET** /transactions/{txId} | Get a specific transaction by Fireblocks transaction ID
[**get_transaction_by_external_id**](TransactionsApi.md#get_transaction_by_external_id) | **GET** /transactions/external_tx_id/{externalTxId} | Get a specific transaction by external transaction ID
[**get_transactions**](TransactionsApi.md#get_transactions) | **GET** /transactions | Get transaction history
[**rescan_transactions_beta**](TransactionsApi.md#rescan_transactions_beta) | **POST** /transactions/rescan | Rescan an array of transactions
[**set_confirmation_threshold_by_transaction_hash**](TransactionsApi.md#set_confirmation_threshold_by_transaction_hash) | **POST** /txHash/{txHash}/set_confirmation_threshold | Set confirmation threshold by transaction hash
[**set_transaction_confirmation_threshold**](TransactionsApi.md#set_transaction_confirmation_threshold) | **POST** /transactions/{txId}/set_confirmation_threshold | Set confirmation threshold by Fireblocks Transaction ID
[**unfreeze_transaction**](TransactionsApi.md#unfreeze_transaction) | **POST** /transactions/{txId}/unfreeze | Unfreeze a transaction



## cancel_transaction

> models::CancelTransactionResponse cancel_transaction(tx_id, x_end_user_wallet_id, idempotency_key)
Cancel a transaction

Cancels a transaction by Fireblocks Transaction ID.  Can be used only for transactions that did not get to the BROADCASTING state. </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tx_id** | **String** | The Fireblocks Transaction ID of the transaction to cancel | [required] |
**x_end_user_wallet_id** | Option<**uuid::Uuid**> | Unique ID of the End-User wallet to the API request. Required for end-user wallet operations. |  |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::CancelTransactionResponse**](CancelTransactionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_transaction

> models::CreateTransactionResponse create_transaction(transaction_request, x_end_user_wallet_id, idempotency_key)
Create a new transaction

Creates a new transaction. This endpoint can be used for regular Transfers, Contract Calls, Raw & Typed message signing. - For Transfers, the required parameters are: `assetId`, `source`, `destination` and `amount`.  - For Contract Calls, the required parameters are: `operation.CONTRACT_CALL`, `assetId` (Base Asset), `source`, `destination`, `amount` (usually 0) and `extraParameters` object with `contractCallData` string.  - For RAW and Typed messages signing, the required parameters are: `operation.RAW/TYPED_MESSAGE`, `assetId` or `derivationPath`, `source` or `derivationPath`, `extraParameters` with [rawMessageData object](https://developers.fireblocks.com/reference/raw-signing-objects).  - Typed Message Signing is supported for the following asset IDs: 'ETH', 'BTC' and 'TRX'. [Typed Message Signing Guide](https://developers.fireblocks.com/docs/typed-message-signing-overview).  - For MEV Protection configuration the required parameters are:   `extraParameters` with the [`nodeControls` object](https://developers.fireblocks.com/reference/transaction-objects#nodecontrols)   Note: MEV Protection is a premium feature. Please contact your Customer Success Manager or the Fireblocks Support team for more information.  - To create ZEC transaction, please call [Get unspent UTXO Input endpoint](https://developers.fireblocks.com/reference/getunspentinputs) to get the amount and use it as an input under `networkfee` on this endpoint. Please use this formula `(0.0001 + 0.00005*N) where N is the number of inputs` to calculate the fee needed and use it as an input under networkFee field Learn more about Fireblocks Transactions management in the following [guide](https://developers.fireblocks.com/reference/create-transactions). </br>Endpoint Permission: Admin, Signer, Editor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transaction_request** | [**TransactionRequest**](TransactionRequest.md) |  | [required] |
**x_end_user_wallet_id** | Option<**uuid::Uuid**> | Unique ID of the End-User wallet to the API request. Required for end-user wallet operations. |  |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::CreateTransactionResponse**](CreateTransactionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## drop_transaction

> models::DropTransactionResponse drop_transaction(tx_id, x_end_user_wallet_id, idempotency_key, drop_transaction_request)
Drop ETH (EVM) transaction by ID

Drops a stuck ETH (EVM) transaction and creates a replacement transaction with 0 amount. </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tx_id** | **String** | The ID of the transaction | [required] |
**x_end_user_wallet_id** | Option<**uuid::Uuid**> | Unique ID of the End-User wallet to the API request. Required for end-user wallet operations. |  |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |
**drop_transaction_request** | Option<[**DropTransactionRequest**](DropTransactionRequest.md)> |  |  |

### Return type

[**models::DropTransactionResponse**](DropTransactionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## estimate_transaction_fee

> models::EstimatedTransactionFeeResponse estimate_transaction_fee(idempotency_key, transaction_request)
Estimate transaction fee

Estimates the transaction fee for a specific transaction request. This endpoint simulates a transaction which means that the system will expect to have the requested asset and balance in the specified wallet.   **Note**: Supports all Fireblocks assets except ZCash (ZEC). Learn more about Fireblocks Fee Management in the following [guide](https://developers.fireblocks.com/reference/estimate-transaction-fee). </br>Endpoint Permission: Admin, Signer, Approver, Editor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |
**transaction_request** | Option<[**TransactionRequest**](TransactionRequest.md)> |  |  |

### Return type

[**models::EstimatedTransactionFeeResponse**](EstimatedTransactionFeeResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## freeze_transaction

> models::FreezeTransactionResponse freeze_transaction(tx_id, x_end_user_wallet_id, idempotency_key)
Freeze a transaction

Freezes a transaction by ID.  Usually used for AML integrations when the incoming funds should be quarantined. For account based assets - the entire amount of the transaction is frozen  For UTXO based assets - all UTXOs of the specified transaction are frozen </br>Endpoint Permission: Admin, Non-Signing Admin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tx_id** | **String** | The ID of the transaction to freeze | [required] |
**x_end_user_wallet_id** | Option<**uuid::Uuid**> | Unique ID of the End-User wallet to the API request. Required for end-user wallet operations. |  |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::FreezeTransactionResponse**](FreezeTransactionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_transaction

> models::TransactionResponse get_transaction(tx_id)
Get a specific transaction by Fireblocks transaction ID

Get a specific transaction data by Fireblocks Transaction ID </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor, Viewer.

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
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_transaction_by_external_id

> models::TransactionResponse get_transaction_by_external_id(external_tx_id)
Get a specific transaction by external transaction ID

Returns transaction by external transaction ID. </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor, Viewer.

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
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_transactions

> Vec<models::TransactionResponse> get_transactions(before, after, status, order_by, sort, limit, source_type, source_id, dest_type, dest_id, assets, tx_hash, source_wallet_id, dest_wallet_id)
Get transaction history

Get the transaction history for your workspace. </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor, Viewer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**before** | Option<**String**> | Unix timestamp in milliseconds. Returns only transactions created before the specified date. Provides an explicit end time. If not provided, default value will be applied, and may change over time.  The current default value is the past 90 days.  |  |
**after** | Option<**String**> | Unix timestamp in milliseconds. Returns only transactions created after the specified date. Provides an explicit start time. If not provided, default value will be applied, and may change over time.  The current default value is the past 90 days.  |  |
**status** | Option<[**TransactionStatus**](.md)> | You can filter by one of the statuses. |  |
**order_by** | Option<**String**> | The field to order the results by  **Note**: Ordering by a field that is not createdAt may result with transactions that receive updates as you request the next or previous pages of results, resulting with missing those transactions. |  |
**sort** | Option<**String**> | The direction to order the results by |  |
**limit** | Option<**u32**> | Limits the number of results. If not provided, a limit of 200 will be used. The maximum allowed limit is 500 |  |[default to 200]
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
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rescan_transactions_beta

> Vec<models::ValidatedTransactionsForRescan> rescan_transactions_beta(rescan_transaction, idempotency_key)
Rescan an array of transactions

Rescan transaction by running an async job. </br>  **Note**: - These endpoints are currently in beta and might be subject to changes. - We limit the amount of the transaction to 16 per request.  </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rescan_transaction** | [**Vec<models::RescanTransaction>**](RescanTransaction.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**Vec<models::ValidatedTransactionsForRescan>**](ValidatedTransactionsForRescan.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_confirmation_threshold_by_transaction_hash

> models::SetConfirmationsThresholdResponse set_confirmation_threshold_by_transaction_hash(tx_hash, idempotency_key, set_confirmations_threshold_request)
Set confirmation threshold by transaction hash

Overrides the required number of confirmations for transaction completion by transaction hash. </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tx_hash** | **String** | The TxHash | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |
**set_confirmations_threshold_request** | Option<[**SetConfirmationsThresholdRequest**](SetConfirmationsThresholdRequest.md)> |  |  |

### Return type

[**models::SetConfirmationsThresholdResponse**](SetConfirmationsThresholdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_transaction_confirmation_threshold

> models::SetConfirmationsThresholdResponse set_transaction_confirmation_threshold(tx_id, idempotency_key, set_confirmations_threshold_request)
Set confirmation threshold by Fireblocks Transaction ID

Overrides the required number of confirmations for transaction completion Fireblocks Transaction ID. </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tx_id** | **String** | The ID of the transaction | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |
**set_confirmations_threshold_request** | Option<[**SetConfirmationsThresholdRequest**](SetConfirmationsThresholdRequest.md)> |  |  |

### Return type

[**models::SetConfirmationsThresholdResponse**](SetConfirmationsThresholdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unfreeze_transaction

> models::UnfreezeTransactionResponse unfreeze_transaction(tx_id, x_end_user_wallet_id, idempotency_key)
Unfreeze a transaction

Unfreezes a transaction by Fireblocks Transaction ID and makes the transaction available again. </br>Endpoint Permission: Admin, Non-Signing Admin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tx_id** | **String** | The ID of the transaction to unfreeze | [required] |
**x_end_user_wallet_id** | Option<**uuid::Uuid**> | Unique ID of the End-User wallet to the API request. Required for end-user wallet operations. |  |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::UnfreezeTransactionResponse**](UnfreezeTransactionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

