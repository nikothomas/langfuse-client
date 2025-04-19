# \ScoreApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**score_create**](ScoreApi.md#score_create) | **POST** /api/public/scores | 
[**score_delete**](ScoreApi.md#score_delete) | **DELETE** /api/public/scores/{scoreId} | 
[**score_get**](ScoreApi.md#score_get) | **GET** /api/public/scores | 
[**score_get_by_id**](ScoreApi.md#score_get_by_id) | **GET** /api/public/scores/{scoreId} | 



## score_create

> models::CreateScoreResponse score_create(create_score_request)


Create a score

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_score_request** | [**CreateScoreRequest**](CreateScoreRequest.md) |  | [required] |

### Return type

[**models::CreateScoreResponse**](CreateScoreResponse.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## score_delete

> score_delete(score_id)


Delete a score

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**score_id** | **String** | The unique langfuse identifier of a score | [required] |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## score_get

> models::GetScoresResponse score_get(page, limit, user_id, name, from_timestamp, to_timestamp, environment, source, operator, value, score_ids, config_id, queue_id, data_type, trace_tags)


Get a list of scores

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Page number, starts at 1. |  |
**limit** | Option<**i32**> | Limit of items per page. If you encounter api issues due to too large page sizes, try to reduce the limit. |  |
**user_id** | Option<**String**> | Retrieve only scores with this userId associated to the trace. |  |
**name** | Option<**String**> | Retrieve only scores with this name. |  |
**from_timestamp** | Option<**String**> | Optional filter to only include scores created on or after a certain datetime (ISO 8601) |  |
**to_timestamp** | Option<**String**> | Optional filter to only include scores created before a certain datetime (ISO 8601) |  |
**environment** | Option<[**Vec<String>**](String.md)> | Optional filter for scores where the environment is one of the provided values. |  |
**source** | Option<[**ScoreSource**](.md)> | Retrieve only scores from a specific source. |  |
**operator** | Option<**String**> | Retrieve only scores with <operator> value. |  |
**value** | Option<**f64**> | Retrieve only scores with <operator> value. |  |
**score_ids** | Option<**String**> | Comma-separated list of score IDs to limit the results to. |  |
**config_id** | Option<**String**> | Retrieve only scores with a specific configId. |  |
**queue_id** | Option<**String**> | Retrieve only scores with a specific annotation queueId. |  |
**data_type** | Option<[**ScoreDataType**](.md)> | Retrieve only scores with a specific dataType. |  |
**trace_tags** | Option<[**Vec<String>**](String.md)> | Only scores linked to traces that include all of these tags will be returned. |  |

### Return type

[**models::GetScoresResponse**](GetScoresResponse.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## score_get_by_id

> models::Score score_get_by_id(score_id)


Get a score

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**score_id** | **String** | The unique langfuse identifier of a score | [required] |

### Return type

[**models::Score**](Score.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

