# \MetricsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**metrics_daily**](MetricsApi.md#metrics_daily) | **GET** /api/public/metrics/daily | 



## metrics_daily

> models::DailyMetrics metrics_daily(page, limit, trace_name, user_id, tags, environment, from_timestamp, to_timestamp)


Get daily metrics of the Langfuse project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | page number, starts at 1 |  |
**limit** | Option<**i32**> | limit of items per page |  |
**trace_name** | Option<**String**> | Optional filter by the name of the trace |  |
**user_id** | Option<**String**> | Optional filter by the userId associated with the trace |  |
**tags** | Option<[**Vec<String>**](String.md)> | Optional filter for metrics where traces include all of these tags |  |
**environment** | Option<[**Vec<String>**](String.md)> | Optional filter for metrics where events include any of these environments |  |
**from_timestamp** | Option<**String**> | Optional filter to only include traces and observations on or after a certain datetime (ISO 8601) |  |
**to_timestamp** | Option<**String**> | Optional filter to only include traces and observations before a certain datetime (ISO 8601) |  |

### Return type

[**models::DailyMetrics**](DailyMetrics.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

