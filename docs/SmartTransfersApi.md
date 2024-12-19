# \SmartTransfersApi

All URIs are relative to *https://api.fireblocks.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel_ticket**](SmartTransfersApi.md#cancel_ticket) | **PUT** /smart-transfers/{ticketId}/cancel | Cancel Ticket
[**create_ticket**](SmartTransfersApi.md#create_ticket) | **POST** /smart-transfers | Create Ticket
[**create_ticket_term**](SmartTransfersApi.md#create_ticket_term) | **POST** /smart-transfers/{ticketId}/terms | Create leg (term)
[**find_ticket_by_id**](SmartTransfersApi.md#find_ticket_by_id) | **GET** /smart-transfers/{ticketId} | Search Ticket by ID
[**find_ticket_term_by_id**](SmartTransfersApi.md#find_ticket_term_by_id) | **GET** /smart-transfers/{ticketId}/terms/{termId} | Get Smart Transfer ticket term
[**fulfill_ticket**](SmartTransfersApi.md#fulfill_ticket) | **PUT** /smart-transfers/{ticketId}/fulfill | Fund ticket manually
[**fund_ticket_term**](SmartTransfersApi.md#fund_ticket_term) | **PUT** /smart-transfers/{ticketId}/terms/{termId}/fund | Define funding source
[**get_smart_transfer_user_groups**](SmartTransfersApi.md#get_smart_transfer_user_groups) | **GET** /smart-transfers/settings/user-groups | Get user group
[**manually_fund_ticket_term**](SmartTransfersApi.md#manually_fund_ticket_term) | **PUT** /smart-transfers/{ticketId}/terms/{termId}/manually-fund | Manually add term transaction
[**remove_ticket_term**](SmartTransfersApi.md#remove_ticket_term) | **DELETE** /smart-transfers/{ticketId}/terms/{termId} | Delete ticket leg (term)
[**search_tickets**](SmartTransfersApi.md#search_tickets) | **GET** /smart-transfers | Find Ticket
[**set_external_ref_id**](SmartTransfersApi.md#set_external_ref_id) | **PUT** /smart-transfers/{ticketId}/external-id | Add external ref. ID
[**set_ticket_expiration**](SmartTransfersApi.md#set_ticket_expiration) | **PUT** /smart-transfers/{ticketId}/expires-in | Set expiration
[**set_user_groups**](SmartTransfersApi.md#set_user_groups) | **POST** /smart-transfers/settings/user-groups | Set user group
[**submit_ticket**](SmartTransfersApi.md#submit_ticket) | **PUT** /smart-transfers/{ticketId}/submit | Submit ticket
[**update_ticket_term**](SmartTransfersApi.md#update_ticket_term) | **PUT** /smart-transfers/{ticketId}/terms/{termId} | Update ticket leg (term)



## cancel_ticket

> models::SmartTransferTicketResponse cancel_ticket(ticket_id, idempotency_key)
Cancel Ticket

Cancel Smart Transfer ticket

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticket_id** | **String** |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::SmartTransferTicketResponse**](SmartTransferTicketResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_ticket

> models::SmartTransferTicketResponse create_ticket(smart_transfer_create_ticket, idempotency_key)
Create Ticket

Creates new Smart Transfer ticket

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**smart_transfer_create_ticket** | [**SmartTransferCreateTicket**](SmartTransferCreateTicket.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::SmartTransferTicketResponse**](SmartTransferTicketResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_ticket_term

> models::SmartTransferTicketTermResponse create_ticket_term(ticket_id, smart_transfer_create_ticket_term, idempotency_key)
Create leg (term)

Creates new smart transfer ticket term (when the ticket status is DRAFT)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticket_id** | **String** |  | [required] |
**smart_transfer_create_ticket_term** | [**SmartTransferCreateTicketTerm**](SmartTransferCreateTicketTerm.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::SmartTransferTicketTermResponse**](SmartTransferTicketTermResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_ticket_by_id

> models::SmartTransferTicketResponse find_ticket_by_id(ticket_id)
Search Ticket by ID

Find Smart Transfer ticket by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticket_id** | **String** |  | [required] |

### Return type

[**models::SmartTransferTicketResponse**](SmartTransferTicketResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_ticket_term_by_id

> models::SmartTransferTicketTermResponse find_ticket_term_by_id(ticket_id, term_id)
Get Smart Transfer ticket term

Find a specific term of a specific Smart Transfer ticket

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticket_id** | **String** |  | [required] |
**term_id** | **String** |  | [required] |

### Return type

[**models::SmartTransferTicketTermResponse**](SmartTransferTicketTermResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fulfill_ticket

> models::SmartTransferTicketResponse fulfill_ticket(ticket_id, idempotency_key)
Fund ticket manually

Manually fulfill ticket, in case when all terms (legs) are funded manually

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticket_id** | **String** |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::SmartTransferTicketResponse**](SmartTransferTicketResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fund_ticket_term

> models::SmartTransferTicketTermResponse fund_ticket_term(ticket_id, term_id, smart_transfer_fund_term, idempotency_key)
Define funding source

Set funding source for ticket term (in case of ASYNC tickets, this will execute transfer immediately)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticket_id** | **String** |  | [required] |
**term_id** | **String** |  | [required] |
**smart_transfer_fund_term** | [**SmartTransferFundTerm**](SmartTransferFundTerm.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::SmartTransferTicketTermResponse**](SmartTransferTicketTermResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_smart_transfer_user_groups

> models::SmartTransferUserGroupsResponse get_smart_transfer_user_groups()
Get user group

Get Smart Transfer user groups

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::SmartTransferUserGroupsResponse**](SmartTransferUserGroupsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## manually_fund_ticket_term

> models::SmartTransferTicketTermResponse manually_fund_ticket_term(ticket_id, term_id, smart_transfer_manually_fund_term, idempotency_key)
Manually add term transaction

Manually set ticket term transaction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticket_id** | **String** |  | [required] |
**term_id** | **String** |  | [required] |
**smart_transfer_manually_fund_term** | [**SmartTransferManuallyFundTerm**](SmartTransferManuallyFundTerm.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::SmartTransferTicketTermResponse**](SmartTransferTicketTermResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_ticket_term

> remove_ticket_term(ticket_id, term_id)
Delete ticket leg (term)

Delete ticket term when ticket is in DRAFT status

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticket_id** | **String** |  | [required] |
**term_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_tickets

> models::SmartTransferTicketFilteredResponse search_tickets(q, statuses, network_id, created_by_me, expires_after, expires_before, r#type, external_ref_id, after, limit)
Find Ticket

Finds Smart Transfer tickets that match the submitted criteria

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | Option<**String**> | Search string - counterparty name or asset or ticketId. Optional |  |
**statuses** | Option<[**Vec<String>**](String.md)> | Ticket statuses for Smart Transfer tickets. Optional |  |[default to []]
**network_id** | Option<**String**> | NetworkId that is used in the ticket . Optional |  |
**created_by_me** | Option<**bool**> | Filter created tickets by created by self or by others. Optional |  |
**expires_after** | Option<**String**> | Lower bound of search range. Optional |  |
**expires_before** | Option<**String**> | Upper bound of search range. Optional |  |
**r#type** | Option<**String**> | Type of transfer. ASYNC executes transfers as they are funded, ATOMIC executes all terms (legs) as one atomic transfer |  |
**external_ref_id** | Option<**String**> | External ref. ID that workspace can use to identify ticket outside of Fireblocks system. |  |
**after** | Option<**String**> | ID of the record after which to fetch $limit records |  |
**limit** | Option<**f64**> | Number of records to fetch. By default, it is 100 |  |

### Return type

[**models::SmartTransferTicketFilteredResponse**](SmartTransferTicketFilteredResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_external_ref_id

> models::SmartTransferTicketResponse set_external_ref_id(ticket_id, smart_transfer_set_ticket_external_id, idempotency_key)
Add external ref. ID

Set external id Smart Transfer ticket

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticket_id** | **String** |  | [required] |
**smart_transfer_set_ticket_external_id** | [**SmartTransferSetTicketExternalId**](SmartTransferSetTicketExternalId.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::SmartTransferTicketResponse**](SmartTransferTicketResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_ticket_expiration

> models::SmartTransferTicketResponse set_ticket_expiration(ticket_id, smart_transfer_set_ticket_expiration, idempotency_key)
Set expiration

Set expiration date on Smart Transfer ticket

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticket_id** | **String** |  | [required] |
**smart_transfer_set_ticket_expiration** | [**SmartTransferSetTicketExpiration**](SmartTransferSetTicketExpiration.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::SmartTransferTicketResponse**](SmartTransferTicketResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_user_groups

> models::SmartTransferUserGroupsResponse set_user_groups(smart_transfer_set_user_groups, idempotency_key)
Set user group

Set Smart Transfers user group to receive email notifications for Smart Transfers

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**smart_transfer_set_user_groups** | [**SmartTransferSetUserGroups**](SmartTransferSetUserGroups.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::SmartTransferUserGroupsResponse**](SmartTransferUserGroupsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submit_ticket

> models::SmartTransferTicketResponse submit_ticket(ticket_id, smart_transfer_submit_ticket, idempotency_key)
Submit ticket

Submit Smart Transfer ticket - change status into ready for approval if auto approval is not turned on, or OPEN if auto approval is on

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticket_id** | **String** |  | [required] |
**smart_transfer_submit_ticket** | [**SmartTransferSubmitTicket**](SmartTransferSubmitTicket.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::SmartTransferTicketResponse**](SmartTransferTicketResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_ticket_term

> models::SmartTransferTicketTermResponse update_ticket_term(ticket_id, term_id, smart_transfer_update_ticket_term, idempotency_key)
Update ticket leg (term)

Update ticket term (when ticket status is DRAFT)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticket_id** | **String** |  | [required] |
**term_id** | **String** |  | [required] |
**smart_transfer_update_ticket_term** | [**SmartTransferUpdateTicketTerm**](SmartTransferUpdateTicketTerm.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::SmartTransferTicketTermResponse**](SmartTransferTicketTermResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

