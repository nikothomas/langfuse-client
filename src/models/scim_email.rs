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
pub struct ScimEmail {
    #[serde(rename = "primary")]
    pub primary: bool,
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

impl ScimEmail {
    pub fn new(primary: bool, value: String, r#type: String) -> ScimEmail {
        ScimEmail {
            primary,
            value,
            r#type,
        }
    }
}

