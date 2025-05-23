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

/// UsageByModel : Daily usage of a given model. Usage corresponds to the unit set for the specific model (e.g. tokens).
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature="bon", derive(bon::Builder))]
pub struct UsageByModel {
    #[serde(rename = "model", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub model: Option<Option<String>>,
    /// Total number of generation input units (e.g. tokens)
    #[serde(rename = "inputUsage")]
    pub input_usage: i32,
    /// Total number of generation output units (e.g. tokens)
    #[serde(rename = "outputUsage")]
    pub output_usage: i32,
    /// Total number of generation total units (e.g. tokens)
    #[serde(rename = "totalUsage")]
    pub total_usage: i32,
    #[serde(rename = "countTraces")]
    pub count_traces: i32,
    #[serde(rename = "countObservations")]
    pub count_observations: i32,
    /// Total model cost in USD
    #[serde(rename = "totalCost")]
    pub total_cost: f64,
}

impl UsageByModel {
    /// Daily usage of a given model. Usage corresponds to the unit set for the specific model (e.g. tokens).
    pub fn new(input_usage: i32, output_usage: i32, total_usage: i32, count_traces: i32, count_observations: i32, total_cost: f64) -> UsageByModel {
        UsageByModel {
            model: None,
            input_usage,
            output_usage,
            total_usage,
            count_traces,
            count_observations,
            total_cost,
        }
    }
}

