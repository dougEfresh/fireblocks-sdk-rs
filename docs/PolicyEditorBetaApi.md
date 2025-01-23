# \PolicyEditorBetaApi

All URIs are relative to *https://api.fireblocks.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_active_policy**](PolicyEditorBetaApi.md#get_active_policy) | **GET** /tap/active_policy | Get the active policy and its validation
[**get_draft**](PolicyEditorBetaApi.md#get_draft) | **GET** /tap/draft | Get the active draft
[**publish_draft**](PolicyEditorBetaApi.md#publish_draft) | **POST** /tap/draft | Send publish request for a certain draft id
[**publish_policy_rules**](PolicyEditorBetaApi.md#publish_policy_rules) | **POST** /tap/publish | Send publish request for a set of policy rules
[**update_draft**](PolicyEditorBetaApi.md#update_draft) | **PUT** /tap/draft | Update the draft with a new set of rules



## get_active_policy

> models::PolicyAndValidationResponse get_active_policy()
Get the active policy and its validation

Returns the active policy and its validation. </br> **Note:** These endpoints are currently in beta and might be subject to changes. If you want to participate and learn more about the Fireblocks TAP, please contact your Fireblocks Customer Success Manager or send an email to CSM@fireblocks.com. </br>Endpoint Permission: Admin, Non-Signing Admin.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::PolicyAndValidationResponse**](PolicyAndValidationResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_draft

> models::DraftReviewAndValidationResponse get_draft()
Get the active draft

Returns the active draft and its validation. </br> **Note:** These endpoints are currently in beta and might be subject to changes. If you want to participate and learn more about the Fireblocks TAP, please contact your Fireblocks Customer Success Manager or send an email to CSM@fireblocks.com. </br>Endpoint Permission: Admin, Non-Signing Admin.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::DraftReviewAndValidationResponse**](DraftReviewAndValidationResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## publish_draft

> models::PublishResult publish_draft(publish_draft_request, idempotency_key)
Send publish request for a certain draft id

Send publish request of certain draft id and returns the response. </br> **Note:** These endpoints are currently in beta and might be subject to changes. If you want to participate and learn more about the Fireblocks TAP, please contact your Fireblocks Customer Success Manager or send an email to CSM@fireblocks.com. </br>Endpoint Permission: Admin, Non-Signing Admin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**publish_draft_request** | [**PublishDraftRequest**](PublishDraftRequest.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::PublishResult**](PublishResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## publish_policy_rules

> models::PublishResult publish_policy_rules(policy_rules, idempotency_key)
Send publish request for a set of policy rules

Send publish request of set of policy rules and returns the response. </br> **Note:** These endpoints are currently in beta and might be subject to changes. If you want to participate and learn more about the Fireblocks TAP, please contact your Fireblocks Customer Success Manager or send an email to CSM@fireblocks.com. </br>Endpoint Permission: Admin, Non-Signing Admin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_rules** | [**PolicyRules**](PolicyRules.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::PublishResult**](PublishResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_draft

> models::DraftReviewAndValidationResponse update_draft(policy_rules, idempotency_key)
Update the draft with a new set of rules

Update the draft and return its validation. </br> **Note:** These endpoints are currently in beta and might be subject to changes. If you want to participate and learn more about the Fireblocks TAP, please contact your Fireblocks Customer Success Manager or send an email to CSM@fireblocks.com. Learn more about Fireblocks Transaction Authorization Policy in the following [guide](https://developers.fireblocks.com/docs/set-transaction-authorization-policy). </br>Endpoint Permission: Admin, Non-Signing Admin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_rules** | [**PolicyRules**](PolicyRules.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::DraftReviewAndValidationResponse**](DraftReviewAndValidationResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

