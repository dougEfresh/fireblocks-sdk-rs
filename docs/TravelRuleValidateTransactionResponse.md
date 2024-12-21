# TravelRuleValidateTransactionResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**is_valid** | **bool** | \"isValid\" will tell you if you have collected all the information needed for the travel rule data transfer. Once this field = \"true\", you can move on to the next step which is to transfer the front-end information to your back-end and perform Travel Rule Transaction create | 
**r#type** | **String** | \"type\" will tell you if the virtual asset value converted to FIAT value of the withdrawal request is above (=TRAVELRULE) or below (=BELOW_THRESHOLD) the threshold in your jurisdiction. If it is to an unhosted wallet which does not require travel rule information to be sent and only collected, it will say NON_CUSTODIAL. | 
**beneficiary_address_type** | **String** | \"beneficiaryAddressType\" will tell you if your blockchain analytics provider or internal address book has been able to identify the wallet address. | 
**address_source** | **String** | \"addressSource\" will tell you if the address was found in your internal address book or identified by the blockchain analytics provider. | 
**beneficiary_vas_pdid** | **String** | The VASP DID of the beneficiary VASP | 
**beneficiary_vas_pname** | **String** | \"beneficiaryVASPname\" will tell you the name of the VASP that has been identified as the owner of the wallet address. This name is used in a subsequent call to get its DID. | 
**warnings** | **Vec<String>** | \"errors/warnings\" will tell you what information about the beneficiary you need to collect from the sender. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


