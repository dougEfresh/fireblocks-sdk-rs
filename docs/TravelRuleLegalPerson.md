# TravelRuleLegalPerson

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<[**models::TravelRuleLegalPersonNameIdentifier**](TravelRuleLegalPersonNameIdentifier.md)> | The structured name of the legal person, referencing name identifiers. | [optional]
**geographic_address** | Option<[**Vec<models::TravelRuleGeographicAddress>**](TravelRuleGeographicAddress.md)> | The array of geographic addresses associated with the legal person. | [optional]
**national_identification** | Option<[**models::TravelRuleNationalIdentification**](TravelRuleNationalIdentification.md)> |  | [optional]
**customer_identification** | Option<**String**> | A unique identifier that identifies the customer in the organization's context. The value must be encrypted. | [optional]
**customer_number** | Option<**String**> | A distinct identifier that uniquely identifies the customer within the organization. The value must be encrypted. | [optional]
**country_of_registration** | Option<**String**> | The ISO-3166 Alpha-2 country code where the legal person is registered. The value must be encrypted. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


