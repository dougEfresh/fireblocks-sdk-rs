# NetworkConnectionResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**local_channel** | Option<[**models::NetworkChannel**](NetworkChannel.md)> | Deprecated - Replaced by `localNetworkId` | [optional]
**remote_channel** | Option<[**models::NetworkChannel**](NetworkChannel.md)> | Deprecated - Replaced by `remoteNetworkId` | [optional]
**status** | [**models::NetworkConnectionStatus**](NetworkConnectionStatus.md) |  | 
**local_network_id** | [**models::NetworkId**](NetworkId.md) |  | 
**remote_network_id** | [**models::NetworkId**](NetworkId.md) |  | 
**routing_policy** | [**std::collections::HashMap<String, models::NetworkConnectionRoutingPolicyValue>**](NetworkConnectionRoutingPolicy_value.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


