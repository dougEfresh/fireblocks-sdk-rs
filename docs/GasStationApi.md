# \GasStationApi

All URIs are relative to *https://api.fireblocks.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_gas_station_by_asset_id**](GasStationApi.md#get_gas_station_by_asset_id) | **GET** /gas_station/{assetId} | Get gas station settings by asset
[**get_gas_station_info**](GasStationApi.md#get_gas_station_info) | **GET** /gas_station | Get gas station settings
[**update_gas_station_configuration**](GasStationApi.md#update_gas_station_configuration) | **PUT** /gas_station/configuration | Edit gas station settings
[**update_gas_station_configuration_by_asset_id**](GasStationApi.md#update_gas_station_configuration_by_asset_id) | **PUT** /gas_station/configuration/{assetId} | Edit gas station settings for an asset



## get_gas_station_by_asset_id

> models::GasStationPropertiesResponse get_gas_station_by_asset_id(asset_id)
Get gas station settings by asset

Returns gas station settings and balances for a requested asset. </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_id** | **String** | The ID of the asset | [required] |

### Return type

[**models::GasStationPropertiesResponse**](GasStationPropertiesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_gas_station_info

> models::GasStationPropertiesResponse get_gas_station_info()
Get gas station settings

Returns gas station settings and ETH balance. </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GasStationPropertiesResponse**](GasStationPropertiesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_gas_station_configuration

> models::EditGasStationConfigurationResponse update_gas_station_configuration(gas_station_configuration, idempotency_key)
Edit gas station settings

Configures gas station settings for ETH. Learn more about the Fireblocks Gas Station in the following [guide](https://developers.fireblocks.com/docs/work-with-gas-station). </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gas_station_configuration** | [**GasStationConfiguration**](GasStationConfiguration.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::EditGasStationConfigurationResponse**](EditGasStationConfigurationResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_gas_station_configuration_by_asset_id

> models::EditGasStationConfigurationResponse update_gas_station_configuration_by_asset_id(asset_id, gas_station_configuration, idempotency_key)
Edit gas station settings for an asset

Configures gas station settings for a requested asset. Learn more about the Fireblocks Gas Station in the following [guide](https://developers.fireblocks.com/docs/work-with-gas-station). </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_id** | **String** | The ID of the asset | [required] |
**gas_station_configuration** | [**GasStationConfiguration**](GasStationConfiguration.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::EditGasStationConfigurationResponse**](EditGasStationConfigurationResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

