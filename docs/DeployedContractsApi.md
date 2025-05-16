# \DeployedContractsApi

All URIs are relative to *https://api.fireblocks.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_contract_abi**](DeployedContractsApi.md#add_contract_abi) | **POST** /tokenization/contracts/abi | Save contract ABI
[**fetch_contract_abi**](DeployedContractsApi.md#fetch_contract_abi) | **POST** /tokenization/contracts/fetch_abi | Fetch the contract ABI
[**get_deployed_contract_by_address**](DeployedContractsApi.md#get_deployed_contract_by_address) | **GET** /tokenization/contracts/{assetId}/{contractAddress} | Return deployed contract data
[**get_deployed_contract_by_id**](DeployedContractsApi.md#get_deployed_contract_by_id) | **GET** /tokenization/contracts/{id} | Return deployed contract data by id
[**get_deployed_contracts**](DeployedContractsApi.md#get_deployed_contracts) | **GET** /tokenization/contracts | List deployed contracts data



## add_contract_abi

> models::ContractWithAbiDto add_contract_abi(add_abi_request_dto, idempotency_key)
Save contract ABI

Save contract ABI for the tenant. </br>Endpoint Permission: Owner, Admin, Non-Signing Admin, Signer, and Editor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_abi_request_dto** | [**AddAbiRequestDto**](AddAbiRequestDto.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::ContractWithAbiDto**](ContractWithAbiDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_contract_abi

> models::ContractWithAbiDto fetch_contract_abi(fetch_abi_request_dto, idempotency_key)
Fetch the contract ABI

Fetch the ABI. If not found fetch the ABI from the block explorer. </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor, Viewer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fetch_abi_request_dto** | [**FetchAbiRequestDto**](FetchAbiRequestDto.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::ContractWithAbiDto**](ContractWithAbiDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_deployed_contract_by_address

> models::DeployedContractResponseDto get_deployed_contract_by_address(contract_address, asset_id)
Return deployed contract data

Return deployed contract data by blockchain native asset id and contract address. </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor, Viewer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_address** | **String** | The contract's onchain address | [required] |
**asset_id** | **String** | The blockchain base asset ID | [required] |

### Return type

[**models::DeployedContractResponseDto**](DeployedContractResponseDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_deployed_contract_by_id

> models::DeployedContractResponseDto get_deployed_contract_by_id(id)
Return deployed contract data by id

Return deployed contract data by id. </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor, Viewer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The deployed contract data identifier | [required] |

### Return type

[**models::DeployedContractResponseDto**](DeployedContractResponseDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_deployed_contracts

> models::DeployedContractsPaginatedResponse get_deployed_contracts(page_cursor, page_size, contract_address, base_asset_id, contract_template_id)
List deployed contracts data

Return a filtered lean representation of the deployed contracts data on all blockchains (paginated). </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor, Viewer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_cursor** | Option<**String**> | Page cursor to get the next page |  |
**page_size** | Option<**f64**> | Number of items per page, requesting more then max will return max items |  |
**contract_address** | Option<**String**> | The contract's onchain address |  |
**base_asset_id** | Option<**String**> | The blockchain asset ID |  |
**contract_template_id** | Option<**String**> | The contract template identifier |  |

### Return type

[**models::DeployedContractsPaginatedResponse**](DeployedContractsPaginatedResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

