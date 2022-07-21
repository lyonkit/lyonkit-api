use crate::services::blok::create_blok;
use crate::{services::page::create::create_page, test_app::TestApp};
use reqwest::StatusCode;
use serde_json::{json, Value};
use std::collections::HashSet;
use test_context::test_context;
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

#[test_context(TestApp)]
#[tokio::test]
async fn get_page_with_bloks_read_only_api_key_should_work(ctx: &mut TestApp) {
  ctx.create_api_key("namespace", false).await;
  let page = create_page(
    ctx,
    &json!({
        "path": "/test/my-path",
        "title": "My Path Title",
        "description": "My Path Description"
    }),
  )
  .await;

  let page_id = page
    .get("id")
    .and_then(|v| v.as_i64())
    .expect("Expected ID");

  create_blok(
    ctx,
    &json!({
        "pageId": page_id,
        "componentId": "Hero",
        "props": {
            "backgroundImage": "https://picsum.photos/200/300"
        }
    }),
  )
  .await;

  create_blok(
    ctx,
    &json!({
        "pageId": page_id,
        "componentId": "Paragraph",
        "props": {
          "spacing": 2
        }
    }),
  )
  .await;

  let page = create_page(
    ctx,
    &json!({
        "path": "/my-other-path",
        "title": "My Other Path Title",
        "description": "My Other Path Description"
    }),
  )
  .await;

  let page_id = page
    .get("id")
    .and_then(|v| v.as_i64())
    .expect("Expected ID");

  create_blok(
    ctx,
    &json!({
        "pageId": page_id,
        "componentId": "Hero",
        "props": {
          "spacing": 4
        }
    }),
  )
  .await;

  ctx.create_api_key("namespace", true).await;
  let response = ctx.get("/page/wb/test/my-path").await;

  assert_eq!(StatusCode::OK, response.status());

  let json = response
    .json::<Value>()
    .await
    .ok()
    .and_then(|v| v.as_object().cloned())
    .expect("Cannot deserialize body !");

  assert_eq!(
    HashSet::from([
      "id",
      "namespace",
      "path",
      "title",
      "description",
      "bloks",
      "createdAt",
      "updatedAt"
    ]),
    json.keys().map(|v| v.as_str()).collect(),
  );

  let json_bloks = json
    .get("bloks")
    .and_then(|v| v.as_array())
    .expect("Expected bloks array in response");
  assert_eq!(2, json_bloks.len());
  assert_eq!(
    vec![2, 1],
    json_bloks
      .iter()
      .map(|v| v
        .get("priority")
        .and_then(|v| v.as_i64())
        .expect("Expected priority"))
      .collect::<Vec<i64>>()
  );
  assert_eq!(
    vec!["Paragraph", "Hero"],
    json_bloks
      .iter()
      .map(|v| v
        .get("componentId")
        .and_then(|v| v.as_str())
        .expect("Expected componentId"))
      .collect::<Vec<&str>>()
  );
}
