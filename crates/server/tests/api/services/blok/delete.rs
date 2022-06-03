use crate::helpers::TestApp;
use crate::services::blok::create::create_blok;
use crate::services::page::create::create_page;
use reqwest::StatusCode;
use serde_json::json;
use test_context::test_context;

#[test_context(TestApp)]
#[tokio::test]
async fn delete_blok_works(ctx: &mut TestApp) {
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

  let response = ctx
    .delete(format!("/blok/{}", blok.get("id").expect("Expected ID")))
    .await;

  assert_eq!(StatusCode::OK, response.status());
}

#[test_context(TestApp)]
#[tokio::test]
async fn delete_blok_from_other_namespace_should_be_denied(ctx: &mut TestApp) {
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

  ctx.create_api_key("other", false).await;

  let response = ctx
    .delete(format!("/blok/{}", blok.get("id").expect("Expected ID")))
    .await;

  assert_eq!(StatusCode::NOT_FOUND, response.status());
}
