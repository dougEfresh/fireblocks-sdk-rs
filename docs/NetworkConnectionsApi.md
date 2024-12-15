# \NetworkConnectionsApi

All URIs are relative to *https://api.fireblocks.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**network_connections_connection_id_delete**](NetworkConnectionsApi.md#network_connections_connection_id_delete) | **DELETE** /network_connections/{connectionId} | Deletes a network connection by ID
[**network_connections_connection_id_get**](NetworkConnectionsApi.md#network_connections_connection_id_get) | **GET** /network_connections/{connectionId} | Get a network connection
[**network_connections_connection_id_is_third_party_routing_asset_type_get**](NetworkConnectionsApi.md#network_connections_connection_id_is_third_party_routing_asset_type_get) | **GET** /network_connections/{connectionId}/is_third_party_routing/{assetType} | Retrieve third-party network routing validation by asset type.
[**network_connections_connection_id_set_routing_policy_patch**](NetworkConnectionsApi.md#network_connections_connection_id_set_routing_policy_patch) | **PATCH** /network_connections/{connectionId}/set_routing_policy | Update network connection routing policy.
[**network_connections_get**](NetworkConnectionsApi.md#network_connections_get) | **GET** /network_connections | List network connections
[**network_connections_post**](NetworkConnectionsApi.md#network_connections_post) | **POST** /network_connections | Creates a new network connection
[**network_ids_get**](NetworkConnectionsApi.md#network_ids_get) | **GET** /network_ids | Returns all network IDs, both local IDs and discoverable remote IDs
[**network_ids_network_id_delete**](NetworkConnectionsApi.md#network_ids_network_id_delete) | **DELETE** /network_ids/{networkId} | Deletes specific network ID.
[**network_ids_network_id_get**](NetworkConnectionsApi.md#network_ids_network_id_get) | **GET** /network_ids/{networkId} | Returns specific network ID.
[**network_ids_network_id_set_discoverability_patch**](NetworkConnectionsApi.md#network_ids_network_id_set_discoverability_patch) | **PATCH** /network_ids/{networkId}/set_discoverability | Update network ID's discoverability.
[**network_ids_network_id_set_name_patch**](NetworkConnectionsApi.md#network_ids_network_id_set_name_patch) | **PATCH** /network_ids/{networkId}/set_name | Update network ID's name.
[**network_ids_network_id_set_routing_policy_patch**](NetworkConnectionsApi.md#network_ids_network_id_set_routing_policy_patch) | **PATCH** /network_ids/{networkId}/set_routing_policy | Update network id routing policy.
[**network_ids_post**](NetworkConnectionsApi.md#network_ids_post) | **POST** /network_ids | Creates a new Network ID



## network_connections_connection_id_delete

> models::NetworkConnectionsConnectionIdSetRoutingPolicyPatch200Response network_connections_connection_id_delete(connection_id)
Deletes a network connection by ID

Deletes an existing network connection specified by its connection ID.  **Note:** This API call is subject to Flexible Routing Schemes.  Your routing policy defines how your transactions are routed. You can choose 1 of the 3 different schemes mentioned below for each asset type:   - **None**; Defines the profile routing to no destination for that asset type. Incoming transactions to asset types routed to `None` will fail.   - **Custom**; Route to an account that you choose. If you remove the account, incoming transactions will fail until you choose another one.   - **Default**; Use the routing specified by the network profile the connection is connected to. This scheme is also referred to as \"Profile Routing\"  Default Workspace Presets:   - Network Profile Crypto → **Custom**   - Network Profile FIAT → **None**   - Network Connection Crypto → **Default**   - Network Connection FIAT → **Default**      - **Note**: By default, Custom routing scheme uses (`dstId` = `0`, `dstType` = `VAULT`). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connection_id** | **String** | The ID of the network connection to delete | [required] |

### Return type

[**models::NetworkConnectionsConnectionIdSetRoutingPolicyPatch200Response**](_network_connections__connectionId__set_routing_policy_patch_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## network_connections_connection_id_get

> models::NetworkConnectionResponse network_connections_connection_id_get(connection_id)
Get a network connection

Gets a network connection by ID.  **Note:** This API call is subject to Flexible Routing Schemes.  Your routing policy defines how your transactions are routed. You can choose 1 of the 3 different schemes mentioned below for each asset type:   - **None**; Defines the profile routing to no destination for that asset type. Incoming transactions to asset types routed to `None` will fail.   - **Custom**; Route to an account that you choose. If you remove the account, incoming transactions will fail until you choose another one.   - **Default**; Use the routing specified by the network profile the connection is connected to. This scheme is also referred to as \"Profile Routing\"  Default Workspace Presets:   - Network Profile Crypto → **Custom**   - Network Profile FIAT → **None**   - Network Connection Crypto → **Default**   - Network Connection FIAT → **Default**      - **Note**: By default, Custom routing scheme uses (`dstId` = `0`, `dstType` = `VAULT`). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connection_id** | **String** | The ID of the connection | [required] |

### Return type

[**models::NetworkConnectionResponse**](NetworkConnectionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## network_connections_connection_id_is_third_party_routing_asset_type_get

> models::NetworkConnectionsConnectionIdIsThirdPartyRoutingAssetTypeGet200Response network_connections_connection_id_is_third_party_routing_asset_type_get(connection_id, asset_type)
Retrieve third-party network routing validation by asset type.

The Fireblocks Network allows for flexibility around incoming deposits. A receiver can receive network deposits to locations other than Fireblocks. This endpoint validates whether future transactions are routed to the displayed recipient or to a 3rd party.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connection_id** | **String** | The ID of the network connection | [required] |
**asset_type** | **String** | The destination asset type | [required] |

### Return type

[**models::NetworkConnectionsConnectionIdIsThirdPartyRoutingAssetTypeGet200Response**](_network_connections__connectionId__is_third_party_routing__assetType__get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## network_connections_connection_id_set_routing_policy_patch

> models::NetworkConnectionsConnectionIdSetRoutingPolicyPatch200Response network_connections_connection_id_set_routing_policy_patch(connection_id, network_connections_connection_id_set_routing_policy_patch_request)
Update network connection routing policy.

Updates an existing network connection's routing policy.  **Note:** This API call is subject to Flexible Routing Schemes.  Your routing policy defines how your transactions are routed. You can choose 1 of the 3 different schemes mentioned below for each asset type:   - **None**; Defines the profile routing to no destination for that asset type. Incoming transactions to asset types routed to `None` will fail.   - **Custom**; Route to an account that you choose. If you remove the account, incoming transactions will fail until you choose another one.   - **Default**; Use the routing specified by the network profile the connection is connected to. This scheme is also referred to as \"Profile Routing\"  Default Workspace Presets:   - Network Profile Crypto → **Custom**   - Network Profile FIAT → **None**   - Network Connection Crypto → **Default**   - Network Connection FIAT → **Default**      - **Note**: By default, Custom routing scheme uses (`dstId` = `0`, `dstType` = `VAULT`). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connection_id** | **String** | The ID of the network connection | [required] |
**network_connections_connection_id_set_routing_policy_patch_request** | Option<[**NetworkConnectionsConnectionIdSetRoutingPolicyPatchRequest**](NetworkConnectionsConnectionIdSetRoutingPolicyPatchRequest.md)> |  |  |

### Return type

[**models::NetworkConnectionsConnectionIdSetRoutingPolicyPatch200Response**](_network_connections__connectionId__set_routing_policy_patch_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## network_connections_get

> Vec<models::NetworkConnectionResponse> network_connections_get()
List network connections

Returns all network connections.  **Note:** This API call is subject to Flexible Routing Schemes.  Your routing policy defines how your transactions are routed. You can choose 1 of the 3 different schemes mentioned below for each asset type:   - **None**; Defines the profile routing to no destination for that asset type. Incoming transactions to asset types routed to `None` will fail.   - **Custom**; Route to an account that you choose. If you remove the account, incoming transactions will fail until you choose another one.   - **Default**; Use the routing specified by the network profile the connection is connected to. This scheme is also referred to as \"Profile Routing\"  Default Workspace Presets:   - Network Profile Crypto → **Custom**   - Network Profile FIAT → **None**   - Network Connection Crypto → **Default**   - Network Connection FIAT → **Default**      - **Note**: By default, Custom routing scheme uses (`dstId` = `0`, `dstType` = `VAULT`). 

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::NetworkConnectionResponse>**](NetworkConnectionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## network_connections_post

> models::NetworkConnectionResponse network_connections_post(network_connection)
Creates a new network connection

Initiates a new network connection.  **Note:** This API call is subject to Flexible Routing Schemes.  Your routing policy defines how your transactions are routed. You can choose 1 of the 3 different schemes mentioned below for each asset type:   - **None**; Defines the profile routing to no destination for that asset type. Incoming transactions to asset types routed to `None` will fail.   - **Custom**; Route to an account that you choose. If you remove the account, incoming transactions will fail until you choose another one.   - **Default**; Use the routing specified by the network profile the connection is connected to. This scheme is also referred to as \"Profile Routing\"  Default Workspace Presets:   - Network Profile Crypto → **Custom**   - Network Profile FIAT → **None**   - Network Connection Crypto → **Default**   - Network Connection FIAT → **Default**      - **Note**: By default, Custom routing scheme uses (`dstId` = `0`, `dstType` = `VAULT`). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**network_connection** | Option<[**NetworkConnection**](NetworkConnection.md)> |  |  |

### Return type

[**models::NetworkConnectionResponse**](NetworkConnectionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## network_ids_get

> Vec<models::NetworkIdResponse> network_ids_get()
Returns all network IDs, both local IDs and discoverable remote IDs

Retrieves a list of all local and discoverable remote network IDs.  **Note:** This API call is subject to Flexible Routing Schemes.  Your routing policy defines how your transactions are routed. You can choose 1 of the 3 different schemes mentioned below for each asset type:   - **None**; Defines the profile routing to no destination for that asset type. Incoming transactions to asset types routed to `None` will fail.   - **Custom**; Route to an account that you choose. If you remove the account, incoming transactions will fail until you choose another one.   - **Default**; Use the routing specified by the network profile the connection is connected to. This scheme is also referred to as \"Profile Routing\"  Default Workspace Presets:   - Network Profile Crypto → **Custom**   - Network Profile FIAT → **None**   - Network Connection Crypto → **Default**   - Network Connection FIAT → **Default**      - **Note**: By default, Custom routing scheme uses (`dstId` = `0`, `dstType` = `VAULT`). 

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::NetworkIdResponse>**](NetworkIdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## network_ids_network_id_delete

> models::NetworkConnectionsConnectionIdSetRoutingPolicyPatch200Response network_ids_network_id_delete(network_id)
Deletes specific network ID.

Deletes a network by its ID.  **Note:** This API call is subject to Flexible Routing Schemes.  Your routing policy defines how your transactions are routed. You can choose 1 of the 3 different schemes mentioned below for each asset type:   - **None**; Defines the profile routing to no destination for that asset type. Incoming transactions to asset types routed to `None` will fail.   - **Custom**; Route to an account that you choose. If you remove the account, incoming transactions will fail until you choose another one.   - **Default**; Use the routing specified by the network profile the connection is connected to. This scheme is also referred to as \"Profile Routing\"  Default Workspace Presets:   - Network Profile Crypto → **Custom**   - Network Profile FIAT → **None**   - Network Connection Crypto → **Default**   - Network Connection FIAT → **Default**      - **Note**: By default, Custom routing scheme uses (`dstId` = `0`, `dstType` = `VAULT`). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**network_id** | **String** | The ID of the network | [required] |

### Return type

[**models::NetworkConnectionsConnectionIdSetRoutingPolicyPatch200Response**](_network_connections__connectionId__set_routing_policy_patch_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## network_ids_network_id_get

> models::NetworkIdResponse network_ids_network_id_get(network_id)
Returns specific network ID.

Retrieves a network by its ID.  **Note:** This API call is subject to Flexible Routing Schemes.  Your routing policy defines how your transactions are routed. You can choose 1 of the 3 different schemes mentioned below for each asset type:   - **None**; Defines the profile routing to no destination for that asset type. Incoming transactions to asset types routed to `None` will fail.   - **Custom**; Route to an account that you choose. If you remove the account, incoming transactions will fail until you choose another one.   - **Default**; Use the routing specified by the network profile the connection is connected to. This scheme is also referred to as \"Profile Routing\"  Default Workspace Presets:   - Network Profile Crypto → **Custom**   - Network Profile FIAT → **None**   - Network Connection Crypto → **Default**   - Network Connection FIAT → **Default**      - **Note**: By default, Custom routing scheme uses (`dstId` = `0`, `dstType` = `VAULT`). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**network_id** | **String** | The ID of the network | [required] |

### Return type

[**models::NetworkIdResponse**](NetworkIdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## network_ids_network_id_set_discoverability_patch

> models::NetworkConnectionsConnectionIdSetRoutingPolicyPatch200Response network_ids_network_id_set_discoverability_patch(network_id, network_ids_network_id_set_discoverability_patch_request)
Update network ID's discoverability.

Update whether or not the network ID is discoverable by others.  **Note:** This API call is subject to Flexible Routing Schemes.  Your routing policy defines how your transactions are routed. You can choose 1 of the 3 different schemes mentioned below for each asset type:   - **None**; Defines the profile routing to no destination for that asset type. Incoming transactions to asset types routed to `None` will fail.   - **Custom**; Route to an account that you choose. If you remove the account, incoming transactions will fail until you choose another one.   - **Default**; Use the routing specified by the network profile the connection is connected to. This scheme is also referred to as \"Profile Routing\"  Default Workspace Presets:   - Network Profile Crypto → **Custom**   - Network Profile FIAT → **None**   - Network Connection Crypto → **Default**   - Network Connection FIAT → **Default**      - **Note**: By default, Custom routing scheme uses (`dstId` = `0`, `dstType` = `VAULT`). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**network_id** | **String** | The ID of the network | [required] |
**network_ids_network_id_set_discoverability_patch_request** | [**NetworkIdsNetworkIdSetDiscoverabilityPatchRequest**](NetworkIdsNetworkIdSetDiscoverabilityPatchRequest.md) |  | [required] |

### Return type

[**models::NetworkConnectionsConnectionIdSetRoutingPolicyPatch200Response**](_network_connections__connectionId__set_routing_policy_patch_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## network_ids_network_id_set_name_patch

> models::NetworkConnectionsConnectionIdSetRoutingPolicyPatch200Response network_ids_network_id_set_name_patch(network_id, network_ids_network_id_set_name_patch_request)
Update network ID's name.

Updates name of a specified network ID.  **Note:** This API call is subject to Flexible Routing Schemes.  Your routing policy defines how your transactions are routed. You can choose 1 of the 3 different schemes mentioned below for each asset type:   - **None**; Defines the profile routing to no destination for that asset type. Incoming transactions to asset types routed to `None` will fail.   - **Custom**; Route to an account that you choose. If you remove the account, incoming transactions will fail until you choose another one.   - **Default**; Use the routing specified by the network profile the connection is connected to. This scheme is also referred to as \"Profile Routing\"  Default Workspace Presets:   - Network Profile Crypto → **Custom**   - Network Profile FIAT → **None**   - Network Connection Crypto → **Default**   - Network Connection FIAT → **Default**      - **Note**: By default, Custom routing scheme uses (`dstId` = `0`, `dstType` = `VAULT`). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**network_id** | **String** | The ID of the network | [required] |
**network_ids_network_id_set_name_patch_request** | [**NetworkIdsNetworkIdSetNamePatchRequest**](NetworkIdsNetworkIdSetNamePatchRequest.md) |  | [required] |

### Return type

[**models::NetworkConnectionsConnectionIdSetRoutingPolicyPatch200Response**](_network_connections__connectionId__set_routing_policy_patch_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## network_ids_network_id_set_routing_policy_patch

> models::NetworkConnectionsConnectionIdSetRoutingPolicyPatch200Response network_ids_network_id_set_routing_policy_patch(network_id, network_ids_network_id_set_routing_policy_patch_request)
Update network id routing policy.

Updates the routing policy of a specified network ID.  **Note:** This API call is subject to Flexible Routing Schemes.  Your routing policy defines how your transactions are routed. You can choose 1 of the 3 different schemes mentioned below for each asset type:   - **None**; Defines the profile routing to no destination for that asset type. Incoming transactions to asset types routed to `None` will fail.   - **Custom**; Route to an account that you choose. If you remove the account, incoming transactions will fail until you choose another one.   - **Default**; Use the routing specified by the network profile the connection is connected to. This scheme is also referred to as \"Profile Routing\"  Default Workspace Presets:   - Network Profile Crypto → **Custom**   - Network Profile FIAT → **None**   - Network Connection Crypto → **Default**   - Network Connection FIAT → **Default**      - **Note**: By default, Custom routing scheme uses (`dstId` = `0`, `dstType` = `VAULT`). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**network_id** | **String** | The ID of the network | [required] |
**network_ids_network_id_set_routing_policy_patch_request** | Option<[**NetworkIdsNetworkIdSetRoutingPolicyPatchRequest**](NetworkIdsNetworkIdSetRoutingPolicyPatchRequest.md)> |  |  |

### Return type

[**models::NetworkConnectionsConnectionIdSetRoutingPolicyPatch200Response**](_network_connections__connectionId__set_routing_policy_patch_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## network_ids_post

> models::NetworkIdResponse network_ids_post(network_ids_post_request)
Creates a new Network ID

Creates a new Network ID.  **Note:** This API call is subject to Flexible Routing Schemes.  Your routing policy defines how your transactions are routed. You can choose 1 of the 3 different schemes mentioned below for each asset type:   - **None**; Defines the profile routing to no destination for that asset type. Incoming transactions to asset types routed to `None` will fail.   - **Custom**; Route to an account that you choose. If you remove the account, incoming transactions will fail until you choose another one.   - **Default**; Use the routing specified by the network profile the connection is connected to. This scheme is also referred to as \"Profile Routing\"  Default Workspace Presets:   - Network Profile Crypto → **Custom**   - Network Profile FIAT → **None**   - Network Connection Crypto → **Default**   - Network Connection FIAT → **Default**      - **Note**: By default, Custom routing scheme uses (`dstId` = `0`, `dstType` = `VAULT`). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**network_ids_post_request** | Option<[**NetworkIdsPostRequest**](NetworkIdsPostRequest.md)> |  |  |

### Return type

[**models::NetworkIdResponse**](NetworkIdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

