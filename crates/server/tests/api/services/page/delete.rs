use crate::{services::page::create::create_page, test_app::TestApp};
use reqwest::StatusCode;
use serde_json::json;
use test_context::test_context;

#[test_context(TestApp)]
#[tokio::test]
async fn delete_page_works(ctx: &mut TestApp) {
    ctx.create_api_key("namespace", false).await;

    let page = create_page(
        ctx,
        &json!({
            "path": "/path",
            "title": "My Title",
            "description": "My Description"
        }),
    )
    .await;

    let id = page
        .get("id")
        .and_then(|v| v.as_i64())
        .expect("Expected ID");

    let response = ctx.delete(format!("/page/{}", id)).await;
    assert_eq!(StatusCode::OK, response.status());
}

#[test_context(TestApp)]
#[tokio::test]
async fn delete_page_from_other_namespace_should_be_denied(ctx: &mut TestApp) {
    ctx.create_api_key("namespace", false).await;

    let page = create_page(
        ctx,
        &json!({
            "path": "/path",
            "title": "My Title",
            "description": "My Description"
        }),
    )
    .await;

    let id = page
        .get("id")
        .and_then(|v| v.as_i64())
        .expect("Expected ID");

    ctx.create_api_key("other_namespace", false).await;

    let response = ctx.delete(format!("/page/{}", id)).await;
    assert_eq!(StatusCode::NOT_FOUND, response.status());
}
