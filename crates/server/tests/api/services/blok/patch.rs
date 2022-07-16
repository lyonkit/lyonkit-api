use crate::helpers::TestApp;
use crate::services::blok::create_blok;
use crate::services::page::create::create_page;
use reqwest::StatusCode;
use serde_json::{json, Value};
use test_context::test_context;

#[test_context(TestApp)]
#[tokio::test]
async fn empty_patch_should_return_pthof_error(ctx: &mut TestApp) {
  ctx.create_api_key("namespace", false).await;

  let page = create_page(
    ctx,
    &json!({
      "path": "/about/me",
      "title": "A propos !",
      "description": "Qui suis-je ? Telle est la question !"
    }),
  )
  .await;

  let page_id = page
    .get("id")
    .and_then(|v| v.as_i64())
    .expect("Expected ID");

  let blok = create_blok(
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
  let blok_id = blok.get("id").expect("Expected ID");

  let response = ctx.patch(format!("/blok/{blok_id}"), &json!({})).await;

  assert_eq!(StatusCode::BAD_REQUEST, response.status());

  let response_body = response
    .json::<Value>()
    .await
    .ok()
    .and_then(|j| j.as_object().cloned())
    .expect("Response body is not valid JSON Object");

  dbg!(&response_body);

  assert_eq!(
    Some(&Value::String("PTHOF".to_string())),
    response_body.get("code")
  );
}

#[test_context(TestApp)]
#[tokio::test]
async fn one_field_patch_should_work(ctx: &mut TestApp) {
  ctx.create_api_key("namespace", false).await;

  let page = create_page(
    ctx,
    &json!({
      "path": "/about/me",
      "title": "A propos !",
      "description": "Qui suis-je ? Telle est la question !"
    }),
  )
  .await;

  let page_id = page
    .get("id")
    .and_then(|v| v.as_i64())
    .expect("Expected ID");

  let blok = create_blok(
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
  let blok_id = blok.get("id").expect("Expected ID");

  let response = ctx
    .patch(
      format!("/blok/{blok_id}"),
      &json!({
        "componentId": "MarketingImage"
      }),
    )
    .await;

  assert_eq!(StatusCode::OK, response.status());

  let response_body = response
    .json::<Value>()
    .await
    .ok()
    .and_then(|j| j.as_object().cloned())
    .expect("Response body is not valid JSON Object");

  dbg!(&response_body);

  assert_eq!(blok.get("id"), response_body.get("id"));

  assert_eq!(blok.get("pageId"), response_body.get("pageId"));

  assert_eq!(
    Some(&Value::String("MarketingImage".to_string())),
    response_body.get("componentId")
  );

  assert_eq!(blok.get("props"), response_body.get("props"));

  assert_eq!(blok.get("priority"), response_body.get("priority"));

  assert_eq!(blok.get("createdAt"), response_body.get("createdAt"));

  assert_ne!(blok.get("updatedAt"), response_body.get("updatedAt"));
}
