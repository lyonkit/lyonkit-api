use crate::test_app::TestApp;
use reqwest::StatusCode;
use server::services::files::{
    models::{FileInput, FilePayload},
    repository::FilesRepository,
};
use std::collections::HashMap;
use test_context::test_context;

#[test_context(TestApp)]
#[tokio::test]
async fn delete_file_should_work(ctx: &mut TestApp) {
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

    let response = ctx.delete(format!("/file/{id}")).await;

    assert_eq!(StatusCode::OK, response.status());

    let body = response
        .json::<serde_json::Value>()
        .await
        .expect("Failed to deserialize body");

    assert!(body.get("id").is_some());
}
