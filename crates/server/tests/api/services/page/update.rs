use crate::{helpers::TestApp, services::page::create::create_page};
use reqwest::StatusCode;
use serde_json::{json, Value};
use std::collections::HashSet;
use test_context::test_context;

#[test_context(TestApp)]
#[tokio::test]
async fn update_page_works(ctx: &mut TestApp) {
  ctx.create_api_key("namespace", false).await;

  let page = create_page(
    ctx,
    &json!({
        "path": "/original-path",
        "title": "Original Title",
        "description": "Original Description"
    }),
  )
  .await;

  let id = page
    .get("id")
    .and_then(|v| v.as_i64())
    .expect("Expected ID");

  let response = ctx
    .put(
      format!("/page/{}", id),
      json!({
          "path": "/updated-path",
          "title": "Updated Title",
          "description": "Updated Description"
      }),
    )
    .await;

  let json = response
    .json::<Value>()
    .await
    .ok()
    .and_then(|v| v.as_object().cloned())
    .expect("Cannot deserialize body");

  assert_eq!(
    HashSet::from([
      "id",
      "namespace",
      "path",
      "title",
      "description",
      "createdAt",
      "updatedAt"
    ]),
    json.keys().map(|v| v.as_str()).collect(),
  );
  assert!(json.get("id").and_then(|v| v.as_i64()).is_some());
  assert_eq!(page.get("id"), json.get("id"));
  assert_eq!(
    Some("namespace"),
    json.get("namespace").and_then(|v| v.as_str())
  );
  assert_eq!(
    Some("/updated-path"),
    json.get("path").and_then(|v| v.as_str())
  );
  assert_eq!(
    Some("Updated Title"),
    json.get("title").and_then(|v| v.as_str())
  );
  assert_eq!(
    Some("Updated Description"),
    json.get("description").and_then(|v| v.as_str())
  );
  assert_eq!(page.get("createdAt"), json.get("createdAt"));
  assert!(json.get("updatedAt").and_then(|v| v.as_str()).is_some());
  assert_ne!(page.get("updatedAt"), json.get("updatedAt"));
}

#[test_context(TestApp)]
#[tokio::test]
async fn update_page_from_other_namespace_should_be_denied(ctx: &mut TestApp) {
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

  let response = ctx
    .put(
      format!("/page/{}", id),
      json!({
          "path": "/updated-path",
          "title": "Updated Title",
          "description": "Updated Description"
      }),
    )
    .await;
  assert_eq!(StatusCode::NOT_FOUND, response.status());
}
