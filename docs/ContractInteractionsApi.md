# \ContractInteractionsApi

All URIs are relative to *https://api.fireblocks.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_deployed_contract_abi**](ContractInteractionsApi.md#get_deployed_contract_abi) | **GET** /contract_interactions/base_asset_id/{baseAssetId}/contract_address/{contractAddress}/functions | Return deployed contract's ABI
[**get_transaction_receipt**](ContractInteractionsApi.md#get_transaction_receipt) | **GET** /contract_interactions/base_asset_id/{baseAssetId}/tx_hash/{txHash}/receipt | Get transaction receipt
[**read_call_function**](ContractInteractionsApi.md#read_call_function) | **POST** /contract_interactions/base_asset_id/{baseAssetId}/contract_address/{contractAddress}/functions/read | Call a read function
[**write_call_function**](ContractInteractionsApi.md#write_call_function) | **POST** /contract_interactions/base_asset_id/{baseAssetId}/contract_address/{contractAddress}/functions/write | Call a write function



## get_deployed_contract_abi

> models::ContractAbiResponseDto get_deployed_contract_abi(contract_address, base_asset_id, idempotency_key)
Return deployed contract's ABI

Return deployed contract's ABI by blockchain native asset id and contract address. </br>Endpoint Permission: Admin, Non-Signing Admin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_address** | **String** | The contract's onchain address | [required] |
**base_asset_id** | **String** | The blockchain base assetId | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::ContractAbiResponseDto**](ContractAbiResponseDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_transaction_receipt

> models::TransactionReceiptResponse get_transaction_receipt(base_asset_id, tx_hash)
Get transaction receipt

Retrieve the transaction receipt by blockchain native asset ID and transaction hash  </br>Endpoint Permission: Admin, Non-Signing Admin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**base_asset_id** | **String** | The blockchain base assetId | [required] |
**tx_hash** | **String** | The transaction hash | [required] |

### Return type

[**models::TransactionReceiptResponse**](TransactionReceiptResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_call_function

> Vec<models::ParameterWithValue> read_call_function(contract_address, base_asset_id, read_call_function_dto, idempotency_key)
Call a read function

Call a read function on a deployed contract by blockchain native asset id and contract address. </br>Endpoint Permission: Admin, Non-Signing Admin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_address** | **String** | The contract's onchain address | [required] |
**base_asset_id** | **String** | The blockchain base assetId | [required] |
**read_call_function_dto** | [**ReadCallFunctionDto**](ReadCallFunctionDto.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**Vec<models::ParameterWithValue>**](ParameterWithValue.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## write_call_function

> models::WriteCallFunctionResponseDto write_call_function(contract_address, base_asset_id, write_call_function_dto, idempotency_key)
Call a write function

Call a write function on a deployed contract by blockchain native asset id and contract address. This creates an onchain transaction, thus it is an async operation. It returns a transaction id that can be polled for status check.  </br>Endpoint Permission: Admin, Non-Signing Admin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_address** | **String** | The contract's onchain address | [required] |
**base_asset_id** | **String** | The blockchain base assetId | [required] |
**write_call_function_dto** | [**WriteCallFunctionDto**](WriteCallFunctionDto.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::WriteCallFunctionResponseDto**](WriteCallFunctionResponseDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

