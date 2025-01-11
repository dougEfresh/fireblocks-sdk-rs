# PayoutResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**payout_id** | **String** |  | 
**payment_account** | [**models::PaymentAccountResponse**](PaymentAccountResponse.md) |  | 
**created_at** | **f64** |  | 
**state** | [**models::PayoutState**](PayoutState.md) |  | 
**status** | [**models::PayoutStatus**](PayoutStatus.md) |  | 
**reason_of_failure** | Option<**String**> | <ul>  <li> INSUFFICIENT_BALANCE</li> <li> SOURCE_TRANSLATION</li> <li> SOURCE_NOT_UNIQUE</li> <li> SOURCE_NOT_FOUND</li> <li> SOURCE_TYPE_NOT_SUPPORTED</li> <li> EMPTY_SOURCE</li> <li> DESTINATION_TRANSLATION</li> <li> DESTINATION_NOT_UNIQUE</li> <li> DESTINATION_NOT_FOUND</li> <li> EMPTY_DESTINATION</li> <li> PARSING </li> <li> UNKNOWN</li> <li> FIREBLOCKS_CLIENT</li> <li> TRANSACTION_SUBMISSION</li> </ul>  | [optional]
**init_method** | Option<[**models::PayoutInitMethod**](PayoutInitMethod.md)> |  | [optional]
**instruction_set** | [**Vec<models::PayoutInstructionResponse>**](PayoutInstructionResponse.md) |  | 
**report_url** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


