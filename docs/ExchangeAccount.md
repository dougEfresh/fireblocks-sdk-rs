# ExchangeAccount

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional]
**r#type** | Option<[**models::ExchangeType**](ExchangeType.md)> |  | [optional]
**name** | Option<**String**> | Display name of the exchange account | [optional]
**status** | Option<**String**> |  | [optional]
**assets** | Option<[**Vec<models::ExchangeAsset>**](ExchangeAsset.md)> |  | [optional]
**trading_accounts** | Option<[**Vec<models::ExchangeTradingAccount>**](ExchangeTradingAccount.md)> |  | [optional]
**is_subaccount** | Option<**bool**> | True if the account is a subaccount in an exchange | [optional]
**main_account_id** | Option<**String**> | if the account is a sub-account, the ID of the main account | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


