# \ContractTemplatesApi

All URIs are relative to *https://api.fireblocks.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_contract_template_by_id**](ContractTemplatesApi.md#delete_contract_template_by_id) | **DELETE** /tokenization/templates/{contractTemplateId} | Delete a contract template by id
[**deploy_contract**](ContractTemplatesApi.md#deploy_contract) | **POST** /tokenization/templates/{contractTemplateId}/deploy | Deploy contract
[**get_constructor_by_contract_template_id**](ContractTemplatesApi.md#get_constructor_by_contract_template_id) | **GET** /tokenization/templates/{contractTemplateId}/constructor | Return contract template's constructor
[**get_contract_template_by_id**](ContractTemplatesApi.md#get_contract_template_by_id) | **GET** /tokenization/templates/{contractTemplateId} | Return contract template by id
[**get_contract_templates**](ContractTemplatesApi.md#get_contract_templates) | **GET** /tokenization/templates | List all contract templates
[**get_function_abi_by_contract_template_id**](ContractTemplatesApi.md#get_function_abi_by_contract_template_id) | **GET** /tokenization/templates/{contractTemplateId}/function | Return contract template's function
[**upload_contract_template**](ContractTemplatesApi.md#upload_contract_template) | **POST** /tokenization/templates | Upload contract template



## delete_contract_template_by_id

> delete_contract_template_by_id(contract_template_id)
Delete a contract template by id

Delete a contract by id. allowed only for private contract templates. Notice: it is irreversible!

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_template_id** | **String** | The Contract Template identifier | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deploy_contract

> models::ContractDeployResponse deploy_contract(contract_template_id, contract_deploy_request, idempotency_key)
Deploy contract

Deploy a new contract by contract template id. If you wish to deploy a token (ERC20, ERC721 etc), and create asset please use POST /tokenization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_template_id** | **String** | The Contract Template identifier | [required] |
**contract_deploy_request** | [**ContractDeployRequest**](ContractDeployRequest.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::ContractDeployResponse**](ContractDeployResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_constructor_by_contract_template_id

> models::AbiFunction get_constructor_by_contract_template_id(contract_template_id, with_docs)
Return contract template's constructor

Return contract template's constructor ABI

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_template_id** | **String** | The Contract Template identifier | [required] |
**with_docs** | Option<**bool**> | true if you want to get the abi with its docs |  |[default to false]

### Return type

[**models::AbiFunction**](AbiFunction.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_contract_template_by_id

> models::ContractTemplateDto get_contract_template_by_id(contract_template_id)
Return contract template by id

Return detailed information about the contract template

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_template_id** | **String** | The Contract Template identifier | [required] |

### Return type

[**models::ContractTemplateDto**](ContractTemplateDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_contract_templates

> models::TemplatesPaginatedResponse get_contract_templates(page_cursor, page_size, r#type, initialization_phase)
List all contract templates

Return minimal representation of all the contract templates available for the workspace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_cursor** | Option<**String**> | Page cursor to get the next page |  |
**page_size** | Option<**f64**> | Number of items per page, requesting more then max will return max items |  |
**r#type** | Option<**String**> | The type of the contract templates you wish to retrieve. Can accept one type, more or none |  |
**initialization_phase** | Option<**String**> | For standalone contracts use ON_DEPLOYMENT and for contracts that are behind proxies use POST_DEPLOYMENT |  |

### Return type

[**models::TemplatesPaginatedResponse**](TemplatesPaginatedResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_function_abi_by_contract_template_id

> models::AbiFunction get_function_abi_by_contract_template_id(contract_template_id, function_signature)
Return contract template's function

Return contract template`s function ABI by signature

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_template_id** | **String** | The Contract Template identifier | [required] |
**function_signature** | **String** | The contract's function signature | [required] |

### Return type

[**models::AbiFunction**](AbiFunction.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_contract_template

> models::ContractTemplateDto upload_contract_template(contract_upload_request, idempotency_key)
Upload contract template

Upload a new contract template. This contract template will be available for the workspace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_upload_request** | [**ContractUploadRequest**](ContractUploadRequest.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::ContractTemplateDto**](ContractTemplateDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

