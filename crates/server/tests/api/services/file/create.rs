use insta::assert_json_snapshot;
use reqwest::StatusCode;
use serde_json::json;
use test_context::test_context;

use crate::test_app::TestApp;

#[test_context(TestApp)]
#[tokio::test]
async fn create_file_should_work(ctx: &mut TestApp) {
    ctx.create_api_key("test_file", false).await;

    let request = ctx
        .post(
            "/file",
            json!({
                "contentLength": 1000000,
                "fileName": "file.pdf",
                "tags": ["events"],
                "metadata": {
                    "some": "metadata"
                }
            }),
        )
        .await;

    let res = request
        .json::<serde_json::Value>()
        .await
        .expect("Failed to create file");

    dbg!(res.clone());
    assert!(res.get("id").is_some());
    assert!(res.get("uploadUrl").is_some());
    assert!(res.get("key").is_some());
    assert!(res.get("publicUrl").is_some());
    assert_json_snapshot!(res.get("tags"));
    assert_json_snapshot!(res.get("metadata"));
    assert!(res.get("createdAt").is_some());
    assert!(res.get("updatedAt").is_some());
}

#[test_context(TestApp)]
#[tokio::test]
async fn create_file_too_large_should_fail(ctx: &mut TestApp) {
    ctx.create_api_key("test_file", false).await;

    let request = ctx
        .post(
            "/file",
            json!({
                "contentLength": 1000000000,
                "fileName": "file.pdf",
                "tags": ["events"],
                "metadata": {
                    "some": "metadata"
                }
            }),
        )
        .await;

    assert_eq!(StatusCode::PAYLOAD_TOO_LARGE, request.status());

    let res = request
        .json::<serde_json::Value>()
        .await
        .expect("Failed to create file");

    assert_json_snapshot!(res);
}
