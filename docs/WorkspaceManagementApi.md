# \WorkspaceManagementApi

All URIs are relative to *https://api.fireblocks.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_user_group**](WorkspaceManagementApi.md#create_user_group) | **POST** /management/user_groups | Create a user group
[**delete_user_group**](WorkspaceManagementApi.md#delete_user_group) | **DELETE** /management/user_groups/{groupId} | Delete user group
[**get_audits**](WorkspaceManagementApi.md#get_audits) | **GET** /management/audit_logs | Get audit logs
[**get_ota_status**](WorkspaceManagementApi.md#get_ota_status) | **GET** /management/ota | Return status of One Time Address (OTA) enabled
[**get_user_group**](WorkspaceManagementApi.md#get_user_group) | **GET** /management/user_groups/{groupId} | Get a user group by ID
[**get_user_groups**](WorkspaceManagementApi.md#get_user_groups) | **GET** /management/user_groups | List user groups
[**get_workspace_status**](WorkspaceManagementApi.md#get_workspace_status) | **GET** /management/workspace_status | Return current workspace status
[**management_api_users_get**](WorkspaceManagementApi.md#management_api_users_get) | **GET** /management/api_users | Get API users (Beta)
[**management_api_users_post**](WorkspaceManagementApi.md#management_api_users_post) | **POST** /management/api_users | Creates an API user (Beta)
[**management_api_users_user_id_whitelist_ip_addresses_get**](WorkspaceManagementApi.md#management_api_users_user_id_whitelist_ip_addresses_get) | **GET** /management/api_users/{userId}/whitelist_ip_addresses | Get list of whitelisted IP addresses
[**management_users_get**](WorkspaceManagementApi.md#management_users_get) | **GET** /management/users | Get Console users
[**management_users_id_reset_device_post**](WorkspaceManagementApi.md#management_users_id_reset_device_post) | **POST** /management/users/{id}/reset_device | Reset device
[**management_users_post**](WorkspaceManagementApi.md#management_users_post) | **POST** /management/users | Create Console user
[**set_ota_status**](WorkspaceManagementApi.md#set_ota_status) | **POST** /management/ota | Enable or disable transactions to One Time Address (OTA)
[**update_user_group**](WorkspaceManagementApi.md#update_user_group) | **PUT** /management/user_groups/{groupId} | Update user group



## create_user_group

> models::CreateUserGroupResponse create_user_group(user_group_create_request)
Create a user group

Create a new user group.</br> 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_group_create_request** | [**UserGroupCreateRequest**](UserGroupCreateRequest.md) |  | [required] |

### Return type

[**models::CreateUserGroupResponse**](CreateUserGroupResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_group

> delete_user_group(group_id)
Delete user group

Delete a user group by ID.</br>  **Note**: - Please note that this endpoint is available only for API keys with Admin permissions. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The ID of the users group | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_audits

> get_audits(time_period, cursor)
Get audit logs

Get audit logs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**time_period** | Option<**String**> | The final time period to fetch audit logs (day/week) |  |
**cursor** | Option<**String**> | The next ID to start fetching audit logs from |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ota_status

> models::GetOtaStatus200Response get_ota_status()
Return status of One Time Address (OTA) enabled

Returns current One Time Address (OTA) status

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetOtaStatus200Response**](getOtaStatus_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_group

> models::UserGroupResponse get_user_group(group_id)
Get a user group by ID

Get a user group by ID.</br> 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The ID of the users group | [required] |

### Return type

[**models::UserGroupResponse**](UserGroupResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_groups

> Vec<models::UserGroupResponse> get_user_groups()
List user groups

Get all user groups in the current workspace. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::UserGroupResponse>**](UserGroupResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workspace_status

> models::GetWorkspaceStatus200Response get_workspace_status()
Return current workspace status

Returns the status of the current workspace. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetWorkspaceStatus200Response**](getWorkspaceStatus_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## management_api_users_get

> management_api_users_get()
Get API users (Beta)

Get API users from the current tenant. </br> - **Note: This endpoint is in beta and subject to change** 

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## management_api_users_post

> management_api_users_post(create_api_user)
Creates an API user (Beta)

Creates an API user. - **Note: This endpoint is in beta and subject to change** 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_api_user** | Option<[**CreateApiUser**](CreateApiUser.md)> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## management_api_users_user_id_whitelist_ip_addresses_get

> management_api_users_user_id_whitelist_ip_addresses_get(user_id)
Get list of whitelisted IP addresses

Gets a list of whitelisted IP addresses, which make it possible to get API calls only from these IP addresses.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## management_users_get

> management_users_get()
Get Console users

Gets a list of Console users from the current tenant.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## management_users_id_reset_device_post

> management_users_id_reset_device_post(id)
Reset device

Re-links a mobile device to the workspace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The user’s ID | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## management_users_post

> management_users_post(create_console_user)
Create Console user

Creates a Fireblocks Console user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_console_user** | Option<[**CreateConsoleUser**](CreateConsoleUser.md)> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_ota_status

> set_ota_status(set_ota_status_request)
Enable or disable transactions to One Time Address (OTA)

Enable or disable transactions to One Time Address (OTA)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**set_ota_status_request** | [**SetOtaStatusRequest**](SetOtaStatusRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_group

> models::UserGroupCreateResponse update_user_group(group_id, user_group_update_request)
Update user group

Update a users group by ID.</br>  **Note**: - Please note that this endpoint is available only for API keys with Admin permissions. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The ID of the users group | [required] |
**user_group_update_request** | [**UserGroupUpdateRequest**](UserGroupUpdateRequest.md) |  | [required] |

### Return type

[**models::UserGroupCreateResponse**](UserGroupCreateResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

