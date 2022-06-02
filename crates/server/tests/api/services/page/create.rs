use crate::helpers::TestApp;
use reqwest::StatusCode;
use serde_json::{json, Map, Value};
use std::collections::HashSet;
use test_context::test_context;

async fn create_page(app: &TestApp, body: &Value) -> Map<String, Value> {
  let response = app.post("/page", body).await;
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
      "path",
      "title",
      "description",
      "createdAt",
      "updatedAt"
    ]),
    json.keys().map(|v| v.as_str()).collect(),
  );
  assert_eq!(body.get("path"), json.get("path"));
  assert_eq!(body.get("description"), json.get("description"));
  assert!(json.get("id").and_then(|v| v.as_i64()).is_some());
  assert!(json.get("createdAt").and_then(|v| v.as_str()).is_some());
  assert!(json.get("updatedAt").and_then(|v| v.as_str()).is_some());

  json.to_owned()
}

#[test_context(TestApp)]
#[tokio::test]
async fn create_valid_page_should_work(ctx: &mut TestApp) {
  ctx
    .create_api_key("create_get_and_delete_page_should_work", false)
    .await;

  let response = create_page(
    ctx,
    &json!({
      "path": "/about/me",
      "title": "A propos !",
      "description": "Qui suis-je ? Telle est la question !"
    }),
  )
  .await;
  assert_eq!(
    response.get("namespace"),
    Some(&Value::String(
      "create_get_and_delete_page_should_work".to_string()
    ))
  );
}

#[test_context(TestApp)]
#[tokio::test]
async fn create_page_cannot_have_duplicate_path(ctx: &mut TestApp) {
  ctx.create_api_key("write_api_key", false).await;

  create_page(
    ctx,
    &json!({
        "path": "/my-path",
        "title": "First title !",
        "description": "First description !"
    }),
  )
  .await;
  let response = ctx
    .post(
      "/page",
      json!({
        "path": "/my-path",
        "title": "Second title !",
        "description": "Second description"
      }),
    )
    .await;

  assert_eq!(StatusCode::INTERNAL_SERVER_ERROR, response.status());
  let json = response
    .json::<Value>()
    .await
    .expect("Failed to deserialize body");
  assert_eq!(Some(&Value::String("DBERR".to_string())), json.get("code"));
}

#[test_context(TestApp)]
#[tokio::test]
async fn create_page_with_invalid_api_key_should_be_denied(ctx: &mut TestApp) {
  let response = ctx
    .post(
      "/page",
      json!({
        "path": "/no-api-key",
        "title": "A page created with no API KEY !",
        "description": "This is an example of page created with no API KEY !"
      }),
    )
    .await;
  assert_eq!(StatusCode::FORBIDDEN, response.status());
  let json = response
    .json::<Value>()
    .await
    .expect("Failed to deserialize body");
  assert_eq!(Some(&Value::String("AKNPV".to_string())), json.get("code"));

  ctx.create_api_key("create_page_read_only_key", true).await;

  let response = ctx
    .post(
      "/page",
      json!({
        "path": "/read-only-api",
        "title": "A page created with RO API KEY !",
        "description": "This is an example of page created with RO API KEY !"
      }),
    )
    .await;
  assert_eq!(StatusCode::UNAUTHORIZED, response.status());
  let json = response
    .json::<Value>()
    .await
    .expect("Failed to deserialize body");
  assert_eq!(Some(&Value::String("AKIRO".to_string())), json.get("code"));

  ctx.set_active_api_key(Some(String::from("MY_CUSTOM_INVALID_API_KEY")));
  let response = ctx
    .post(
      "/page",
      json!({
        "path": "/invalid-api-key",
        "title": "A page created with invalid API KEY !",
        "description": "This is an example of page created with invalid API KEY !"
      }),
    )
    .await;
  assert_eq!(StatusCode::FORBIDDEN, response.status());
  let json = response
    .json::<Value>()
    .await
    .expect("Failed to deserialize body");
  assert_eq!(Some(&Value::String("AKINV".to_string())), json.get("code"));
}
