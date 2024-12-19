# TravelRuleValidateTransactionRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**transaction_asset** | **String** | Transaction asset symbol BTC,ETH) | 
**destination** | **String** | Transaction destination address | 
**transaction_amount** | **String** | Transaction amount in the transaction asset | 
**originator_vas_pdid** | **String** | This is the identifier assigned to your VASP | 
**originator_equals_beneficiary** | **bool** | \"True\" if the originator and beneficiary is the same person and you therefore do not need to collect any information. \"False\" if it is a third-party transfer. | 
**travel_rule_behavior** | Option<**bool**> | This will also check if the transaction is a TRAVEL_RULE in the beneficiary VASP's jurisdiction | [optional]
**beneficiary_vas_pdid** | Option<**String**> | This is the identifier assigned to the VASP the funds are being sent to | [optional]
**beneficiary_vas_pname** | Option<**String**> | Beneficiary VASP name | [optional]
**beneficiary_name** | Option<**String**> | Beneficiary  name | [optional]
**beneficiary_account_number** | Option<**String**> | Beneficiary  name | [optional]
**beneficiary_address** | Option<[**models::TravelRuleAddress**](TravelRuleAddress.md)> | Beneficiary  name | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


