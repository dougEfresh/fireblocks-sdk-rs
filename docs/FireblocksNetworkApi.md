# \FireblocksNetworkApi

All URIs are relative to *https://api.fireblocks.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**check_third_party_routing**](FireblocksNetworkApi.md#check_third_party_routing) | **GET** /network_connections/{connectionId}/is_third_party_routing/{assetType} | Retrieve third-party network routing validation
[**create_network_connection**](FireblocksNetworkApi.md#create_network_connection) | **POST** /network_connections | Creates a new network connection
[**create_network_id**](FireblocksNetworkApi.md#create_network_id) | **POST** /network_ids | Creates a new Network ID
[**delete_network_connection**](FireblocksNetworkApi.md#delete_network_connection) | **DELETE** /network_connections/{connectionId} | Deletes a network connection by ID
[**delete_network_id**](FireblocksNetworkApi.md#delete_network_id) | **DELETE** /network_ids/{networkId} | Deletes specific network ID.
[**get_network**](FireblocksNetworkApi.md#get_network) | **GET** /network_connections/{connectionId} | Get a network connection
[**get_network_connections**](FireblocksNetworkApi.md#get_network_connections) | **GET** /network_connections | List network connections
[**get_network_id**](FireblocksNetworkApi.md#get_network_id) | **GET** /network_ids/{networkId} | Returns specific network ID.
[**get_network_ids**](FireblocksNetworkApi.md#get_network_ids) | **GET** /network_ids | Get all network IDs
[**get_routing_policy_asset_groups**](FireblocksNetworkApi.md#get_routing_policy_asset_groups) | **GET** /network_ids/routing_policy_asset_groups | Returns all enabled routing policy asset groups
[**search_network_ids**](FireblocksNetworkApi.md#search_network_ids) | **GET** /network_ids/search | Get both local IDs and discoverable remote IDs
[**set_network_id_discoverability**](FireblocksNetworkApi.md#set_network_id_discoverability) | **PATCH** /network_ids/{networkId}/set_discoverability | Update network ID's discoverability.
[**set_network_id_name**](FireblocksNetworkApi.md#set_network_id_name) | **PATCH** /network_ids/{networkId}/set_name | Update network ID's name.
[**set_network_id_routing_policy**](FireblocksNetworkApi.md#set_network_id_routing_policy) | **PATCH** /network_ids/{networkId}/set_routing_policy | Update network id routing policy.
[**set_routing_policy**](FireblocksNetworkApi.md#set_routing_policy) | **PATCH** /network_connections/{connectionId}/set_routing_policy | Update network connection routing policy.



## check_third_party_routing

> models::ThirdPartyRouting check_third_party_routing(connection_id, asset_type)
Retrieve third-party network routing validation

The Fireblocks Network allows for flexibility around incoming deposits. A receiver can receive network deposits to locations other than Fireblocks. This endpoint validates whether future transactions are routed to the displayed recipient or to a 3rd party.  </br>Endpoint Permission: Admin, Non-Signing Admin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connection_id** | **String** | The ID of the network connection | [required] |
**asset_type** | **String** | The destination asset type | [required] |

### Return type

[**models::ThirdPartyRouting**](ThirdPartyRouting.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_network_connection

> models::NetworkConnectionResponse create_network_connection(idempotency_key, network_connection)
Creates a new network connection

Initiates a new network connection.  **Note:** This API call is subject to Flexible Routing Schemes.  Your routing policy defines how your transactions are routed. You can choose 1 of the 3 different schemes mentioned below for each asset type:   - **None**; Defines the profile routing to no destination for that asset type. Incoming transactions to asset types routed to `None` will fail.   - **Custom**; Route to an account that you choose. If you remove the account, incoming transactions will fail until you choose another one.   - **Default**; Use the routing specified by the network profile the connection is connected to. This scheme is also referred to as \"Profile Routing\"  Default Workspace Presets:   - Network Profile Crypto → **Custom**   - Network Profile FIAT → **None**   - Network Connection Crypto → **Default**   - Network Connection FIAT → **Default**  Supported asset groups for routing policy can be found at `/network_ids/routing_policy_asset_groups` **Note**: By default, Custom routing scheme uses (`dstId` = `0`, `dstType` = `VAULT`). Learn more about Fireblocks Network in the following [guide](https://developers.fireblocks.com/docs/connect-to-the-fireblocks-network). </br>Endpoint Permission: Admin, Non-Signing Admin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |
**network_connection** | Option<[**NetworkConnection**](NetworkConnection.md)> |  |  |

### Return type

[**models::NetworkConnectionResponse**](NetworkConnectionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_network_id

> models::NetworkIdResponse create_network_id(idempotency_key, create_network_id_request)
Creates a new Network ID

Creates a new Network ID.  **Note:** This API call is subject to Flexible Routing Schemes.  Your routing policy defines how your transactions are routed. You can choose 1 of the 3 different schemes mentioned below for each asset type:   - **None**; Defines the profile routing to no destination for that asset type. Incoming transactions to asset types routed to `None` will fail.   - **Custom**; Route to an account that you choose. If you remove the account, incoming transactions will fail until you choose another one.   - **Default**; Use the routing specified by the network profile the connection is connected to. This scheme is also referred to as \"Profile Routing\"  Default Workspace Presets:   - Network Profile Crypto → **Custom**   - Network Profile FIAT → **None**   - Network Connection Crypto → **Default**   - Network Connection FIAT → **Default**  Supported asset groups for routing policy can be found at `/network_ids/routing_policy_asset_groups` **Note**: By default, Custom routing scheme uses (`dstId` = `0`, `dstType` = `VAULT`). </br>Endpoint Permission: Admin, Non-Signing Admin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |
**create_network_id_request** | Option<[**CreateNetworkIdRequest**](CreateNetworkIdRequest.md)> |  |  |

### Return type

[**models::NetworkIdResponse**](NetworkIdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_network_connection

> models::DeleteNetworkConnectionResponse delete_network_connection(connection_id)
Deletes a network connection by ID

Deletes an existing network connection specified by its connection ID.  **Note:** This API call is subject to Flexible Routing Schemes.  Your routing policy defines how your transactions are routed. You can choose 1 of the 3 different schemes mentioned below for each asset type:   - **None**; Defines the profile routing to no destination for that asset type. Incoming transactions to asset types routed to `None` will fail.   - **Custom**; Route to an account that you choose. If you remove the account, incoming transactions will fail until you choose another one.   - **Default**; Use the routing specified by the network profile the connection is connected to. This scheme is also referred to as \"Profile Routing\"  Default Workspace Presets:   - Network Profile Crypto → **Custom**   - Network Profile FIAT → **None**   - Network Connection Crypto → **Default**   - Network Connection FIAT → **Default**  **Note**: By default, Custom routing scheme uses (`dstId` = `0`, `dstType` = `VAULT`). </br>Endpoint Permission: Admin, Non-Signing Admin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connection_id** | **String** | The ID of the network connection to delete | [required] |

### Return type

[**models::DeleteNetworkConnectionResponse**](DeleteNetworkConnectionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_network_id

> models::DeleteNetworkIdResponse delete_network_id(network_id)
Deletes specific network ID.

Deletes a network by its ID.  **Note:** This API call is subject to Flexible Routing Schemes.  Your routing policy defines how your transactions are routed. You can choose 1 of the 3 different schemes mentioned below for each asset type:   - **None**; Defines the profile routing to no destination for that asset type. Incoming transactions to asset types routed to `None` will fail.   - **Custom**; Route to an account that you choose. If you remove the account, incoming transactions will fail until you choose another one.   - **Default**; Use the routing specified by the network profile the connection is connected to. This scheme is also referred to as \"Profile Routing\"  Default Workspace Presets:   - Network Profile Crypto → **Custom**   - Network Profile FIAT → **None**   - Network Connection Crypto → **Default**   - Network Connection FIAT → **Default**  **Note**: By default, Custom routing scheme uses (`dstId` = `0`, `dstType` = `VAULT`). </br>Endpoint Permission: Admin, Non-Signing Admin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**network_id** | **String** | The ID of the network | [required] |

### Return type

[**models::DeleteNetworkIdResponse**](DeleteNetworkIdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_network

> models::NetworkConnectionResponse get_network(connection_id)
Get a network connection

Gets a network connection by ID.  **Note:** This API call is subject to Flexible Routing Schemes.  Your routing policy defines how your transactions are routed. You can choose 1 of the 3 different schemes mentioned below for each asset type:   - **None**; Defines the profile routing to no destination for that asset type. Incoming transactions to asset types routed to `None` will fail.   - **Custom**; Route to an account that you choose. If you remove the account, incoming transactions will fail until you choose another one.   - **Default**; Use the routing specified by the network profile the connection is connected to. This scheme is also referred to as \"Profile Routing\"  Default Workspace Presets:   - Network Profile Crypto → **Custom**   - Network Profile FIAT → **None**   - Network Connection Crypto → **Default**   - Network Connection FIAT → **Default**  **Note**: By default, Custom routing scheme uses (`dstId` = `0`, `dstType` = `VAULT`). </br>Endpoint Permission: Admin, Non-Signing Admin.

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
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_network_connections

> Vec<models::NetworkConnectionResponse> get_network_connections()
List network connections

Returns all network connections.  **Note:** This API call is subject to Flexible Routing Schemes.  Your routing policy defines how your transactions are routed. You can choose 1 of the 3 different schemes mentioned below for each asset type:   - **None**; Defines the profile routing to no destination for that asset type. Incoming transactions to asset types routed to `None` will fail.   - **Custom**; Route to an account that you choose. If you remove the account, incoming transactions will fail until you choose another one.   - **Default**; Use the routing specified by the network profile the connection is connected to. This scheme is also referred to as \"Profile Routing\"  Default Workspace Presets:   - Network Profile Crypto → **Custom**   - Network Profile FIAT → **None**   - Network Connection Crypto → **Default**   - Network Connection FIAT → **Default**      - **Note**: By default, Custom routing scheme uses (`dstId` = `0`, `dstType` = `VAULT`).  </br>Endpoint Permission: Admin, Non-Signing Admin.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::NetworkConnectionResponse>**](NetworkConnectionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_network_id

> models::NetworkIdResponse get_network_id(network_id)
Returns specific network ID.

Retrieves a network by its ID.  **Note:** This API call is subject to Flexible Routing Schemes.  Your routing policy defines how your transactions are routed. You can choose 1 of the 3 different schemes mentioned below for each asset type:   - **None**; Defines the profile routing to no destination for that asset type. Incoming transactions to asset types routed to `None` will fail.   - **Custom**; Route to an account that you choose. If you remove the account, incoming transactions will fail until you choose another one.   - **Default**; Use the routing specified by the network profile the connection is connected to. This scheme is also referred to as \"Profile Routing\"  Default Workspace Presets:   - Network Profile Crypto → **Custom**   - Network Profile FIAT → **None**   - Network Connection Crypto → **Default**   - Network Connection FIAT → **Default**  **Note**: By default, Custom routing scheme uses (`dstId` = `0`, `dstType` = `VAULT`). </br>Endpoint Permission: Admin, Non-Signing Admin.

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
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_network_ids

> Vec<models::NetworkIdResponse> get_network_ids()
Get all network IDs

Retrieves a list of all local and discoverable remote network IDs.  **Note:** This API call is subject to Flexible Routing Schemes.  Your routing policy defines how your transactions are routed. You can choose 1 of the 3 different schemes mentioned below for each asset type:   - **None**; Defines the profile routing to no destination for that asset type. Incoming transactions to asset types routed to `None` will fail.   - **Custom**; Route to an account that you choose. If you remove the account, incoming transactions will fail until you choose another one.   - **Default**; Use the routing specified by the network profile the connection is connected to. This scheme is also referred to as \"Profile Routing\"  Default Workspace Presets:   - Network Profile Crypto → **Custom**   - Network Profile FIAT → **None**   - Network Connection Crypto → **Default**   - Network Connection FIAT → **Default**  **Note**: By default, Custom routing scheme uses (`dstId` = `0`, `dstType` = `VAULT`). </br>Endpoint Permission: Admin, Non-Signing Admin.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::NetworkIdResponse>**](NetworkIdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_routing_policy_asset_groups

> Vec<String> get_routing_policy_asset_groups()
Returns all enabled routing policy asset groups

Retrieves a list of all enabled routing policy asset groups. Your routing policy defines how your transactions are routed. You can use one or more enabled routing policy asset groups to describe connection or network id routing policy. </br>Endpoint Permission: Admin, Non-Signing Admin.

### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_network_ids

> models::SearchNetworkIdsResponse search_network_ids(search, exclude_self, exclude_connected, page_cursor, page_size)
Get both local IDs and discoverable remote IDs

Retrieves a list of all local and discoverable remote network IDs. Can be filtered.  **Note:** This API call is subject to Flexible Routing Schemes.  Your routing policy defines how your transactions are routed. You can choose 1 of the 3 different schemes mentioned below for each asset type:   - **None**; Defines the profile routing to no destination for that asset type. Incoming transactions to asset types routed to `None` will fail.   - **Custom**; Route to an account that you choose. If you remove the account, incoming transactions will fail until you choose another one.   - **Default**; Use the routing specified by the network profile the connection is connected to. This scheme is also referred to as \"Profile Routing\"  Default Workspace Presets:   - Network Profile Crypto → **Custom**   - Network Profile FIAT → **None**   - Network Connection Crypto → **Default**   - Network Connection FIAT → **Default**      - **Note**: By default, Custom routing scheme uses (`dstId` = `0`, `dstType` = `VAULT`). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search** | Option<**String**> | Search string - displayName networkId. Optional |  |
**exclude_self** | Option<**bool**> | Exclude your networkIds. Optional, default false |  |
**exclude_connected** | Option<**bool**> | Exclude connected networkIds. Optional, default false |  |
**page_cursor** | Option<**String**> | ID of the record after which to fetch $limit records |  |
**page_size** | Option<**f64**> | Number of records to fetch. By default, it is 50 |  |[default to 50]

### Return type

[**models::SearchNetworkIdsResponse**](SearchNetworkIdsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_network_id_discoverability

> models::SetNetworkIdResponse set_network_id_discoverability(network_id, set_network_id_discoverability_request)
Update network ID's discoverability.

Update whether or not the network ID is discoverable by others.  **Note:** This API call is subject to Flexible Routing Schemes.  Your routing policy defines how your transactions are routed. You can choose 1 of the 3 different schemes mentioned below for each asset type:   - **None**; Defines the profile routing to no destination for that asset type. Incoming transactions to asset types routed to `None` will fail.   - **Custom**; Route to an account that you choose. If you remove the account, incoming transactions will fail until you choose another one.   - **Default**; Use the routing specified by the network profile the connection is connected to. This scheme is also referred to as \"Profile Routing\"  Default Workspace Presets:   - Network Profile Crypto → **Custom**   - Network Profile FIAT → **None**   - Network Connection Crypto → **Default**   - Network Connection FIAT → **Default**  **Note**: By default, Custom routing scheme uses (`dstId` = `0`, `dstType` = `VAULT`). </br>Endpoint Permission: Admin, Non-Signing Admin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**network_id** | **String** | The ID of the network | [required] |
**set_network_id_discoverability_request** | [**SetNetworkIdDiscoverabilityRequest**](SetNetworkIdDiscoverabilityRequest.md) |  | [required] |

### Return type

[**models::SetNetworkIdResponse**](SetNetworkIdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_network_id_name

> models::SetNetworkIdResponse set_network_id_name(network_id, set_network_id_name_request)
Update network ID's name.

Updates name of a specified network ID.  **Note:** This API call is subject to Flexible Routing Schemes.  Your routing policy defines how your transactions are routed. You can choose 1 of the 3 different schemes mentioned below for each asset type:   - **None**; Defines the profile routing to no destination for that asset type. Incoming transactions to asset types routed to `None` will fail.   - **Custom**; Route to an account that you choose. If you remove the account, incoming transactions will fail until you choose another one.   - **Default**; Use the routing specified by the network profile the connection is connected to. This scheme is also referred to as \"Profile Routing\"  Default Workspace Presets:   - Network Profile Crypto → **Custom**   - Network Profile FIAT → **None**   - Network Connection Crypto → **Default**   - Network Connection FIAT → **Default**  **Note**: By default, Custom routing scheme uses (`dstId` = `0`, `dstType` = `VAULT`). </br>Endpoint Permission: Admin, Non-Signing Admin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**network_id** | **String** | The ID of the network | [required] |
**set_network_id_name_request** | [**SetNetworkIdNameRequest**](SetNetworkIdNameRequest.md) |  | [required] |

### Return type

[**models::SetNetworkIdResponse**](SetNetworkIdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_network_id_routing_policy

> models::SetNetworkIdResponse set_network_id_routing_policy(network_id, set_network_id_routing_policy_request)
Update network id routing policy.

Updates the routing policy of a specified network ID.  **Note:** This API call is subject to Flexible Routing Schemes.  Your routing policy defines how your transactions are routed. You can choose 1 of the 3 different schemes mentioned below for each asset type:   - **None**; Defines the profile routing to no destination for that asset type. Incoming transactions to asset types routed to `None` will fail.   - **Custom**; Route to an account that you choose. If you remove the account, incoming transactions will fail until you choose another one.   - **Default**; Use the routing specified by the network profile the connection is connected to. This scheme is also referred to as \"Profile Routing\"  Default Workspace Presets:   - Network Profile Crypto → **Custom**   - Network Profile FIAT → **None**   - Network Connection Crypto → **Default**   - Network Connection FIAT → **Default**  Supported asset groups for routing policy can be found at `/network_ids/routing_policy_asset_groups` **Note**: By default, Custom routing scheme uses (`dstId` = `0`, `dstType` = `VAULT`). </br>Endpoint Permission: Admin, Non-Signing Admin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**network_id** | **String** | The ID of the network | [required] |
**set_network_id_routing_policy_request** | Option<[**SetNetworkIdRoutingPolicyRequest**](SetNetworkIdRoutingPolicyRequest.md)> |  |  |

### Return type

[**models::SetNetworkIdResponse**](SetNetworkIdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_routing_policy

> models::SetRoutingPolicyResponse set_routing_policy(connection_id, set_routing_policy_request)
Update network connection routing policy.

Updates an existing network connection's routing policy.  **Note:** This API call is subject to Flexible Routing Schemes.  Your routing policy defines how your transactions are routed. You can choose 1 of the 3 different schemes mentioned below for each asset type:   - **None**; Defines the profile routing to no destination for that asset type. Incoming transactions to asset types routed to `None` will fail.   - **Custom**; Route to an account that you choose. If you remove the account, incoming transactions will fail until you choose another one.   - **Default**; Use the routing specified by the network profile the connection is connected to. This scheme is also referred to as \"Profile Routing\"  Default Workspace Presets:   - Network Profile Crypto → **Custom**   - Network Profile FIAT → **None**   - Network Connection Crypto → **Default**   - Network Connection FIAT → **Default**  Supported asset groups for routing policy can be found at `/network_ids/routing_policy_asset_groups` **Note**: By default, Custom routing scheme uses (`dstId` = `0`, `dstType` = `VAULT`).  </br>Endpoint Permission: Admin, Non-Signing Admin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connection_id** | **String** | The ID of the network connection | [required] |
**set_routing_policy_request** | Option<[**SetRoutingPolicyRequest**](SetRoutingPolicyRequest.md)> |  |  |

### Return type

[**models::SetRoutingPolicyResponse**](SetRoutingPolicyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

