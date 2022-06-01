use crate::helpers::{spawn_app, TestApp};
use serde_json::{json, Map, Value};
use std::collections::HashSet;

async fn create_page(app: &TestApp, body: Value) -> Map<String, Value> {
  let response = app.post("/page", body).await;

  let body = response
    .json::<Value>()
    .await
    .expect("Cannot deserialize body");
  let json = body.as_object().expect("Expected response to be an object");

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
  assert_eq!(body.get("namespace"), json.get("namespace"));
  assert_eq!(body.get("path"), json.get("path"));
  assert_eq!(body.get("description"), json.get("description"));
  assert!(json.get("id").and_then(|v| v.as_i64()).is_some());
  assert!(json.get("createdAt").and_then(|v| v.as_str()).is_some());
  assert!(json.get("updatedAt").and_then(|v| v.as_str()).is_some());

  json.to_owned()
}

#[tokio::test]
async fn create_valid_page_should_work() {
  let mut app = spawn_app().await;

  app
    .create_api_key("create_get_and_delete_page_should_work", false)
    .await;

  create_page(
    &app,
    json!({
      "path": "/about/me",
      "title": "A propos !",
      "description": "Qui suis-je ? Telle est la question !"
    }),
  )
  .await;

  app.teardown().await;
}
