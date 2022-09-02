use crate::services::quote::create::create_quote;
use crate::test_app::TestApp;
use reqwest::StatusCode;
use serde_json::{json, Value};
use test_context::test_context;

#[test_context(TestApp)]
#[tokio::test]
async fn get_quote_by_id(ctx: &mut TestApp) {
  ctx.create_api_key("other_namespace", false).await;
  let quote = create_quote(
    ctx,
    &json!({
      "author": "Blabla",
      "message": "Blabla blabla"
    }),
  )
  .await;

  let quote_id = quote
    .get("id")
    .and_then(|v| v.as_i64())
    .expect("Expected ID");

  let response = ctx.get(format!("/quote/{quote_id}")).await;

  let body = response
    .json::<serde_json::Value>()
    .await
    .expect("Cannot deserialize body");
  let id = body.as_object().expect("Expected json object").get("id");

  assert_eq!(quote.get("id"), id);
}

#[test_context(TestApp)]
#[tokio::test]
async fn list_quote_should_list_pages_in_current_namespace(ctx: &mut TestApp) {
  ctx.create_api_key("other_namespace", false).await;
  let quote = create_quote(
    ctx,
    &json!({
      "author": "Blabla",
      "message": "Blabla blabla"
    }),
  )
  .await;
  let id1_n2 = quote
    .get("id")
    .and_then(|v| v.as_i64())
    .expect("Expected ID");
  ctx.create_api_key("namespace", false).await;

  let page = create_quote(
    ctx,
    &json!({
      "author": "Valid blabla",
      "message": "Valid blabla"
    }),
  )
  .await;
  let id1_n1 = page
    .get("id")
    .and_then(|v| v.as_i64())
    .expect("Expected ID");

  let page = create_quote(
    ctx,
    &json!({
      "author": "Valid blabla2",
      "message": "Valid blabla2"
    }),
  )
  .await;
  let id2_n1 = page
    .get("id")
    .and_then(|v| v.as_i64())
    .expect("Expected ID");

  let response = ctx.get("/quote").await;
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
