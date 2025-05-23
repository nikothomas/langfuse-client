/*
 * langfuse
 *
 * ## Authentication  Authenticate with the API using [Basic Auth](https://en.wikipedia.org/wiki/Basic_access_authentication), get API keys in the project settings:  - username: Langfuse Public Key - password: Langfuse Secret Key  ## Exports  - OpenAPI spec: https://cloud.langfuse.com/generated/api/openapi.yml - Postman collection: https://cloud.langfuse.com/generated/postman/collection.json
 *
 * The version of the OpenAPI document: 
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration, ContentType};


/// struct for typed errors of method [`annotation_queues_create_queue_item`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AnnotationQueuesCreateQueueItemError {
    Status400(serde_json::Value),
    Status401(serde_json::Value),
    Status403(serde_json::Value),
    Status404(serde_json::Value),
    Status405(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`annotation_queues_delete_queue_item`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AnnotationQueuesDeleteQueueItemError {
    Status400(serde_json::Value),
    Status401(serde_json::Value),
    Status403(serde_json::Value),
    Status404(serde_json::Value),
    Status405(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`annotation_queues_get_queue`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AnnotationQueuesGetQueueError {
    Status400(serde_json::Value),
    Status401(serde_json::Value),
    Status403(serde_json::Value),
    Status404(serde_json::Value),
    Status405(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`annotation_queues_get_queue_item`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AnnotationQueuesGetQueueItemError {
    Status400(serde_json::Value),
    Status401(serde_json::Value),
    Status403(serde_json::Value),
    Status404(serde_json::Value),
    Status405(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`annotation_queues_list_queue_items`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AnnotationQueuesListQueueItemsError {
    Status400(serde_json::Value),
    Status401(serde_json::Value),
    Status403(serde_json::Value),
    Status404(serde_json::Value),
    Status405(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`annotation_queues_list_queues`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AnnotationQueuesListQueuesError {
    Status400(serde_json::Value),
    Status401(serde_json::Value),
    Status403(serde_json::Value),
    Status404(serde_json::Value),
    Status405(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`annotation_queues_update_queue_item`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AnnotationQueuesUpdateQueueItemError {
    Status400(serde_json::Value),
    Status401(serde_json::Value),
    Status403(serde_json::Value),
    Status404(serde_json::Value),
    Status405(serde_json::Value),
    UnknownValue(serde_json::Value),
}


/// Add an item to an annotation queue
pub async fn annotation_queues_create_queue_item(configuration: &configuration::Configuration, queue_id: &str, create_annotation_queue_item_request: models::CreateAnnotationQueueItemRequest) -> Result<models::AnnotationQueueItem, Error<AnnotationQueuesCreateQueueItemError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_queue_id = queue_id;
    let p_create_annotation_queue_item_request = create_annotation_queue_item_request;

    let uri_str = format!("{}/api/public/annotation-queues/{queueId}/items", configuration.base_path, queueId=crate::apis::urlencode(p_queue_id));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref auth_conf) = configuration.basic_auth {
        req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
    };
    req_builder = req_builder.json(&p_create_annotation_queue_item_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AnnotationQueueItem`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AnnotationQueueItem`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AnnotationQueuesCreateQueueItemError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Remove an item from an annotation queue
pub async fn annotation_queues_delete_queue_item(configuration: &configuration::Configuration, queue_id: &str, item_id: &str) -> Result<models::DeleteAnnotationQueueItemResponse, Error<AnnotationQueuesDeleteQueueItemError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_queue_id = queue_id;
    let p_item_id = item_id;

    let uri_str = format!("{}/api/public/annotation-queues/{queueId}/items/{itemId}", configuration.base_path, queueId=crate::apis::urlencode(p_queue_id), itemId=crate::apis::urlencode(p_item_id));
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref auth_conf) = configuration.basic_auth {
        req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DeleteAnnotationQueueItemResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DeleteAnnotationQueueItemResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AnnotationQueuesDeleteQueueItemError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Get an annotation queue by ID
pub async fn annotation_queues_get_queue(configuration: &configuration::Configuration, queue_id: &str) -> Result<models::AnnotationQueue, Error<AnnotationQueuesGetQueueError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_queue_id = queue_id;

    let uri_str = format!("{}/api/public/annotation-queues/{queueId}", configuration.base_path, queueId=crate::apis::urlencode(p_queue_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref auth_conf) = configuration.basic_auth {
        req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AnnotationQueue`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AnnotationQueue`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AnnotationQueuesGetQueueError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Get a specific item from an annotation queue
pub async fn annotation_queues_get_queue_item(configuration: &configuration::Configuration, queue_id: &str, item_id: &str) -> Result<models::AnnotationQueueItem, Error<AnnotationQueuesGetQueueItemError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_queue_id = queue_id;
    let p_item_id = item_id;

    let uri_str = format!("{}/api/public/annotation-queues/{queueId}/items/{itemId}", configuration.base_path, queueId=crate::apis::urlencode(p_queue_id), itemId=crate::apis::urlencode(p_item_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref auth_conf) = configuration.basic_auth {
        req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AnnotationQueueItem`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AnnotationQueueItem`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AnnotationQueuesGetQueueItemError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Get items for a specific annotation queue
pub async fn annotation_queues_list_queue_items(configuration: &configuration::Configuration, queue_id: &str, status: Option<models::AnnotationQueueStatus>, page: Option<i32>, limit: Option<i32>) -> Result<models::PaginatedAnnotationQueueItems, Error<AnnotationQueuesListQueueItemsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_queue_id = queue_id;
    let p_status = status;
    let p_page = page;
    let p_limit = limit;

    let uri_str = format!("{}/api/public/annotation-queues/{queueId}/items", configuration.base_path, queueId=crate::apis::urlencode(p_queue_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_status {
        req_builder = req_builder.query(&[("status", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_page {
        req_builder = req_builder.query(&[("page", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref auth_conf) = configuration.basic_auth {
        req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::PaginatedAnnotationQueueItems`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::PaginatedAnnotationQueueItems`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AnnotationQueuesListQueueItemsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Get all annotation queues
pub async fn annotation_queues_list_queues(configuration: &configuration::Configuration, page: Option<i32>, limit: Option<i32>) -> Result<models::PaginatedAnnotationQueues, Error<AnnotationQueuesListQueuesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_page = page;
    let p_limit = limit;

    let uri_str = format!("{}/api/public/annotation-queues", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_page {
        req_builder = req_builder.query(&[("page", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref auth_conf) = configuration.basic_auth {
        req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::PaginatedAnnotationQueues`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::PaginatedAnnotationQueues`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AnnotationQueuesListQueuesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Update an annotation queue item
pub async fn annotation_queues_update_queue_item(configuration: &configuration::Configuration, queue_id: &str, item_id: &str, update_annotation_queue_item_request: models::UpdateAnnotationQueueItemRequest) -> Result<models::AnnotationQueueItem, Error<AnnotationQueuesUpdateQueueItemError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_queue_id = queue_id;
    let p_item_id = item_id;
    let p_update_annotation_queue_item_request = update_annotation_queue_item_request;

    let uri_str = format!("{}/api/public/annotation-queues/{queueId}/items/{itemId}", configuration.base_path, queueId=crate::apis::urlencode(p_queue_id), itemId=crate::apis::urlencode(p_item_id));
    let mut req_builder = configuration.client.request(reqwest::Method::PATCH, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref auth_conf) = configuration.basic_auth {
        req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
    };
    req_builder = req_builder.json(&p_update_annotation_queue_item_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AnnotationQueueItem`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AnnotationQueueItem`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AnnotationQueuesUpdateQueueItemError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

