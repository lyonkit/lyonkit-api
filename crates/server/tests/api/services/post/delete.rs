use crate::helpers::TestApp;
use crate::services::post::create_post;
use reqwest::StatusCode;
use serde_json::json;
use test_context::test_context;

#[test_context(TestApp)]
#[tokio::test]
async fn delete_post_works(ctx: &mut TestApp) {
  ctx.create_api_key("namespace", false).await;

  let post = create_post(
    ctx,
    &json!({
      "title": "My first article",
      "description": "Article description",
      "body": {
        "...": "..."
      }
    }),
  )
  .await;

  let response = ctx
    .delete(format!("/post/{}", post.get("id").expect("Expected ID")))
    .await;

  assert_eq!(StatusCode::OK, response.status());
}

#[test_context(TestApp)]
#[tokio::test]
async fn delete_post_from_other_namespace_should_be_denied(ctx: &mut TestApp) {
  ctx.create_api_key("namespace", false).await;

  let post = create_post(
    ctx,
    &json!({
      "title": "My first article",
      "description": "Article description",
      "body": {
        "...": "..."
      }
    }),
  )
  .await;

  ctx.create_api_key("other", false).await;

  let response = ctx
    .delete(format!("/post/{}", post.get("id").expect("Expected ID")))
    .await;

  assert_eq!(StatusCode::NOT_FOUND, response.status());
}
