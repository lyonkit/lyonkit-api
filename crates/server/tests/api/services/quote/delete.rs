use crate::{services::quote::create::create_quote, test_app::TestApp};
use reqwest::StatusCode;
use serde_json::json;
use test_context::test_context;

#[test_context(TestApp)]
#[tokio::test]
async fn delete_quote_works(ctx: &mut TestApp) {
    ctx.create_api_key("namespace", false).await;

    let page = create_quote(
        ctx,
        &json!({
          "author": "Delete",
          "message": "Please delete me !"
        }),
    )
    .await;

    let id = page
        .get("id")
        .and_then(|v| v.as_i64())
        .expect("Expected ID");
    dbg!(&id);

    let response = ctx.delete(format!("/quote/{}", id)).await;
    assert_eq!(StatusCode::OK, response.status());
}

#[test_context(TestApp)]
#[tokio::test]
async fn delete_page_from_other_namespace_should_be_denied(ctx: &mut TestApp) {
    ctx.create_api_key("namespace", false).await;

    let page = create_quote(
        ctx,
        &json!({
          "author": "Delete",
          "message": "Please delete me !"
        }),
    )
    .await;

    let id = page
        .get("id")
        .and_then(|v| v.as_i64())
        .expect("Expected ID");

    ctx.create_api_key("other_namespace", false).await;

    let response = ctx.delete(format!("/quote/{}", id)).await;
    assert_eq!(StatusCode::NOT_FOUND, response.status());
}
