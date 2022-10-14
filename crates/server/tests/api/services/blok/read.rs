use crate::services::blok::create::create_blok;
use crate::services::page::create::create_page;
use crate::test_app::TestApp;
use actix_web::http::StatusCode;
use serde_json::{json, Value};
use std::collections::HashSet;
use test_context::test_context;

#[test_context(TestApp)]
#[tokio::test]
async fn get_one_blok_should_work(ctx: &mut TestApp) {
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

    let response = ctx
        .get(format!("/blok/{}", blok.get("id").expect("Expected ID")))
        .await;

    let json = response
        .json::<Value>()
        .await
        .ok()
        .and_then(|v| v.as_object().cloned())
        .expect("Expected value");

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
    assert_eq!(blok.get("componentId"), json.get("componentId"));
    assert_eq!(blok.get("props"), json.get("props"));
    assert!(json.get("id").and_then(|v| v.as_i64()).is_some());
    assert!(json.get("createdAt").and_then(|v| v.as_str()).is_some());
    assert!(json.get("updatedAt").and_then(|v| v.as_str()).is_some());
}

#[test_context(TestApp)]
#[tokio::test]
async fn get_one_blok_from_another_namespace_should_be_denied(ctx: &mut TestApp) {
    ctx.create_api_key("other", false).await;

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

    ctx.create_api_key("namespace", false).await;

    let response = ctx
        .get(format!("/blok/{}", blok.get("id").expect("Expected ID")))
        .await;

    assert_eq!(StatusCode::NOT_FOUND, response.status());

    let json = response
        .json::<Value>()
        .await
        .ok()
        .and_then(|v| v.as_object().cloned())
        .expect("Expected value");

    assert_eq!(Some("NTFND"), json.get("code").and_then(|v| v.as_str()));
}
