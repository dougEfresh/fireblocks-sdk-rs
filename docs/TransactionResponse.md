# TransactionResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Fireblocks Transaction ID | 
**external_tx_id** | Option<**String**> | Unique externbal transaction identifier provided by the user. Fireblocks highly recommends setting an `externalTxId` for every transaction created, to avoid submitting the same transaction twice. | [optional]
**status** | [**models::TransactionStatus**](TransactionStatus.md) |  | 
**sub_status** | Option<[**models::TransactionSubStatus**](TransactionSubStatus.md)> |  | [optional]
**tx_hash** | Option<**String**> | The hash of the transaction on the blockchain.  * This parameter exists if at least one of the following conditions is met:       1. The transaction’s source type is `UNKNOWN`, `WHITELISTED_ADDRESS`, `NETWORK_CONNECTION`, `ONE_TIME_ADDRESS`, `FIAT_ACCOUNT` or `GAS_STATION`.       2. The transaction’s source type is `VAULT` and the status is either: `CONFIRMING`, `COMPLETED`, or was in any of these statuses prior to changing to `FAILED` or `REJECTED`. In some instances, transactions in status `BROADCASTING` will include the txHash as well.       3. The transaction’s source type is `EXCHANGE_ACCOUNT` and the transaction’s destination type is `VAULT`, and the status is either: `CONFIRMING`, `COMPLETED`, or was in any of these status prior to changing to `FAILED`.   * In addition, the following conditions must be met:      1. The asset is a crypto asset (not fiat).      2. The transaction operation is not `RAW` or `TYPED_MESSAGE`. | [optional]
**operation** | [**models::GetTransactionOperation**](GetTransactionOperation.md) |  | 
**note** | Option<**String**> | Custom note, not sent to the blockchain, that describes the transaction at your Fireblocks workspace. | [optional]
**asset_id** | Option<**String**> | The ID of the asset for `TRANSFER`, `MINT`, `BURN`, `ENABLE_ASSET`,`STAKE` ,`UNSTAKE` or `WITHDRAW` operations. [See the list of supported assets and their IDs on Fireblocks.](https://developers.fireblocks.com/reference/getsupportedassets) | [optional]
**source** | Option<[**models::SourceTransferPeerPathResponse**](SourceTransferPeerPathResponse.md)> |  | [optional]
**source_address** | Option<**String**> | For account based assets only, the source address of the transaction. **Note:** If the status is `CONFIRMING`, `COMPLETED`, or has been `CONFIRMING`; then moved forward to `FAILED` or `REJECTED`, then this parameter will contain the source address. In any other case, this parameter will be empty. | [optional]
**tag** | Option<**String**> | Source address tag for Tag/Memo supporting assets, or Bank Transfer Description for the fiat provider BLINC (by BCB Group). | [optional]
**destination** | Option<[**models::DestinationTransferPeerPathResponse**](DestinationTransferPeerPathResponse.md)> |  | [optional]
**destinations** | Option<[**Vec<models::TransactionResponseDestination>**](TransactionResponseDestination.md)> | The transaction’s destinations. **Note:** In case the transaction is sent to a single destination, the `destination` parameter is used instead of this. | [optional]
**destination_address** | Option<**String**> | Address where the asset were transferred. Notes:   - For [Multi destination transactions](https://support.fireblocks.io/hc/en-us/articles/360018447980-Multi-destination-transactions), this parameter will be empty. In this case, you should refer to the destinations field.   - If the status is `CONFIRMING`, `COMPLETED`, or has been `CONFIRMING`; then moved forward to `FAILED` or `REJECTED`, then this parameter will contain the destination address. In any other case, this parameter will be empty. | [optional]
**destination_address_description** | Option<**String**> | Description of the destination address. | [optional]
**destination_tag** | Option<**String**> | Destination address tag for Tag/Memo supporting assets, or Bank Transfer Description for the fiat provider BLINC (by BCB Group). | [optional]
**contract_call_decoded_data** | Option<[**models::TransactionResponseContractCallDecodedData**](TransactionResponse_contractCallDecodedData.md)> |  | [optional]
**amount_info** | Option<[**models::AmountInfo**](AmountInfo.md)> |  | [optional]
**treat_as_gross_amount** | Option<**bool**> | For transactions initiated via this Fireblocks workspace, when set to `true`, the fee is deducted from the requested amount.  **Note**: This parameter can only be considered if a transaction's asset is a base asset, such as ETH or MATIC. If the asset can't be used for transaction fees, like USDC, this parameter is ignored and the fee is deducted from the relevant base asset wallet in the source account. | [optional]
**fee_info** | Option<[**models::FeeInfo**](FeeInfo.md)> |  | [optional]
**fee_currency** | Option<**String**> | The asset which was withdrawn to pay the transaction fee, for example ETH for EVM-based blockchains, BTC for Tether Omni. | [optional]
**network_records** | Option<[**Vec<models::NetworkRecord>**](NetworkRecord.md)> | In case a single transaction resulted with multiple transfers, for example a result of a contract call, then this parameter specifies each transfer that took place on the blockchain. | [optional]
**created_at** | Option<**f64**> | The transaction’s creation date and time, in unix timestamp. | [optional]
**last_updated** | Option<**f64**> | The transaction’s last update date and time, in unix timestamp. | [optional]
**created_by** | Option<**String**> | User ID of the initiator of the transaction. | [optional]
**signed_by** | Option<**Vec<String>**> | User ID’s of the signers of the transaction. | [optional]
**rejected_by** | Option<**String**> | User ID of the user that rejected the transaction (in case it was rejected). | [optional]
**authorization_info** | Option<[**models::AuthorizationInfo**](AuthorizationInfo.md)> |  | [optional]
**exchange_tx_id** | Option<**String**> | If the transaction originated from an exchange, this is the ID of this transaction at the exchange. | [optional]
**customer_ref_id** | Option<**String**> | The ID for AML providers to associate the owner of funds with transactions. | [optional]
**aml_screening_result** | Option<[**models::AmlScreeningResult**](AmlScreeningResult.md)> |  | [optional]
**compliance_result** | Option<[**models::ComplianceResult**](ComplianceResult.md)> |  | [optional]
**extra_parameters** | Option<[**models::ExtraParameters**](ExtraParameters.md)> |  | [optional]
**signed_messages** | Option<[**Vec<models::SignedMessage>**](SignedMessage.md)> | An array of signed messages | [optional]
**num_of_confirmations** | Option<**f64**> | The number of confirmations of the transaction. The number will increase until the transaction will be considered completed according to the confirmation policy. | [optional]
**block_info** | Option<[**models::BlockInfo**](BlockInfo.md)> |  | [optional]
**index** | Option<**f64**> | For UTXO based assets this is the vOut, for Ethereum based, this is the index of the event of the contract call.  **Note:** This field is not returned if a transaction uses the `destinations` object with more than one value. | [optional]
**reward_info** | Option<[**models::RewardInfo**](RewardInfo.md)> |  | [optional]
**system_messages** | Option<[**models::SystemMessageInfo**](SystemMessageInfo.md)> |  | [optional]
**address_type** | Option<**String**> |  | [optional]
**requested_amount** | Option<**f64**> | The amount requested by the user. Deprecated - please use the `amountInfo` field for accuracy. | [optional]
**amount** | Option<**f64**> | If the transfer is a withdrawal from an exchange, the actual amount that was requested to be transferred. Otherwise, the requested amount. Deprecated - please use the `amountInfo` field for accuracy. | [optional]
**net_amount** | Option<**f64**> | The net amount of the transaction, after fee deduction. Deprecated - please use the `amountInfo` field for accuracy. | [optional]
**amount_usd** | Option<**f64**> | The USD value of the requested amount. Deprecated - please use the `amountInfo` field for accuracy. | [optional]
**service_fee** | Option<**f64**> | The total fee deducted by the exchange from the actual requested amount (`serviceFee` = `amount` - `netAmount`). Deprecated - please use the `feeInfo` field for accuracy. | [optional]
**fee** | Option<**f64**> | Deprecated - please use the `feeInfo` field for accuracy. | [optional]
**network_fee** | Option<**f64**> | The fee paid to the network. Deprecated - please use the `feeInfo` field for accuracy. | [optional]
**error_description** | Option<**String**> | The transaction's revert reason. This field will be returned when  `subStatus` =  'SMART_CONTRACT_EXECUTION_FAILED'. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


