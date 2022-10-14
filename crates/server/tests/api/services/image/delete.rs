use crate::services::image::create::create_image;
use crate::test_app::TestApp;
use reqwest::StatusCode;
use serde_json::Value;
use test_context::test_context;

#[test_context(TestApp)]
#[tokio::test]
async fn delete_image_should_work(ctx: &mut TestApp) {
    ctx.create_api_key("tests", false).await;

    let response = create_image(
        ctx,
        "tests/fixtures/img/gray_400x400.jpg",
        "delete_image.jpg",
        mime::IMAGE_JPEG.as_ref(),
        Some("Delete"),
    )
    .await;

    let json = response
        .json::<Value>()
        .await
        .expect("Expected response body to be valid JSON");

    let id = json
        .get("id")
        .and_then(|v| v.as_i64())
        .expect("Expected ID");

    let response = ctx.delete(format!("/image/{}", id)).await;

    assert_eq!(StatusCode::OK, response.status());
}
