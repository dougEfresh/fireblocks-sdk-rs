# \SmartTransferApi

All URIs are relative to *https://api.fireblocks.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel_ticket**](SmartTransferApi.md#cancel_ticket) | **PUT** /smart-transfers/{ticketId}/cancel | Cancel Ticket
[**create_ticket**](SmartTransferApi.md#create_ticket) | **POST** /smart-transfers | Create Ticket
[**create_ticket_term**](SmartTransferApi.md#create_ticket_term) | **POST** /smart-transfers/{ticketId}/terms | Search ticket by term ID
[**find_ticket_by_id**](SmartTransferApi.md#find_ticket_by_id) | **GET** /smart-transfers/{ticketId} | Search Tickets by ID
[**find_ticket_term_by_id**](SmartTransferApi.md#find_ticket_term_by_id) | **GET** /smart-transfers/{ticketId}/terms/{termId} | Get Smart Transfer Ticket Term
[**fulfill_ticket**](SmartTransferApi.md#fulfill_ticket) | **PUT** /smart-transfers/{ticketId}/fulfill | Fund ticket manually
[**fund_ticket_term**](SmartTransferApi.md#fund_ticket_term) | **PUT** /smart-transfers/{ticketId}/terms/{termId}/fund | Define funding source
[**get_smart_transfer_user_groups**](SmartTransferApi.md#get_smart_transfer_user_groups) | **GET** /smart-transfers/settings/user-groups | Get user group
[**manually_fund_ticket_term**](SmartTransferApi.md#manually_fund_ticket_term) | **PUT** /smart-transfers/{ticketId}/terms/{termId}/manually-fund | Manually add term transaction
[**remove_ticket_term**](SmartTransferApi.md#remove_ticket_term) | **DELETE** /smart-transfers/{ticketId}/terms/{termId} | Delete ticket term
[**search_tickets**](SmartTransferApi.md#search_tickets) | **GET** /smart-transfers | Find Ticket
[**set_external_ref_id**](SmartTransferApi.md#set_external_ref_id) | **PUT** /smart-transfers/{ticketId}/external-id | Add external reference ID
[**set_ticket_expiration**](SmartTransferApi.md#set_ticket_expiration) | **PUT** /smart-transfers/{ticketId}/expires-in | Set expiration
[**set_user_groups**](SmartTransferApi.md#set_user_groups) | **POST** /smart-transfers/settings/user-groups | Set user group
[**submit_ticket**](SmartTransferApi.md#submit_ticket) | **PUT** /smart-transfers/{ticketId}/submit | Submit ticket
[**update_ticket_term**](SmartTransferApi.md#update_ticket_term) | **PUT** /smart-transfers/{ticketId}/terms/{termId} | Update ticket term



## cancel_ticket

> models::FindTicketById200Response cancel_ticket(ticket_id)
Cancel Ticket

Cancel Smart Transfer ticket

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticket_id** | **String** |  | [required] |

### Return type

[**models::FindTicketById200Response**](findTicketById_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_ticket

> models::CreateTicket201Response create_ticket(smart_transfer_create_ticket_dto)
Create Ticket

Creates new Smart Transfer ticket

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**smart_transfer_create_ticket_dto** | [**SmartTransferCreateTicketDto**](SmartTransferCreateTicketDto.md) |  | [required] |

### Return type

[**models::CreateTicket201Response**](createTicket_201_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_ticket_term

> models::CreateTicketTerm201Response create_ticket_term(ticket_id, smart_transfer_create_ticket_term_dto)
Search ticket by term ID

Creates new smart transfer ticket term (when the ticket status is DRAFT)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticket_id** | **String** |  | [required] |
**smart_transfer_create_ticket_term_dto** | [**SmartTransferCreateTicketTermDto**](SmartTransferCreateTicketTermDto.md) |  | [required] |

### Return type

[**models::CreateTicketTerm201Response**](createTicketTerm_201_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_ticket_by_id

> models::FindTicketById200Response find_ticket_by_id(ticket_id)
Search Tickets by ID

Find Smart Transfer ticket by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticket_id** | **String** |  | [required] |

### Return type

[**models::FindTicketById200Response**](findTicketById_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_ticket_term_by_id

> models::FindTicketTermById200Response find_ticket_term_by_id(ticket_id, term_id)
Get Smart Transfer Ticket Term

Find Smart Transfer ticket term by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticket_id** | **String** |  | [required] |
**term_id** | **String** |  | [required] |

### Return type

[**models::FindTicketTermById200Response**](findTicketTermById_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fulfill_ticket

> models::FindTicketById200Response fulfill_ticket(ticket_id)
Fund ticket manually

Manually fulfill ticket, in case when all terms (legs) are funded manually

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticket_id** | **String** |  | [required] |

### Return type

[**models::FindTicketById200Response**](findTicketById_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fund_ticket_term

> fund_ticket_term(ticket_id, term_id, smart_transfer_fund_term_dto)
Define funding source

Set funding source for ticket term (in case of ASYNC tickets, this will execute transfer immediately)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticket_id** | **String** |  | [required] |
**term_id** | **String** |  | [required] |
**smart_transfer_fund_term_dto** | [**SmartTransferFundTermDto**](SmartTransferFundTermDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_smart_transfer_user_groups

> models::GetSmartTransferUserGroups200Response get_smart_transfer_user_groups()
Get user group

Get Smart Transfer user groups

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetSmartTransferUserGroups200Response**](getSmartTransferUserGroups_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## manually_fund_ticket_term

> manually_fund_ticket_term(ticket_id, term_id, smart_transfer_manually_fund_term_dto)
Manually add term transaction

Manually set ticket term transaction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticket_id** | **String** |  | [required] |
**term_id** | **String** |  | [required] |
**smart_transfer_manually_fund_term_dto** | [**SmartTransferManuallyFundTermDto**](SmartTransferManuallyFundTermDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_ticket_term

> remove_ticket_term(ticket_id, term_id)
Delete ticket term

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

> models::SearchTickets200Response search_tickets(q, statuses, network_id, created_by_me, expires_after, expires_before, r#type, external_ref_id, after, limit)
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

[**models::SearchTickets200Response**](searchTickets_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_external_ref_id

> models::FindTicketById200Response set_external_ref_id(ticket_id, smart_transfer_set_ticket_external_id_dto)
Add external reference ID

Set external id to a Smart Transfer ticket

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticket_id** | **String** |  | [required] |
**smart_transfer_set_ticket_external_id_dto** | [**SmartTransferSetTicketExternalIdDto**](SmartTransferSetTicketExternalIdDto.md) |  | [required] |

### Return type

[**models::FindTicketById200Response**](findTicketById_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_ticket_expiration

> models::FindTicketById200Response set_ticket_expiration(ticket_id, smart_transfer_set_ticket_expiration_dto)
Set expiration

Set expiration date on Smart Transfer ticket

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticket_id** | **String** |  | [required] |
**smart_transfer_set_ticket_expiration_dto** | [**SmartTransferSetTicketExpirationDto**](SmartTransferSetTicketExpirationDto.md) |  | [required] |

### Return type

[**models::FindTicketById200Response**](findTicketById_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_user_groups

> models::SetUserGroups201Response set_user_groups(smart_transfer_set_user_groups_dto)
Set user group

Set Smart Transfer user group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**smart_transfer_set_user_groups_dto** | [**SmartTransferSetUserGroupsDto**](SmartTransferSetUserGroupsDto.md) |  | [required] |

### Return type

[**models::SetUserGroups201Response**](setUserGroups_201_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submit_ticket

> models::FindTicketById200Response submit_ticket(ticket_id, smart_transfer_submit_ticket_dto)
Submit ticket

Submit Smart Transfer ticket - change status into ready for approval if auto approval is not turned on, or OPEN if auto approval is on

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticket_id** | **String** |  | [required] |
**smart_transfer_submit_ticket_dto** | [**SmartTransferSubmitTicketDto**](SmartTransferSubmitTicketDto.md) |  | [required] |

### Return type

[**models::FindTicketById200Response**](findTicketById_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_ticket_term

> models::FindTicketTermById200Response update_ticket_term(ticket_id, term_id, smart_transfer_update_ticket_term_dto)
Update ticket term

Update ticket term (when ticket status is DRAFT)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticket_id** | **String** |  | [required] |
**term_id** | **String** |  | [required] |
**smart_transfer_update_ticket_term_dto** | [**SmartTransferUpdateTicketTermDto**](SmartTransferUpdateTicketTermDto.md) |  | [required] |

### Return type

[**models::FindTicketTermById200Response**](findTicketTermById_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

