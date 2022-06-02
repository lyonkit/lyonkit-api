use crate::{helpers::TestApp, services::page::create::create_page};
use reqwest::StatusCode;
use serde_json::{json, Value};
use std::collections::HashSet;
use test_context::test_context;

#[test_context(TestApp)]
#[tokio::test]
async fn get_page_works_with_read_only_api_key(ctx: &mut TestApp) {
  ctx.create_api_key("namespace", false).await;
  let page = create_page(
    ctx,
    &json!({
        "path": "/my-path",
        "title": "My Path Title",
        "description": "My Path Description"
    }),
  )
  .await;
  let id = page
    .get("id")
    .and_then(|v| v.as_i64())
    .expect("Expected ID");

  ctx.create_api_key("namespace", true).await;
  let response = ctx.get(format!("/page/{}", id)).await;
  assert_eq!(StatusCode::OK, response.status());
  let json = response
    .json::<Value>()
    .await
    .ok()
    .and_then(|v| v.as_object().cloned())
    .expect("Failed to deserialize json body");

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
  assert_eq!(
    Some("namespace"),
    json.get("namespace").and_then(|v| v.as_str())
  );
  assert_eq!(Some("/my-path"), json.get("path").and_then(|v| v.as_str()));
  assert_eq!(
    Some("My Path Title"),
    json.get("title").and_then(|v| v.as_str())
  );
  assert_eq!(
    Some("My Path Description"),
    json.get("description").and_then(|v| v.as_str())
  );
  assert!(json.get("createdAt").and_then(|v| v.as_str()).is_some());
  assert!(json.get("updatedAt").and_then(|v| v.as_str()).is_some());
}

#[test_context(TestApp)]
#[tokio::test]
async fn get_page_fails_with_invalid_api_key(ctx: &mut TestApp) {
  ctx.create_api_key("namespace", false).await;

  let page = create_page(
    ctx,
    &json!({
        "path": "/my-path",
        "title": "My Path Title",
        "description": "My Path Description"
    }),
  )
  .await;
  let id = page
    .get("id")
    .and_then(|v| v.as_i64())
    .expect("Expected ID");

  ctx.set_active_api_key(None);
  let response = ctx.get(format!("/page/{}", id)).await;
  assert_eq!(StatusCode::FORBIDDEN, response.status());

  ctx.set_active_api_key(Some(String::from("INVALID_API_KEY")));
  let response = ctx.get(format!("/page/{}", id)).await;
  assert_eq!(StatusCode::FORBIDDEN, response.status());
}

#[test_context(TestApp)]
#[tokio::test]
async fn get_page_with_other_namespace_is_not_found(ctx: &mut TestApp) {
  ctx.create_api_key("namespace", false).await;
  let page = create_page(
    ctx,
    &json!({
        "path": "/my-path",
        "title": "My Path Title",
        "description": "My Path Description"
    }),
  )
  .await;
  let id = page
    .get("id")
    .and_then(|v| v.as_i64())
    .expect("Expected ID");

  ctx.create_api_key("other_namespace", false).await;
  let response = ctx.get(format!("/page/{}", id)).await;
  assert_eq!(StatusCode::NOT_FOUND, response.status());
}

#[test_context(TestApp)]
#[tokio::test]
async fn list_page_should_list_pages_in_current_namespace(ctx: &mut TestApp) {
  ctx.create_api_key("other_namespace", false).await;
  let page = create_page(
    ctx,
    &json!({
        "path": "/my-path",
        "title": "My Path Title",
        "description": "My Path Description"
    }),
  )
  .await;
  let id1_n2 = page
    .get("id")
    .and_then(|v| v.as_i64())
    .expect("Expected ID");
  ctx.create_api_key("namespace", false).await;

  let page = create_page(
    ctx,
    &json!({
        "path": "/my-path",
        "title": "My Path Title",
        "description": "My Path Description"
    }),
  )
  .await;
  let id1_n1 = page
    .get("id")
    .and_then(|v| v.as_i64())
    .expect("Expected ID");

  let page = create_page(
    ctx,
    &json!({
        "path": "/my-path-2",
        "title": "My Path Title 2",
        "description": "My Path Description 2"
    }),
  )
  .await;
  let id2_n1 = page
    .get("id")
    .and_then(|v| v.as_i64())
    .expect("Expected ID");

  let response = ctx.get("/page").await;
  assert_eq!(StatusCode::OK, response.status());
  let ids = response
    .json::<Value>()
    .await
    .ok()
    .and_then(|v| v.as_array().cloned())
    .expect("Failed to deserialize json body")
    .iter()
    .map(|v| v.as_object())
    .map(|v| v.and_then(|m| m.get("id")))
    .map(|v| v.and_then(|v| v.as_i64()))
    .map(|v| v.expect("Expected ID"))
    .collect::<Vec<i64>>();

  assert!(ids.contains(&id1_n1));
  assert!(ids.contains(&id2_n1));
  assert!(!ids.contains(&id1_n2));
}
