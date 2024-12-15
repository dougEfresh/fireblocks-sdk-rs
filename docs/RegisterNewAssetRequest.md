# RegisterNewAssetRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**blockchain_id** | **String** | Native asset ID of the blockchain | 
**address** | **String** | Asset address. - EVM-based chains: token contract address - Stellar (XLM): issuer address - Algorand (ALGO): asset ID - TRON (TRX): token contract address - NEAR: token address - Solana: token's mint account address  | 
**symbol** | Option<**String**> | Required for Stellar only, asset code is expected. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


