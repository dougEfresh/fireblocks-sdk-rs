# \PolicyEditorBetaApi

All URIs are relative to *https://api.fireblocks.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**tap_active_policy_get**](PolicyEditorBetaApi.md#tap_active_policy_get) | **GET** /tap/active_policy | Get the active policy and its validation
[**tap_draft_get**](PolicyEditorBetaApi.md#tap_draft_get) | **GET** /tap/draft | Get the active draft
[**tap_draft_post**](PolicyEditorBetaApi.md#tap_draft_post) | **POST** /tap/draft | Send publish request for a certain draft id
[**tap_draft_put**](PolicyEditorBetaApi.md#tap_draft_put) | **PUT** /tap/draft | Update the draft with a new set of rules
[**tap_publish_post**](PolicyEditorBetaApi.md#tap_publish_post) | **POST** /tap/publish | Send publish request for a set of policy rules



## tap_active_policy_get

> models::PolicyAndValidationResponse tap_active_policy_get()
Get the active policy and its validation

Returns the active policy and its validation. </br> **Note:** These endpoints are currently in beta and might be subject to changes. If you want to participate and learn more about the Fireblocks TAP, please contact your Fireblocks Customer Success Manager or send an email to CSM@fireblocks.com. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::PolicyAndValidationResponse**](PolicyAndValidationResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tap_draft_get

> models::DraftReviewAndValidationResponse tap_draft_get()
Get the active draft

Returns the active draft and its validation. </br> **Note:** These endpoints are currently in beta and might be subject to changes. If you want to participate and learn more about the Fireblocks TAP, please contact your Fireblocks Customer Success Manager or send an email to CSM@fireblocks.com. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::DraftReviewAndValidationResponse**](DraftReviewAndValidationResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tap_draft_post

> models::PublishResult tap_draft_post(tap_draft_post_request)
Send publish request for a certain draft id

Send publish request of certain draft id and returns the response. </br> **Note:** These endpoints are currently in beta and might be subject to changes. If you want to participate and learn more about the Fireblocks TAP, please contact your Fireblocks Customer Success Manager or send an email to CSM@fireblocks.com. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tap_draft_post_request** | [**TapDraftPostRequest**](TapDraftPostRequest.md) |  | [required] |

### Return type

[**models::PublishResult**](PublishResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tap_draft_put

> models::DraftReviewAndValidationResponse tap_draft_put(tap_draft_put_request)
Update the draft with a new set of rules

Update the draft and return its validation. </br> **Note:** These endpoints are currently in beta and might be subject to changes. If you want to participate and learn more about the Fireblocks TAP, please contact your Fireblocks Customer Success Manager or send an email to CSM@fireblocks.com. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tap_draft_put_request** | [**TapDraftPutRequest**](TapDraftPutRequest.md) |  | [required] |

### Return type

[**models::DraftReviewAndValidationResponse**](DraftReviewAndValidationResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tap_publish_post

> models::PublishResult tap_publish_post(tap_publish_post_request)
Send publish request for a set of policy rules

Send publish request of set of policy rules and returns the response. </br> **Note:** These endpoints are currently in beta and might be subject to changes. If you want to participate and learn more about the Fireblocks TAP, please contact your Fireblocks Customer Success Manager or send an email to CSM@fireblocks.com. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tap_publish_post_request** | [**TapPublishPostRequest**](TapPublishPostRequest.md) |  | [required] |

### Return type

[**models::PublishResult**](PublishResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

