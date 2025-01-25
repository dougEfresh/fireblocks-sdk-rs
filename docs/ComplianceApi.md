# \ComplianceApi

All URIs are relative to *https://api.fireblocks.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_aml_post_screening_policy**](ComplianceApi.md#get_aml_post_screening_policy) | **GET** /screening/aml/post_screening_policy | AML - View Post-Screening Policy
[**get_aml_screening_configuration**](ComplianceApi.md#get_aml_screening_configuration) | **GET** /screening/aml/policy_configuration | Get AML Screening Policy Configuration
[**get_aml_screening_policy**](ComplianceApi.md#get_aml_screening_policy) | **GET** /screening/aml/screening_policy | AML - View Screening Policy
[**get_post_screening_policy**](ComplianceApi.md#get_post_screening_policy) | **GET** /screening/travel_rule/post_screening_policy | Travel Rule - View Post-Screening Policy
[**get_screening_configuration**](ComplianceApi.md#get_screening_configuration) | **GET** /screening/travel_rule/policy_configuration | Get Travel Rule Screening Policy Configuration
[**get_screening_policy**](ComplianceApi.md#get_screening_policy) | **GET** /screening/travel_rule/screening_policy | Travel Rule - View Screening Policy
[**get_vasp_for_vault**](ComplianceApi.md#get_vasp_for_vault) | **GET** /screening/travel_rule/vault/{vaultAccountId}/vasp | Get assigned VASP to vault
[**get_vaspby_did**](ComplianceApi.md#get_vaspby_did) | **GET** /screening/travel_rule/vasp/{did} | Get VASP details
[**get_vasps**](ComplianceApi.md#get_vasps) | **GET** /screening/travel_rule/vasp | Get All VASPs
[**retry_rejected_transaction_bypass_screening_checks**](ComplianceApi.md#retry_rejected_transaction_bypass_screening_checks) | **POST** /screening/transaction/{txId}/bypass_screening_policy | Bypass Screening Policy
[**set_vasp_for_vault**](ComplianceApi.md#set_vasp_for_vault) | **POST** /screening/travel_rule/vault/{vaultAccountId}/vasp | Assign VASP to vault
[**update_aml_screening_configuration**](ComplianceApi.md#update_aml_screening_configuration) | **PUT** /screening/aml/policy_configuration | Update AML Configuration
[**update_screening_configuration**](ComplianceApi.md#update_screening_configuration) | **PUT** /screening/configurations | Screening Configuration Update
[**update_travel_rule_config**](ComplianceApi.md#update_travel_rule_config) | **PUT** /screening/travel_rule/policy_configuration | Update Travel Rule Configuration
[**update_vasp**](ComplianceApi.md#update_vasp) | **PUT** /screening/travel_rule/vasp/update | Add jsonDidKey to VASP details
[**validate_full_travel_rule_transaction**](ComplianceApi.md#validate_full_travel_rule_transaction) | **POST** /screening/travel_rule/transaction/validate/full | Validate Full Travel Rule Transaction
[**validate_travel_rule_transaction**](ComplianceApi.md#validate_travel_rule_transaction) | **POST** /screening/travel_rule/transaction/validate | Validate Travel Rule Transaction



## get_aml_post_screening_policy

> models::ScreeningPolicyResponse get_aml_post_screening_policy()
AML - View Post-Screening Policy

Get the post-screening policy for AML. </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor, Viewer.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ScreeningPolicyResponse**](ScreeningPolicyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_aml_screening_configuration

> models::ScreeningConfigurationsRequest get_aml_screening_configuration()
Get AML Screening Policy Configuration

Retrieves the configuration for Travel Rule screening policy. </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor, Viewer.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ScreeningConfigurationsRequest**](ScreeningConfigurationsRequest.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_aml_screening_policy

> models::ScreeningProviderRulesConfigurationResponse get_aml_screening_policy()
AML - View Screening Policy

Get the screening policy for AML. </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor, Viewer.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ScreeningProviderRulesConfigurationResponse**](ScreeningProviderRulesConfigurationResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_post_screening_policy

> models::ScreeningPolicyResponse get_post_screening_policy()
Travel Rule - View Post-Screening Policy

Get the post-screening policy for Travel Rule. </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor, Viewer.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ScreeningPolicyResponse**](ScreeningPolicyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_screening_configuration

> models::ScreeningConfigurationsRequest get_screening_configuration()
Get Travel Rule Screening Policy Configuration

Retrieves the configuration for Travel Rule screening policy. </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor, Viewer.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ScreeningConfigurationsRequest**](ScreeningConfigurationsRequest.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_screening_policy

> models::ScreeningProviderRulesConfigurationResponse get_screening_policy()
Travel Rule - View Screening Policy

Get the screening policy for Travel Rule. </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor, Viewer.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ScreeningProviderRulesConfigurationResponse**](ScreeningProviderRulesConfigurationResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_vasp_for_vault

> models::TravelRuleVaspForVault get_vasp_for_vault(vault_account_id)
Get assigned VASP to vault

Get assigned VASP DID for a specific vault.  Returns empty `vaspDid` string value in response if none assigned. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vault_account_id** | **String** | The ID of the vault account | [required] |

### Return type

[**models::TravelRuleVaspForVault**](TravelRuleVaspForVault.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_vaspby_did

> models::TravelRuleVasp get_vaspby_did(did, fields)
Get VASP details

Get VASP Details. Returns information about a VASP that has the specified DID. **Note:** The reference content in this section documents the Travel Rule beta endpoint. The beta endpoint includes APIs that are currently in preview and aren't yet generally available. To enroll in the beta and enable this endpoint, contact your Fireblocks Customer Success Manager or send an email to [CSM@fireblocks.com](mailto:CSM@fireblocks.com).  </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor, Viewer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**fields** | Option<**String**> | CSV of fields to return (all, \"blank\" or see list of all field names below) |  |

### Return type

[**models::TravelRuleVasp**](TravelRuleVASP.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_vasps

> models::TravelRuleGetAllVaspsResponse get_vasps(order, per_page, page, fields)
Get All VASPs

Get All VASPs. Returns a list of VASPs. VASPs can be searched and sorted and results are paginated. **Note:** The reference content in this section documents the Travel Rule beta endpoint. The beta endpoint includes APIs that are currently in preview and aren't yet generally available. To enroll in the beta and enable this endpoint, contact your Fireblocks Customer Success Manager or send an email to [CSM@fireblocks.com](mailto:CSM@fireblocks.com).  </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor, Viewer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order** | Option<**String**> | Field to order by |  |
**per_page** | Option<**f64**> | Records per page |  |
**page** | Option<**f64**> | Page number |  |
**fields** | Option<**String**> | CSV of fields to return (all, \"blank\" or see list of all field names below) |  |

### Return type

[**models::TravelRuleGetAllVaspsResponse**](TravelRuleGetAllVASPsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retry_rejected_transaction_bypass_screening_checks

> models::CreateTransactionResponse retry_rejected_transaction_bypass_screening_checks(tx_id, idempotency_key)
Bypass Screening Policy

This endpoint is restricted to Admin API users and is only applicable to outgoing transactions. Calling the \"Bypass Screening Policy\" API endpoint triggers a new transaction, with the API user as the initiator, bypassing the screening policy check </br>Endpoint Permission: Admin and Non-Signing Admin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tx_id** | **String** | The transaction id that was rejected by screening checks | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::CreateTransactionResponse**](CreateTransactionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_vasp_for_vault

> models::TravelRuleVaspForVault set_vasp_for_vault(vault_account_id, travel_rule_vasp_for_vault, idempotency_key)
Assign VASP to vault

Sets the VASP DID for a specific vault.  Pass empty string to remove an existing one. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vault_account_id** | **String** | The ID of the vault account | [required] |
**travel_rule_vasp_for_vault** | [**TravelRuleVaspForVault**](TravelRuleVaspForVault.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::TravelRuleVaspForVault**](TravelRuleVaspForVault.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_aml_screening_configuration

> models::ScreeningConfigurationsRequest update_aml_screening_configuration(idempotency_key)
Update AML Configuration

Updates bypass screening, inbound delay, or outbound delay configurations for AML. </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor, Viewer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::ScreeningConfigurationsRequest**](ScreeningConfigurationsRequest.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_screening_configuration

> models::ScreeningUpdateConfigurations update_screening_configuration(screening_update_configurations, idempotency_key)
Screening Configuration Update

Update Tenant screening configuration. Learn more about Fireblocks AML management in the following [guide](https://developers.fireblocks.com/docs/define-aml-policies). </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor, Viewer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**screening_update_configurations** | [**ScreeningUpdateConfigurations**](ScreeningUpdateConfigurations.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::ScreeningUpdateConfigurations**](ScreeningUpdateConfigurations.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_travel_rule_config

> models::ScreeningConfigurationsRequest update_travel_rule_config(idempotency_key)
Update Travel Rule Configuration

Updates bypass screening, inbound delay, or outbound delay configurations for Travel Rule. </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor, Viewer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::ScreeningConfigurationsRequest**](ScreeningConfigurationsRequest.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_vasp

> models::TravelRuleUpdateVaspDetails update_vasp(travel_rule_update_vasp_details, idempotency_key)
Add jsonDidKey to VASP details

Update VASP Details. Updates a VASP with the provided parameters. Use this endpoint to add your public jsonDIDkey generated by Notabene. **Note:** The reference content in this section documents the Travel Rule beta endpoint. The beta endpoint includes APIs that are currently in preview and aren't yet generally available. To enroll in the beta and enable this endpoint, contact your Fireblocks Customer Success Manager or send an email to [CSM@fireblocks.com](mailto:CSM@fireblocks.com).  </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor, Viewer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**travel_rule_update_vasp_details** | [**TravelRuleUpdateVaspDetails**](TravelRuleUpdateVaspDetails.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::TravelRuleUpdateVaspDetails**](TravelRuleUpdateVASPDetails.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validate_full_travel_rule_transaction

> models::TravelRuleValidateTransactionResponse validate_full_travel_rule_transaction(travel_rule_validate_full_transaction_request, idempotency_key)
Validate Full Travel Rule Transaction

Validate Full Travel Rule transactions. Checks for all required information on the originator and beneficiary VASPs. **Note:** The reference content in this section documents the Travel Rule beta endpoint. The beta endpoint includes APIs that are currently in preview and aren't yet generally available.  To enroll in the beta and enable this endpoint, contact your Fireblocks Customer Success Manager or send an email to [CSM@fireblocks.com](mailto:CSM@fireblocks.com). Learn more about Fireblocks Travel Rule management in the following [guide](https://developers.fireblocks.com/docs/define-travel-rule-policies).  </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor, Viewer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**travel_rule_validate_full_transaction_request** | [**TravelRuleValidateFullTransactionRequest**](TravelRuleValidateFullTransactionRequest.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::TravelRuleValidateTransactionResponse**](TravelRuleValidateTransactionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validate_travel_rule_transaction

> models::TravelRuleValidateTransactionResponse validate_travel_rule_transaction(travel_rule_validate_transaction_request, idempotency_key)
Validate Travel Rule Transaction

Validate Travel Rule transactions. Checks what beneficiary VASP details are required by your jurisdiction and the beneficiary's jurisdiction. **Note:** The reference content in this section documents the Travel Rule beta endpoint. The beta endpoint includes APIs that are currently in preview and aren't yet generally available. To enroll in the beta and enable this endpoint, contact your Fireblocks Customer Success Manager or send an email to [CSM@fireblocks.com](mailto:CSM@fireblocks.com). Learn more about Fireblocks Travel Rule management in the following [guide](https://developers.fireblocks.com/docs/define-travel-rule-policies).  </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor, Viewer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**travel_rule_validate_transaction_request** | [**TravelRuleValidateTransactionRequest**](TravelRuleValidateTransactionRequest.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::TravelRuleValidateTransactionResponse**](TravelRuleValidateTransactionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

