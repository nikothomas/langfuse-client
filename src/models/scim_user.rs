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
pub struct ScimUser {
    #[serde(rename = "schemas")]
    pub schemas: Vec<String>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "userName")]
    pub user_name: String,
    #[serde(rename = "name")]
    pub name: models::ScimName,
    #[serde(rename = "emails")]
    pub emails: Vec<models::ScimEmail>,
    #[serde(rename = "meta")]
    pub meta: models::UserMeta,
}

impl ScimUser {
    pub fn new(schemas: Vec<String>, id: String, user_name: String, name: models::ScimName, emails: Vec<models::ScimEmail>, meta: models::UserMeta) -> ScimUser {
        ScimUser {
            schemas,
            id,
            user_name,
            name,
            emails,
            meta,
        }
    }
}

