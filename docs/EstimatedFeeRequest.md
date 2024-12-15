# EstimatedFeeRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**operation** | Option<[**models::TransactionOperation**](TransactionOperation.md)> |  | [optional]
**asset_id** | Option<**String**> | The ID of the asset to transfer, for `TRANSFER`, `MINT` or `BURN` operations. [See the list of supported assets and their IDs on Fireblocks.](https://developers.fireblocks.com/reference/get_supported-assets) | [optional]
**source** | Option<[**models::TransferPeerPath**](TransferPeerPath.md)> |  | [optional]
**destination** | Option<[**models::DestinationTransferPeerPath**](DestinationTransferPeerPath.md)> |  | [optional]
**destinations** | Option<[**Vec<models::TransactionRequestDestination>**](TransactionRequestDestination.md)> | For UTXO based blockchains, you can send a single transaction to multiple destinations. | [optional]
**amount** | Option<[**models::TransactionRequestAmount**](TransactionRequest_amount.md)> |  | [optional]
**treat_as_gross_amount** | Option<**bool**> | \"When set to `true`, the fee will be deducted from the requested amount.\"  **Note**: This parameter can only be considered if a transaction’s asset is a base asset, such as ETH or MATIC. If the asset can’t be used for transaction fees, like USDC, this parameter is ignored and the fee is deducted from the relevant base asset wallet in the source account. | [optional]
**extra_parameters** | Option<[**models::ExtraParameters**](ExtraParameters.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


