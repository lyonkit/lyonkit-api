use crate::{services::post::create_post, test_app::TestApp};
use reqwest::StatusCode;
use serde_json::{json, Value};
use std::collections::HashSet;
use test_context::test_context;

#[test_context(TestApp)]
#[tokio::test]
async fn update_valid_post_should_work(ctx: &mut TestApp) {
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
        .put(
            format!("/post/{}", post.get("id").expect("Expected ID")),
            &json!({
              "title": "My second article",
              "description": "Other article description",
              "slug": "second-article",
              "body": {
                "other": "value"
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
    assert_eq!(
        Some(&Value::String("My second article".to_string())),
        json.get("title")
    );
    assert_eq!(
        Some(&Value::String("Other article description".to_string())),
        json.get("description")
    );
    assert_eq!(Some(&json!({ "other": "value" })), json.get("body"));
    assert_eq!(post.get("createdAt"), json.get("createdAt"));
    assert_ne!(post.get("updatedAt"), json.get("updatedAt"));
}
