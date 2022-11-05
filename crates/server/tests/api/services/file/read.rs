use std::collections::HashMap;

use reqwest::StatusCode;
use sea_orm::ConnectionTrait;
use test_context::test_context;

use server::services::files::{models::FileInput, repository::FilesRepository};

use crate::test_app::TestApp;

async fn create_test_file<T: ConnectionTrait>(
    db: &T,
    namespace: &str,
    tags: Vec<&str>,
) -> entity::file::Model {
    db.create_file(
        namespace,
        FileInput {
            content_length: 10000000,
            tags: tags.iter().map(|str| str.to_string()).collect(),
            metadata: HashMap::from([("some".to_string(), "metadata".to_string())]),
            file_name: "file.txt".to_string(),
        },
    )
    .await
    .expect("Failed to create file")
}

#[test_context(TestApp)]
#[tokio::test]
async fn list_file_should_work(ctx: &mut TestApp) {
    ctx.create_api_key("test_file", true).await;

    create_test_file(ctx.database_connection(), "test_file", vec!["events"]).await;
    create_test_file(ctx.database_connection(), "test_file", vec!["events"]).await;
    create_test_file(ctx.database_connection(), "test_file", vec!["events"]).await;
    create_test_file(ctx.database_connection(), "test_file", vec!["other_tag"]).await;
    create_test_file(ctx.database_connection(), "other_namespace", vec!["events"]).await;

    let response = ctx.get("/file?tag=events").await;

    assert_eq!(StatusCode::OK, response.status());

    let json_body = response
        .json::<serde_json::Value>()
        .await
        .expect("Failed to deserialize body");
    dbg!(&json_body);

    let body = json_body.as_array().expect("Expected json list response");

    assert_eq!(3, body.len());
}
