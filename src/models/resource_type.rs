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
pub struct ResourceType {
    #[serde(rename = "schemas", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub schemas: Option<Option<Vec<String>>>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "endpoint")]
    pub endpoint: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "schema")]
    pub schema: String,
    #[serde(rename = "schemaExtensions")]
    pub schema_extensions: Vec<models::SchemaExtension>,
    #[serde(rename = "meta")]
    pub meta: models::ResourceMeta,
}

impl ResourceType {
    pub fn new(id: String, name: String, endpoint: String, description: String, schema: String, schema_extensions: Vec<models::SchemaExtension>, meta: models::ResourceMeta) -> ResourceType {
        ResourceType {
            schemas: None,
            id,
            name,
            endpoint,
            description,
            schema,
            schema_extensions,
            meta,
        }
    }
}

