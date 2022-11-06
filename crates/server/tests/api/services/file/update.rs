use crate::test_app::TestApp;
use insta::assert_json_snapshot;
use reqwest::StatusCode;
use serde_json::json;
use server::services::files::{
    models::{FileInput, FilePayload},
    repository::FilesRepository,
};
use std::collections::HashMap;
use test_context::test_context;

#[test_context(TestApp)]
#[tokio::test]
async fn update_file_metadata_should_work(ctx: &mut TestApp) {
    ctx.create_api_key("test_update_file", false).await;

    let model = ctx
        .database_connection()
        .create_file(
            "test_update_file",
            FileInput {
                file: FilePayload {
                    file_name: "test_file.txt".to_string(),
                    content_type: Some("text/plain".to_string()),
                    content_length: 100000,
                },
                metadata: HashMap::from([("update".to_string(), "no".to_string())]),
                tags: vec!["events".to_string()],
            },
        )
        .await
        .expect("Failed to create test file");

    let id = model.id();

    let response = ctx
        .put(
            format!("/file/{id}"),
            json!({
                "metadata": {
                    "update": "yes"
                }
            }),
        )
        .await;

    assert_eq!(StatusCode::OK, response.status());

    let body = response
        .json::<serde_json::Value>()
        .await
        .expect("Failed to deserialize body");

    assert!(body.get("id").is_some());
    assert!(body.get("createdAt").is_some());
    assert!(body.get("updatedAt").is_some());
    assert!(body.get("publicUrl").is_some());
    assert!(body.get("key").is_some());
    assert_eq!(Some(&serde_json::Value::Null), body.get("uploadUrl"));

    let mut body = (*body.as_object().expect("Body is not a JSON object")).clone();

    // Remove unpredictable values for snapshot
    body.remove("id");
    body.remove("createdAt");
    body.remove("updatedAt");
    body.remove("publicUrl");
    body.remove("key");

    assert_json_snapshot!(body);
}

#[test_context(TestApp)]
#[tokio::test]
async fn update_file_should_work(ctx: &mut TestApp) {
    ctx.create_api_key("test_update_file", false).await;

    let model = ctx
        .database_connection()
        .create_file(
            "test_update_file",
            FileInput {
                file: FilePayload {
                    file_name: "test_file.txt".to_string(),
                    content_type: Some("text/plain".to_string()),
                    content_length: 100000,
                },
                metadata: HashMap::from([("update".to_string(), "no".to_string())]),
                tags: vec!["events".to_string()],
            },
        )
        .await
        .expect("Failed to create test file");

    let id = model.id();

    let response = ctx
        .put(
            format!("/file/{id}"),
            json!({
                "file": {
                    "contentType": "text/plain",
                    "contentLength": 10000,
                    "fileName": "file.txt"
                }
            }),
        )
        .await;

    assert_eq!(StatusCode::OK, response.status());

    let body = response
        .json::<serde_json::Value>()
        .await
        .expect("Failed to deserialize body");

    assert!(body.get("id").is_some());
    assert!(body.get("createdAt").is_some());
    assert!(body.get("updatedAt").is_some());
    assert!(body.get("publicUrl").is_some());
    assert!(body.get("key").is_some());
    assert!(body.get("uploadUrl").is_some());

    let mut body = (*body.as_object().expect("Body is not a JSON object")).clone();

    // Remove unpredictable values for snapshot
    body.remove("id");
    body.remove("createdAt");
    body.remove("updatedAt");
    body.remove("publicUrl");
    body.remove("key");
    body.remove("uploadUrl");

    assert_json_snapshot!(body);
}
