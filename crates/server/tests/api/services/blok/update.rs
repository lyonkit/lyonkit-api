use crate::{
    services::{blok::create::create_blok, page::create::create_page},
    test_app::TestApp,
};
use reqwest::StatusCode;
use serde_json::{json, Value};
use std::collections::HashSet;
use test_context::test_context;

#[test_context(TestApp)]
#[tokio::test]
async fn update_valid_blok_should_work(ctx: &mut TestApp) {
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

    let response = ctx
        .put(
            format!("/blok/{}", blok.get("id").expect("Expected ID")),
            &json!({
              "pageId": page_id,
              "componentId": "NewHero",
              "props": {
                "size": 3
              }
            }),
        )
        .await;

    assert_eq!(StatusCode::OK, response.status());

    let json = response
        .json::<Value>()
        .await
        .ok()
        .and_then(|v| v.as_object().cloned())
        .expect("Cannot deserialize body");

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
    assert_eq!(page.get("id"), json.get("pageId"));
    assert_eq!(
        Some(&Value::String("NewHero".to_string())),
        json.get("componentId")
    );
    assert_eq!(Some(&json!({"size": 3})), json.get("props"));
    assert_eq!(blok.get("id"), json.get("id"));
    assert_eq!(blok.get("createdAt"), json.get("createdAt"));
    assert_ne!(blok.get("updatedAt"), json.get("updatedAt"));
}

#[test_context(TestApp)]
#[tokio::test]
async fn update_blok_with_page_from_different_namespace_shoulod_be_denied(ctx: &mut TestApp) {
    ctx.create_api_key("other", false).await;

    let other_page = create_page(
        ctx,
        &json!({
          "path": "/about/me",
          "title": "A propos !",
          "description": "Qui suis-je ? Telle est la question !"
        }),
    )
    .await;
    ctx.create_api_key("namespace", false).await;

    let other_page_id = other_page
        .get("id")
        .and_then(|v| v.as_i64())
        .expect("Expected ID");

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

    let response = ctx
        .put(
            format!("/blok/{}", blok.get("id").expect("Expected ID")),
            &json!({
              "pageId": other_page_id,
              "componentId": "NewHero",
              "props": {
                "size": 3
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
        .expect("Cannot deserialize body");

    assert_eq!(Some(&Value::String("REFNF".to_string())), json.get("code"));
}
