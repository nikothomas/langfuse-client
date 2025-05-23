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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature="bon", derive(bon::Builder))]
pub struct DatasetItem {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "status")]
    pub status: models::DatasetStatus,
    #[serde(rename = "input", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub input: Option<Option<serde_json::Value>>,
    #[serde(rename = "expectedOutput", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub expected_output: Option<Option<serde_json::Value>>,
    #[serde(rename = "metadata", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Option<serde_json::Value>>,
    #[serde(rename = "sourceTraceId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub source_trace_id: Option<Option<String>>,
    #[serde(rename = "sourceObservationId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub source_observation_id: Option<Option<String>>,
    #[serde(rename = "datasetId")]
    pub dataset_id: String,
    #[serde(rename = "datasetName")]
    pub dataset_name: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

impl DatasetItem {
    pub fn new(id: String, status: models::DatasetStatus, dataset_id: String, dataset_name: String, created_at: String, updated_at: String) -> DatasetItem {
        DatasetItem {
            id,
            status,
            input: None,
            expected_output: None,
            metadata: None,
            source_trace_id: None,
            source_observation_id: None,
            dataset_id,
            dataset_name,
            created_at,
            updated_at,
        }
    }
}

