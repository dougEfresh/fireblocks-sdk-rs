# UnspentInputsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**input** | Option<[**models::UnspentInput**](UnspentInput.md)> |  | [optional]
**address** | Option<**String**> | The blockchain address associated with the UTXO. Example: \"tb1qn83elmj3yhxsgrahdhtf9vgyxkfzqa5vy0eswr\"  | [optional]
**amount** | Option<**String**> | The quantity of the asset held in the UTXO, expressed in base asset units. Example: \"0.001\"  | [optional]
**confirmations** | Option<**f64**> | The count of confirmations on the blockchain for this UTXO. Example: 231313  | [optional]
**status** | Option<**String**> | Indicates the current state of the unspent input: - `AVAILABLE`: The unspent transaction output (UTXO) is available for spending (confirmed as per the specified confirmation policy) - `PENDING`: The UTXO is in the process of being confirmed and has not yet reached the required number of confirmations - `FROZEN`: The UTXO has been frozen, preventing it from being spent. For more details, refer to the [Fireblocks documentation on freezing transactions](https://developers.fireblocks.com/reference/post_transactions-txid-freeze)  Example: 'AVAILABLE'  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


