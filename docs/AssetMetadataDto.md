# AssetMetadataDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**asset_id** | **String** | The Fireblocks` asset id | 
**name** | Option<**String**> | The name of the token | [optional]
**symbol** | Option<**String**> | The symbol of the token | [optional]
**network_protocol** | Option<**String**> | The network protocol of the token | [optional]
**total_supply** | Option<**String**> | The total supply of the token | [optional]
**holders_count** | Option<**f64**> | The number of holders of the token | [optional]
**r#type** | Option<**String**> | The type of the token | [optional]
**contract_address** | Option<**String**> | The address of the token contract | [optional]
**issuer_address** | Option<**String**> | In case of Stellar or Ripple, the address of the issuer of the token | [optional]
**testnet** | Option<**bool**> | Is it deployed on testnet or to mainnet | [optional]
**blockchain** | Option<**String**> | The blockchain native asset id which the token is deployed on | [optional]
**decimals** | Option<**f64**> | The number of decimals of the token | [optional]
**vault_account_id** | Option<**String**> | The id of the vault account that initiated the request to issue the token. Will be empty if token was issued outside of Fireblocks. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


