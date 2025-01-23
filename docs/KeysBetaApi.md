# \KeysBetaApi

All URIs are relative to *https://api.fireblocks.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_mpc_keys_list**](KeysBetaApi.md#get_mpc_keys_list) | **GET** /keys/mpc/list | Get list of mpc keys
[**get_mpc_keys_list_by_user**](KeysBetaApi.md#get_mpc_keys_list_by_user) | **GET** /keys/mpc/list/{userId} | Get list of mpc keys by `userId`



## get_mpc_keys_list

> models::GetMpcKeysResponse get_mpc_keys_list()
Get list of mpc keys

Returns a list of MPC signing keys of the workspace. For each key, the list of players associated with it is attached. **Note:**  This endpoint is currently in beta and might be subject to changes.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetMpcKeysResponse**](GetMpcKeysResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_mpc_keys_list_by_user

> models::GetMpcKeysResponse get_mpc_keys_list_by_user(user_id)
Get list of mpc keys by `userId`

Returns a list of MPC signing keys of a specific user. For each key, the list of players associated with it is attached. **Note:** This endpoint is currently in beta and might be subject to changes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The id for the user | [required] |

### Return type

[**models::GetMpcKeysResponse**](GetMpcKeysResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

