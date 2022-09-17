use crate::{services::page::create::create_page, test_app::TestApp};
use reqwest::StatusCode;
use serde_json::{json, Map, Number, Value};
use std::collections::HashSet;
use test_context::test_context;

pub async fn create_blok(app: &TestApp, body: &Value) -> Map<String, Value> {
  let response = app.post("/blok", body).await;
  assert_eq!(StatusCode::OK, response.status());

  let json = response
    .json::<Value>()
    .await
    .expect("Cannot deserialize body");
  let json = json.as_object().expect("Expected response to be an object");

  assert_eq!(
    HashSet::from([
      "id",
      "pageId",
      "componentId",
      "props",
      "priority",
      "createdAt",
      "updatedAt"
    ]),
    json.keys().map(|v| v.as_str()).collect(),
  );
  assert_eq!(body.get("pageId"), json.get("pageId"));
  assert_eq!(body.get("componentId"), json.get("componentId"));
  assert_eq!(body.get("props"), json.get("props"));
  assert!(json.get("id").and_then(|v| v.as_i64()).is_some());
  assert!(json.get("createdAt").and_then(|v| v.as_str()).is_some());
  assert!(json.get("updatedAt").and_then(|v| v.as_str()).is_some());

  json.to_owned()
}

#[test_context(TestApp)]
#[tokio::test]
async fn create_valid_blok_should_work(ctx: &mut TestApp) {
  ctx
    .create_api_key("create_valid_blok_should_work", false)
    .await;

  let page = create_page(
    ctx,
    &json!({
      "path": "/about/me",
      "title": "A propos !",
      "description": "Qui suis-je ? Telle est la question !"
    }),
  )
  .await;

  let blok = create_blok(
    ctx,
    &json!({
        "pageId": page.get("id").and_then(|v| v.as_i64()).expect("Expected ID"),
        "componentId": "Hero",
        "props": {
            "backgroundImage": "https://picsum.photos/200/300"
        }
    }),
  )
  .await;

  assert_eq!(blok.get("priority"), Some(&Value::Number(Number::from(0))));
}

#[test_context(TestApp)]
#[tokio::test]
async fn create_blok_with_page_from_other_namespace_should_be_denied(ctx: &mut TestApp) {
  ctx.create_api_key("other_namespace", false).await;

  let page = create_page(
    ctx,
    &json!({
      "path": "/about/me",
      "title": "A propos !",
      "description": "Qui suis-je ? Telle est la question !"
    }),
  )
  .await;

  ctx.create_api_key("namespace", false).await;

  let response = ctx
    .post(
      "/blok",
      &json!({
          "pageId": page.get("id").and_then(|v| v.as_i64()).expect("Expected ID"),
          "componentId": "Hero",
          "props": {
              "backgroundImage": "https://picsum.photos/200/300"
          }
      }),
    )
    .await;

  assert_eq!(StatusCode::BAD_REQUEST, response.status());

  let json = response
    .json::<Value>()
    .await
    .ok()
    .and_then(|v| v.as_object().cloned())
    .expect("Failed to deserialize body");

  assert_eq!(Some(&Value::String("REFNF".to_string())), json.get("code"));
}

#[test_context(TestApp)]
#[tokio::test]
async fn create_blok_with_non_existant_page_should_be_denied(ctx: &mut TestApp) {
  ctx.create_api_key("namespace", false).await;

  let response = ctx
    .post(
      "/blok",
      &json!({
          "pageId": 3,
          "componentId": "Hero",
          "props": {
              "backgroundImage": "https://picsum.photos/200/300"
          }
      }),
    )
    .await;

  assert_eq!(StatusCode::BAD_REQUEST, response.status());

  let json = response
    .json::<Value>()
    .await
    .ok()
    .and_then(|v| v.as_object().cloned())
    .expect("Failed to deserialize body");

  assert_eq!(Some(&Value::String("REFNF".to_string())), json.get("code"));
}
