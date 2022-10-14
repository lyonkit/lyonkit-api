use crate::test_app::TestApp;
use reqwest::StatusCode;
use serde_json::{json, Map, Value};
use std::collections::HashSet;
use test_context::test_context;

pub async fn create_post(app: &TestApp, body: &Value) -> Map<String, Value> {
    let response = app.post("/post", body).await;
    assert_eq!(StatusCode::OK, response.status());

    let json = response
        .json::<Value>()
        .await
        .expect("Cannot deserialize body");
    let json = json.as_object().expect("Expected response to be an object");

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
    assert_eq!(body.get("title"), json.get("title"));
    assert_eq!(body.get("description"), json.get("description"));
    assert_eq!(body.get("slug"), json.get("slug"));
    assert_eq!(body.get("body"), json.get("body"));
    assert!(json.get("id").and_then(|v| v.as_i64()).is_some());
    assert!(json.get("createdAt").and_then(|v| v.as_str()).is_some());
    assert!(json.get("updatedAt").and_then(|v| v.as_str()).is_some());

    json.to_owned()
}

#[test_context(TestApp)]
#[tokio::test]
async fn create_valid_post_should_work(ctx: &mut TestApp) {
    ctx.create_api_key("create_valid_post_should_work", false)
        .await;

    create_post(
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
}
