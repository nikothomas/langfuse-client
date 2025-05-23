/*
 * langfuse
 *
 * ## Authentication  Authenticate with the API using [Basic Auth](https://en.wikipedia.org/wiki/Basic_access_authentication), get API keys in the project settings:  - username: Langfuse Public Key - password: Langfuse Secret Key  ## Exports  - OpenAPI spec: https://cloud.langfuse.com/generated/api/openapi.yml - Postman collection: https://cloud.langfuse.com/generated/postman/collection.json
 *
 * The version of the OpenAPI document: 
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// OpenAiResponseUsageSchema : OpenAI Usage schema from Response API
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature="bon", derive(bon::Builder))]
pub struct OpenAiResponseUsageSchema {
    #[serde(rename = "input_tokens")]
    pub input_tokens: i32,
    #[serde(rename = "output_tokens")]
    pub output_tokens: i32,
    #[serde(rename = "total_tokens")]
    pub total_tokens: i32,
    #[serde(rename = "input_tokens_details", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub input_tokens_details: Option<Option<std::collections::HashMap<String, i32>>>,
    #[serde(rename = "output_tokens_details", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub output_tokens_details: Option<Option<std::collections::HashMap<String, i32>>>,
}

impl OpenAiResponseUsageSchema {
    /// OpenAI Usage schema from Response API
    pub fn new(input_tokens: i32, output_tokens: i32, total_tokens: i32) -> OpenAiResponseUsageSchema {
        OpenAiResponseUsageSchema {
            input_tokens,
            output_tokens,
            total_tokens,
            input_tokens_details: None,
            output_tokens_details: None,
        }
    }
}

