# CreateChatPromptRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**prompt** | [**Vec<models::ChatMessage>**](ChatMessage.md) |  | 
**config** | Option<[**serde_json::Value**](.md)> |  | [optional]
**labels** | Option<**Vec<String>**> | List of deployment labels of this prompt version. | [optional]
**tags** | Option<**Vec<String>**> | List of tags to apply to all versions of this prompt. | [optional]
**commit_message** | Option<**String**> | Commit message for this prompt version. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


