# TravelRuleNaturalPerson

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<[**Vec<models::TravelRuleNaturalPersonNameIdentifier>**](TravelRuleNaturalPersonNameIdentifier.md)> | An array of structured name identifiers for the natural person, referencing the TravelRuleNaturalPersonNameIdentifier schema. | [optional]
**geographic_address** | Option<[**Vec<models::TravelRuleGeographicAddress>**](TravelRuleGeographicAddress.md)> | An array of geographic addresses associated with the natural person, referencing the TravelRuleGeographicAddress schema. | [optional]
**national_identification** | Option<[**models::TravelRuleNationalIdentification**](TravelRuleNationalIdentification.md)> | The national identification of the natural person, referencing the TravelRuleNationalIdentification schema. | [optional]
**date_and_place_of_birth** | Option<[**models::TravelRuleDateAndPlaceOfBirth**](TravelRuleDateAndPlaceOfBirth.md)> | The date and place of birth of the natural person, referencing the TravelRuleDateAndPlaceOfBirth schema. | [optional]
**customer_identification** | Option<**String**> | A unique identifier for the customer within the organization's context. The value must be encrypted. | [optional]
**country_of_residence** | Option<**String**> | The ISO-3166 Alpha-2 country code of the natural person's residence. The value must be encrypted. | [optional]
**customer_number** | Option<**String**> | A distinct identifier that uniquely identifies the customer within the organization. The value must be encrypted. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


