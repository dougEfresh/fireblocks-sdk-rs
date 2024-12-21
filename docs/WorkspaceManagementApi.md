# \WorkspaceManagementApi

All URIs are relative to *https://api.fireblocks.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_api_user**](WorkspaceManagementApi.md#create_api_user) | **POST** /management/api_users | Create API Key
[**create_console_user**](WorkspaceManagementApi.md#create_console_user) | **POST** /management/users | Create console user
[**create_user_group**](WorkspaceManagementApi.md#create_user_group) | **POST** /management/user_groups | Create user group
[**delete_user_group**](WorkspaceManagementApi.md#delete_user_group) | **DELETE** /management/user_groups/{groupId} | Delete user group
[**get_api_users**](WorkspaceManagementApi.md#get_api_users) | **GET** /management/api_users | Get API Keys
[**get_audit_logs**](WorkspaceManagementApi.md#get_audit_logs) | **GET** /management/audit_logs | Get audit logs
[**get_audits**](WorkspaceManagementApi.md#get_audits) | **GET** /audits | Get audit logs
[**get_console_users**](WorkspaceManagementApi.md#get_console_users) | **GET** /management/users | Get console users
[**get_ota_status**](WorkspaceManagementApi.md#get_ota_status) | **GET** /management/ota | Returns current One Time Address status
[**get_user_group**](WorkspaceManagementApi.md#get_user_group) | **GET** /management/user_groups/{groupId} | Get user group
[**get_user_groups**](WorkspaceManagementApi.md#get_user_groups) | **GET** /management/user_groups | List user groups
[**get_users**](WorkspaceManagementApi.md#get_users) | **GET** /users | List users
[**get_whitelist_ip_addresses**](WorkspaceManagementApi.md#get_whitelist_ip_addresses) | **GET** /management/api_users/{userId}/whitelist_ip_addresses | Get whitelisted ip addresses for an API Key
[**get_workspace_status**](WorkspaceManagementApi.md#get_workspace_status) | **GET** /management/workspace_status | Returns current workspace status
[**reset_device**](WorkspaceManagementApi.md#reset_device) | **POST** /management/users/{id}/reset_device | Resets device
[**set_ota_status**](WorkspaceManagementApi.md#set_ota_status) | **PUT** /management/ota | Enable or disable transactions to One Time Addresses
[**update_user_group**](WorkspaceManagementApi.md#update_user_group) | **PUT** /management/user_groups/{groupId} | Update user group



## create_api_user

> create_api_user(idempotency_key, create_api_user)
Create API Key

Create a new API key in your workspace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |
**create_api_user** | Option<[**CreateApiUser**](CreateApiUser.md)> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_console_user

> create_console_user(idempotency_key, create_console_user)
Create console user

Create console users in your workspace  - Please note that this endpoint is available only for API keys with Admin/Non Signing Admin permissions. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |
**create_console_user** | Option<[**CreateConsoleUser**](CreateConsoleUser.md)> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user_group

> models::CreateUserGroupResponse create_user_group(user_group_create_request, idempotency_key)
Create user group

Create a new user group.  - Please note that this endpoint is available only for API keys with Admin/Non Signing Admin permissions. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_group_create_request** | [**UserGroupCreateRequest**](UserGroupCreateRequest.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::CreateUserGroupResponse**](CreateUserGroupResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_group

> delete_user_group(group_id)
Delete user group

Delete a user group by ID.</br> - Please note that this endpoint is available only for API keys with Admin/Non Signing Admin permissions. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The ID of the user group | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_api_users

> models::GetApiUsersResponse get_api_users()
Get API Keys

List all API keys in your workspace.  - Please note that this endpoint is available only for API keys with Admin/Non Signing Admin permissions. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetApiUsersResponse**](GetAPIUsersResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_audit_logs

> models::GetAuditLogsResponse get_audit_logs(time_period, cursor)
Get audit logs

Get Audit logs for the last Day/Week.  - Please note that this endpoint is available only for API keys with Admin/Non Signing Admin permissions. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**time_period** | Option<**String**> | The last time period to fetch audit logs |  |
**cursor** | Option<**String**> | The next id to start fetch audit logs from |  |

### Return type

[**models::GetAuditLogsResponse**](GetAuditLogsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_audits

> models::GetAuditLogsResponseDto get_audits(time_period)
Get audit logs

Deprecated. Please use the `GET /management/audit_logs` endpoint instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**time_period** | Option<**String**> | The last time period to fetch audit logs |  |

### Return type

[**models::GetAuditLogsResponseDto**](GetAuditLogsResponseDTO.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_console_users

> models::GetConsoleUsersResponse get_console_users()
Get console users

Get console users for your workspace.  - Please note that this endpoint is available only for API keys with Admin/Non Signing Admin permissions. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetConsoleUsersResponse**](GetConsoleUsersResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ota_status

> models::GetOtaStatusResponse get_ota_status()
Returns current One Time Address status

Returns current One Time Address status  Learn more about [One Time Addresses](https://support.fireblocks.io/hc/en-us/articles/4409104568338-One-Time-Address-OTA-feature)  - Please note that this endpoint is available only for API keys with Admin/Non Signing Admin permissions. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetOtaStatusResponse**](GetOtaStatusResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_group

> models::UserGroupResponse get_user_group(group_id)
Get user group

Get a user group by ID  - Please note that this endpoint is available only for API keys with Admin/Non Signing Admin permissions. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The ID of the user group | [required] |

### Return type

[**models::UserGroupResponse**](UserGroupResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_groups

> Vec<models::UserGroupResponse> get_user_groups()
List user groups

Get all user groups in your workspace  - Please note that this endpoint is available only for API keys with Admin/Non Signing Admin permissions. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::UserGroupResponse>**](UserGroupResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users

> Vec<models::UserResponse> get_users()
List users

DEPRECATED - please use `GET /management/users` instead 

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::UserResponse>**](UserResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_whitelist_ip_addresses

> models::GetWhitelistIpAddressesResponse get_whitelist_ip_addresses(user_id)
Get whitelisted ip addresses for an API Key

Get a list of the whitelisted IP addresses for a specific API Key  - Please note that this endpoint is available only for API keys with Admin/Non Signing Admin permissions. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the api user | [required] |

### Return type

[**models::GetWhitelistIpAddressesResponse**](GetWhitelistIpAddressesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workspace_status

> models::GetWorkspaceStatusResponse get_workspace_status()
Returns current workspace status

Returns current workspace status (Beta)  **Note**:  - This endpoint is now in Beta, disabled for general availability at this time.  - Please note that this endpoint is available only for API keys with Admin/Non Signing Admin permissions. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetWorkspaceStatusResponse**](GetWorkspaceStatusResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_device

> reset_device(id, idempotency_key)
Resets device

Resets mobile device for given console user, that user will need to do mobile onboarding again.  - Please note that this endpoint is available only for API keys with Admin/Non Signing Admin permissions. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the console user | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_ota_status

> models::SetOtaStatusResponse set_ota_status(set_ota_status_request, idempotency_key)
Enable or disable transactions to One Time Addresses

Enable or disable transactions to One Time Addresses (Non Whitelisted addresses)  Learn more about [One Time Addresses](https://support.fireblocks.io/hc/en-us/articles/4409104568338-One-Time-Address-OTA-feature)  - Please note that this endpoint is available only for API keys with Admin/Non Signing Admin permissions. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**set_ota_status_request** | [**SetOtaStatusRequest**](SetOtaStatusRequest.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::SetOtaStatusResponse**](SetOtaStatusResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_group

> models::UserGroupCreateResponse update_user_group(group_id, user_group_update_request, idempotency_key)
Update user group

Update a user group by ID - Please note that this endpoint is available only for API keys with Admin permissions. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The ID of the user group | [required] |
**user_group_update_request** | [**UserGroupUpdateRequest**](UserGroupUpdateRequest.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::UserGroupCreateResponse**](UserGroupCreateResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

