# \PaymentsCrossBorderSettlementApi

All URIs are relative to *https://api.fireblocks.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**payments_xb_settlements_configs_config_id_delete**](PaymentsCrossBorderSettlementApi.md#payments_xb_settlements_configs_config_id_delete) | **DELETE** /payments/xb-settlements/configs/{configId} | Delete a cross-border settlement configuration
[**payments_xb_settlements_configs_config_id_get**](PaymentsCrossBorderSettlementApi.md#payments_xb_settlements_configs_config_id_get) | **GET** /payments/xb-settlements/configs/{configId} | Get a specific cross-border settlement configuration
[**payments_xb_settlements_configs_config_id_put**](PaymentsCrossBorderSettlementApi.md#payments_xb_settlements_configs_config_id_put) | **PUT** /payments/xb-settlements/configs/{configId} | Edit a cross-border settlement configuration
[**payments_xb_settlements_configs_get**](PaymentsCrossBorderSettlementApi.md#payments_xb_settlements_configs_get) | **GET** /payments/xb-settlements/configs | Get all the cross-border settlement configurations
[**payments_xb_settlements_configs_post**](PaymentsCrossBorderSettlementApi.md#payments_xb_settlements_configs_post) | **POST** /payments/xb-settlements/configs | Create a new cross-border settlement configuration
[**payments_xb_settlements_flows_flow_id_actions_execute_post**](PaymentsCrossBorderSettlementApi.md#payments_xb_settlements_flows_flow_id_actions_execute_post) | **POST** /payments/xb-settlements/flows/{flowId}/actions/execute | Execute cross-border settlement flow
[**payments_xb_settlements_flows_flow_id_get**](PaymentsCrossBorderSettlementApi.md#payments_xb_settlements_flows_flow_id_get) | **GET** /payments/xb-settlements/flows/{flowId} | Get specific cross-border settlement flow details
[**payments_xb_settlements_flows_post**](PaymentsCrossBorderSettlementApi.md#payments_xb_settlements_flows_post) | **POST** /payments/xb-settlements/flows | Create a new cross-border settlement flow



## payments_xb_settlements_configs_config_id_delete

> models::XbSettlementConfigModel payments_xb_settlements_configs_config_id_delete(config_id)
Delete a cross-border settlement configuration

Delete a cross-border settlement configuration. This does not delete or remove previously executed flows that used this configuration. **Note:** The reference content in this section documents the Payments Engine endpoint. The Payments Engine endpoint includes APIs available only for customers with the Payments Engine enabled on their accounts. These endpoints are currently in beta and might be subject to changes. If you want to learn more about the Fireblocks Payments Engine, please contact your Fireblocks Customer Success Manager or send an email to CSM@fireblocks.com. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**config_id** | **String** | The cross-border settlement configuration ID. | [required] |

### Return type

[**models::XbSettlementConfigModel**](XBSettlementConfigModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payments_xb_settlements_configs_config_id_get

> models::XbSettlementConfigModel payments_xb_settlements_configs_config_id_get(config_id)
Get a specific cross-border settlement configuration

Get a specific cross-border settlement configuration.</br> **Note:** The reference content in this section documents the Payments Engine endpoint. The Payments Engine endpoint includes APIs available only for customers with the Payments Engine enabled on their accounts. These endpoints are currently in beta and might be subject to changes. If you want to learn more about the Fireblocks Payments Engine, please contact your Fireblocks Customer Success Manager or send an email to CSM@fireblocks.com. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**config_id** | **String** | The cross-border settlement configuration ID. | [required] |

### Return type

[**models::XbSettlementConfigModel**](XBSettlementConfigModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payments_xb_settlements_configs_config_id_put

> models::XbSettlementConfigModel payments_xb_settlements_configs_config_id_put(config_id, xb_settlement_config_edit_request_body)
Edit a cross-border settlement configuration

Edit a cross-border settlement configuration. Editing a configuration does not affect previously executed flows that used the configuration. **Note:** The reference content in this section documents the Payments Engine endpoint. The Payments Engine endpoint includes APIs available only for customers with the Payments Engine enabled on their accounts. These endpoints are currently in beta and might be subject to changes. If you want to learn more about the Fireblocks Payments Engine, please contact your Fireblocks Customer Success Manager or send an email to CSM@fireblocks.com. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**config_id** | **String** | The cross-border settlement configuration ID. | [required] |
**xb_settlement_config_edit_request_body** | Option<[**XbSettlementConfigEditRequestBody**](XbSettlementConfigEditRequestBody.md)> |  |  |

### Return type

[**models::XbSettlementConfigModel**](XBSettlementConfigModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payments_xb_settlements_configs_get

> models::XbSettlementGetAllConfigsResponse payments_xb_settlements_configs_get()
Get all the cross-border settlement configurations

Get all the cross-border settlement configurations. </br> **Note:** The reference content in this section documents the Payments Engine endpoint. The Payments Engine endpoint includes APIs available only for customers with the Payments Engine enabled on their accounts. These endpoints are currently in beta and might be subject to changes. If you want to learn more about the Fireblocks Payments Engine, please contact your Fireblocks Customer Success Manager or send an email to CSM@fireblocks.com. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::XbSettlementGetAllConfigsResponse**](XBSettlementGetAllConfigsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payments_xb_settlements_configs_post

> models::XbSettlementConfigModel payments_xb_settlements_configs_post(xb_settlement_config_creation_request_body)
Create a new cross-border settlement configuration

<u><b>Create a new cross-border settlement configuration. </u></b></br>Configurations define the default assets, on-ramps, and off-ramps to use for the cross-border settlement. </br>  A configuration must contain at least two steps - `ON_RAMP` and `VAULT_ACCOUNT`. </br> All other steps (e.g., `OFF_RAMP`, `FIAT_DESTINATION`, etc.) are optional. </br> Every step must include the `accountId` to be used, while `inputAssetId` and `outputAssetId` are optional.  If those are not provided, a default value will be used from the Corridor Settings.</br> If the inputAssetId or the outputAssetId is provided for one of the objects, all assets in the objects must be consistent. For example, if the output asset of ON_RAMP is XLM_USDC_5F3T, then the input asset of the VAULT_ACCOUNT must also be XLM_USDC_5F3T..</br> You can set a slippage amount for your configuration. Slippage is defined by basis points (bps). This value can be overloaded on execution. If you do not configure a slippage amount, the default slippage of 10000 bps (10%) is used. </br> **Note:** The reference content in this section documents the Payments Engine endpoint. The Payments Engine endpoint includes APIs available only for customers with the Payments Engine enabled on their accounts. These endpoints are currently in beta and might be subject to changes. If you want to learn more about the Fireblocks Payments Engine, please contact your Fireblocks Customer Success Manager or send an email to CSM@fireblocks.com. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**xb_settlement_config_creation_request_body** | Option<[**XbSettlementConfigCreationRequestBody**](XbSettlementConfigCreationRequestBody.md)> |  |  |

### Return type

[**models::XbSettlementConfigModel**](XBSettlementConfigModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payments_xb_settlements_flows_flow_id_actions_execute_post

> models::XbSettlementFlowExecutionModel payments_xb_settlements_flows_flow_id_actions_execute_post(flow_id, xb_settlement_flow_execution_request_body)
Execute cross-border settlement flow

Send a payment flow with 'flowId' for execution. If a differet slippage configuraion is needed for this execution than configured in the flow configuration, the request body must define the desired slippage configuration for this execution.  **Note:** The reference content in this section documents the Payments Engine endpoint. The Payments Engine endpoint includes APIs available only for customers with the Payments Engine enabled on their accounts. These endpoints are currently in beta and might be subject to changes. If you want to learn more about the Fireblocks Payments Engine, please contact your Fireblocks Customer Success Manager or send an email to CSM@fireblocks.com. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**flow_id** | **String** | The cross-border settlement flow ID. | [required] |
**xb_settlement_flow_execution_request_body** | Option<[**XbSettlementFlowExecutionRequestBody**](XbSettlementFlowExecutionRequestBody.md)> |  |  |

### Return type

[**models::XbSettlementFlowExecutionModel**](XBSettlementFlowExecutionModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payments_xb_settlements_flows_flow_id_get

> models::XbSettlementGetFlowResponse payments_xb_settlements_flows_flow_id_get(flow_id)
Get specific cross-border settlement flow details

Gets details for a specific cross-border settlement flow **Note:** The reference content in this section documents the Payments Engine endpoint. The Payments Engine endpoint includes APIs available only for customers with the Payments Engine enabled on their accounts. These endpoints are currently in beta and might be subject to changes. If you want to learn more about the Fireblocks Payments Engine, please contact your Fireblocks Customer Success Manager or send an email to CSM@fireblocks.com. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**flow_id** | **String** | The cross-border settlement flow ID. | [required] |

### Return type

[**models::XbSettlementGetFlowResponse**](XBSettlementGetFlowResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payments_xb_settlements_flows_post

> models::XbSettlementFlowPreviewModel payments_xb_settlements_flows_post(xb_settlement_create_flow_request_body)
Create a new cross-border settlement flow

Create a cross-border flow (based on a cross-border configuration) with an amount to transfer.  The assetId is defined by the cross-border configuration. Creating a flow triggers a calculation of the flow estimations, including FX rates, times, and fees based on the amount provided. Creating a cross-border flow will not execute the flow.  **Note:** The reference content in this section documents the Payments Engine endpoint. The Payments Engine endpoint includes APIs available only for customers with the Payments Engine enabled on their accounts. These endpoints are currently in beta and might be subject to changes. If you want to learn more about the Fireblocks Payments Engine, please contact your Fireblocks Customer Success Manager or send an email to CSM@fireblocks.com. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**xb_settlement_create_flow_request_body** | Option<[**XbSettlementCreateFlowRequestBody**](XbSettlementCreateFlowRequestBody.md)> |  |  |

### Return type

[**models::XbSettlementFlowPreviewModel**](XBSettlementFlowPreviewModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

