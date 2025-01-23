# \JobManagementApi

All URIs are relative to *https://api.fireblocks.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel_job**](JobManagementApi.md#cancel_job) | **POST** /batch/{jobId}/cancel | Cancel a running job
[**continue_job**](JobManagementApi.md#continue_job) | **POST** /batch/{jobId}/continue | Continue a paused job
[**get_job**](JobManagementApi.md#get_job) | **GET** /batch/{jobId} | Get job details
[**get_job_tasks**](JobManagementApi.md#get_job_tasks) | **GET** /batch/{jobId}/tasks | Return a list of tasks for given job
[**get_jobs**](JobManagementApi.md#get_jobs) | **GET** /batch/jobs | Return a list of jobs belonging to tenant
[**pause_job**](JobManagementApi.md#pause_job) | **POST** /batch/{jobId}/pause | Pause a job



## cancel_job

> cancel_job(job_id, idempotency_key)
Cancel a running job

Stop the given job immediately. If the job is in the ‘Active’ state, the job will be canceled after completing the current task. Vault accounts and Wallets that are already created will not be affected.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | The requested job id | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## continue_job

> continue_job(job_id, idempotency_key)
Continue a paused job

Continue the given paused job.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | The requested job id | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_job

> models::Job get_job(job_id)
Get job details

Get an object describing the given job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | The requested job id | [required] |

### Return type

[**models::Job**](Job.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_job_tasks

> Vec<models::Task> get_job_tasks(job_id)
Return a list of tasks for given job

Return a list of tasks for given job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | The requested job id | [required] |

### Return type

[**Vec<models::Task>**](Task.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_jobs

> Vec<models::Job> get_jobs(from_time, to_time)
Return a list of jobs belonging to tenant

Get an array of objects including all active, paused, canceled, and complete jobs in a workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from_time** | Option<**i32**> | Start of time range in ms since 1970 |  |
**to_time** | Option<**i32**> | End of time range in ms since 1970 |  |

### Return type

[**Vec<models::Job>**](Job.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pause_job

> pause_job(job_id, idempotency_key)
Pause a job

Pause the given job, after the current task is done. A paused job can later be resumed by calling ‘continue’, or canceled.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | The requested job id | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

