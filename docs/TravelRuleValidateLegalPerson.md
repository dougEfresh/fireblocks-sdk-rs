# TravelRuleValidateLegalPerson

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<[**models::TravelRuleValidateLegalPersonNameIdentifier**](TravelRuleValidateLegalPersonNameIdentifier.md)> | The structured name of the legal person, referencing name identifiers. | [optional]
**geographic_address** | Option<[**Vec<models::TravelRuleValidateGeographicAddress>**](TravelRuleValidateGeographicAddress.md)> | The array of geographic addresses associated with the legal person. | [optional]
**national_identification** | Option<[**models::TravelRuleValidateNationalIdentification**](TravelRuleValidateNationalIdentification.md)> |  | [optional]
**customer_identification** | Option<**String**> | A unique identifier that identifies the customer in the organization's context. | [optional]
**customer_number** | Option<**String**> | A distinct identifier that uniquely identifies the customer within the organization. | [optional]
**country_of_registration** | Option<**String**> | The ISO-3166 Alpha-2 country code where the legal person is registered. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


