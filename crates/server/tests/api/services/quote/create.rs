use crate::test_app::TestApp;
use serde_json::{json, Map, Value};
use std::collections::HashSet;
use test_context::test_context;

pub async fn create_quote(app: &TestApp, body: &Value) -> Map<String, Value> {
  let response = app.post("/quote", body).await;
  assert!(response.status().is_success());

  let json = response
    .json::<Value>()
    .await
    .expect("Cannot deserialize body");
  let json = json.as_object().expect("Expected response to be an object");

  assert_eq!(
    HashSet::from([
      "id",
      "namespace",
      "author",
      "message",
      "createdAt",
      "updatedAt"
    ]),
    json.keys().map(|v| v.as_str()).collect(),
  );
  assert_eq!(body.get("author"), json.get("author"));
  assert_eq!(body.get("message"), json.get("message"));
  assert!(json.get("id").and_then(|v| v.as_i64()).is_some());
  assert!(json.get("createdAt").and_then(|v| v.as_str()).is_some());
  assert!(json.get("updatedAt").and_then(|v| v.as_str()).is_some());

  json.to_owned()
}

#[test_context(TestApp)]
#[tokio::test]
async fn create_valid_quote_should_work(ctx: &mut TestApp) {
  ctx
    .create_api_key("create_get_and_delete_quote_should_work", false)
    .await;

  let response = create_quote(
    ctx,
    &json!({
      "author": "Léo Coletta",
      "message": "I love me !"
    }),
  )
  .await;

  assert_eq!(
    Some("create_get_and_delete_quote_should_work"),
    response.get("namespace").and_then(|v| v.as_str())
  );
}
