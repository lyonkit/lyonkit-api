use crate::services::quote::create::create_quote;
use crate::test_app::TestApp;
use reqwest::StatusCode;
use serde_json::{json, Value};
use std::collections::HashSet;
use test_context::test_context;

#[test_context(TestApp)]
#[tokio::test]
async fn update_quote_works(ctx: &mut TestApp) {
  ctx.create_api_key("namespace", false).await;

  let quote = create_quote(
    ctx,
    &json!({
        "author": "Léo Coletta",
        "message": "Source blabla"
    }),
  )
  .await;

  let id = quote
    .get("id")
    .and_then(|v| v.as_i64())
    .expect("Expected ID");

  let response = ctx
    .put(
      format!("/quote/{}", id),
      json!({
        "author": "Anaïs Coletta",
        "message": "Blabla changed"
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
      "author",
      "message",
      "namespace",
      "createdAt",
      "updatedAt"
    ]),
    json.keys().map(|v| v.as_str()).collect(),
  );
  assert!(json.get("id").and_then(|v| v.as_i64()).is_some());
  assert_eq!(quote.get("id"), json.get("id"));
  assert_eq!(
    Some("Anaïs Coletta"),
    json.get("author").and_then(|v| v.as_str())
  );
  assert_eq!(
    Some("Blabla changed"),
    json.get("message").and_then(|v| v.as_str())
  );
  assert_eq!(quote.get("createdAt"), json.get("createdAt"));
  assert!(json.get("updatedAt").and_then(|v| v.as_str()).is_some());
  assert_ne!(quote.get("updatedAt"), json.get("updatedAt"));
}

#[test_context(TestApp)]
#[tokio::test]
async fn update_quote_from_other_namespace_should_be_denied(ctx: &mut TestApp) {
  ctx.create_api_key("namespace", false).await;

  let page = create_quote(
    ctx,
    &json!({
        "author": "Léo Coletta",
        "message": "Blabla"
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
      format!("/quote/{}", id),
      json!({
        "author": "Léo Updated",
        "message": "Blabla updated"
      }),
    )
    .await;
  assert_eq!(StatusCode::NOT_FOUND, response.status());
}
