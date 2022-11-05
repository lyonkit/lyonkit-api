use crate::{services::post::create_post, test_app::TestApp};
use actix_web::http::StatusCode;
use serde_json::{json, Value};
use std::collections::HashSet;
use test_context::test_context;

#[test_context(TestApp)]
#[tokio::test]
async fn get_one_post_should_work(ctx: &mut TestApp) {
    ctx.create_api_key("namespace", false).await;

    let post = create_post(
        ctx,
        &json!({
          "title": "My first article",
          "description": "Article description",
          "slug": "first-article",
          "body": {
            "...": "..."
          }
        }),
    )
    .await;

    let response = ctx
        .get(format!("/post/{}", post.get("id").expect("Expected ID")))
        .await;

    assert_eq!(StatusCode::OK, response.status());

    let json = response
        .json::<Value>()
        .await
        .ok()
        .and_then(|v| v.as_object().cloned())
        .expect("Expected value");

    assert_eq!(
        HashSet::from([
            "id",
            "namespace",
            "title",
            "description",
            "slug",
            "body",
            "createdAt",
            "updatedAt"
        ]),
        json.keys().map(|v| v.as_str()).collect(),
    );
    assert_eq!(post.get("id"), json.get("id"));
    assert_eq!(post.get("namespace"), json.get("namespace"));
    assert_eq!(post.get("title"), json.get("title"));
    assert_eq!(post.get("description"), json.get("description"));
    assert_eq!(post.get("slug"), json.get("slug"));
    assert_eq!(post.get("json"), json.get("json"));
    assert!(json.get("id").and_then(|v| v.as_i64()).is_some());
    assert!(json.get("createdAt").and_then(|v| v.as_str()).is_some());
    assert!(json.get("updatedAt").and_then(|v| v.as_str()).is_some());
}

#[test_context(TestApp)]
#[tokio::test]
async fn get_one_post_from_slug_should_work(ctx: &mut TestApp) {
    ctx.create_api_key("namespace", false).await;

    let post = create_post(
        ctx,
        &json!({
          "title": "My first article",
          "description": "Article description",
          "slug": "slug-article",
          "body": {
            "...": "..."
          }
        }),
    )
    .await;

    let response = ctx.get("/post/s/slug-article").await;

    assert_eq!(StatusCode::OK, response.status());

    let json = response
        .json::<Value>()
        .await
        .ok()
        .and_then(|v| v.as_object().cloned())
        .expect("Expected value");

    assert_eq!(
        HashSet::from([
            "id",
            "namespace",
            "title",
            "description",
            "slug",
            "body",
            "createdAt",
            "updatedAt"
        ]),
        json.keys().map(|v| v.as_str()).collect(),
    );
    assert_eq!(post.get("id"), json.get("id"));
    assert_eq!(post.get("namespace"), json.get("namespace"));
    assert_eq!(post.get("title"), json.get("title"));
    assert_eq!(post.get("description"), json.get("description"));
    assert_eq!(post.get("slug"), json.get("slug"));
    assert_eq!(post.get("json"), json.get("json"));
    assert!(json.get("id").and_then(|v| v.as_i64()).is_some());
    assert!(json.get("createdAt").and_then(|v| v.as_str()).is_some());
    assert!(json.get("updatedAt").and_then(|v| v.as_str()).is_some());
}

#[test_context(TestApp)]
#[tokio::test]
async fn get_one_post_from_another_namespace_should_be_denied(ctx: &mut TestApp) {
    ctx.create_api_key("other", false).await;

    let post = create_post(
        ctx,
        &json!({
          "title": "My first article",
          "description": "Article description",
          "slug": "first-article",
          "body": {
            "...": "..."
          }
        }),
    )
    .await;

    ctx.create_api_key("namespace", false).await;

    let response = ctx
        .get(format!("/post/{}", post.get("id").expect("Expected ID")))
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
