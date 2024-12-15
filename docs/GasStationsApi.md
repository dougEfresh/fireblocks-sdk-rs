# \GasStationsApi

All URIs are relative to *https://api.fireblocks.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**gas_station_asset_id_get**](GasStationsApi.md#gas_station_asset_id_get) | **GET** /gas_station/{assetId} | Get gas station settings by asset
[**gas_station_configuration_asset_id_put**](GasStationsApi.md#gas_station_configuration_asset_id_put) | **PUT** /gas_station/configuration/{assetId} | Edit gas station settings for an asset
[**gas_station_configuration_put**](GasStationsApi.md#gas_station_configuration_put) | **PUT** /gas_station/configuration | Edit gas station settings
[**gas_station_get**](GasStationsApi.md#gas_station_get) | **GET** /gas_station | Get gas station settings



## gas_station_asset_id_get

> models::GasStationPropertiesResponse gas_station_asset_id_get(asset_id)
Get gas station settings by asset

Returns gas station settings and balances for a requested asset.

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
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## gas_station_configuration_asset_id_put

> gas_station_configuration_asset_id_put(asset_id, gas_station_configuration)
Edit gas station settings for an asset

Configures gas station settings for a requested asset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_id** | **String** | The ID of the asset | [required] |
**gas_station_configuration** | [**GasStationConfiguration**](GasStationConfiguration.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## gas_station_configuration_put

> gas_station_configuration_put(gas_station_configuration)
Edit gas station settings

Configures gas station settings for ETH.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gas_station_configuration** | [**GasStationConfiguration**](GasStationConfiguration.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## gas_station_get

> models::GasStationPropertiesResponse gas_station_get()
Get gas station settings

Returns gas station settings and ETH balance.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GasStationPropertiesResponse**](GasStationPropertiesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

