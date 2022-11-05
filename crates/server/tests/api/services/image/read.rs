use crate::{
    services::image::{assert_image_output, create::create_image},
    test_app::TestApp,
};
use reqwest::StatusCode;
use serde_json::Value;
use test_context::test_context;

#[test_context(TestApp)]
#[tokio::test]
async fn list_image_with_read_only_api_key_should_work(ctx: &mut TestApp) {
    ctx.create_api_key("list_image_with_read_only_api_key_should_work", false)
        .await;

    create_image(
        ctx,
        "tests/fixtures/img/gray_400x400.jpg",
        "create_valid_image_should_work.jpg",
        mime::IMAGE_JPEG.as_ref(),
        Some("Image 1"),
    )
    .await;

    create_image(
        ctx,
        "tests/fixtures/img/gray_400x400.jpg",
        "create_valid_image_should_work.jpg",
        mime::IMAGE_JPEG.as_ref(),
        Some("Image 2"),
    )
    .await;

    ctx.create_api_key("list_image_with_read_only_api_key_should_work", true)
        .await;

    let response = ctx.get("/image").await;

    assert_eq!(StatusCode::OK, response.status());

    let json = response
        .json::<Value>()
        .await
        .expect("Expected response body to be valid JSON");

    let images = json.as_array().expect("Expected response to be an array");

    assert_eq!(2, images.len());
    assert_image_output(&images[0]);
    assert_image_output(&images[1]);
    assert_ne!(&images[0], &images[1]);
}
