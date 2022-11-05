use crate::{services, services::image::assert_image_output, test_app::TestApp};
use reqwest::{multipart, Response, StatusCode};
use serde_json::Value;
use test_context::test_context;
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

pub async fn create_image(
    app: &TestApp,
    file_path: &str,
    filename: &'static str,
    mime: &str,
    alt: Option<&str>,
) -> Response {
    let query = match alt {
        Some(a) => format!("?alt={}", a),
        None => "".to_string(),
    };

    let file_handle = File::open(file_path)
        .await
        .unwrap_or_else(|_| panic!("Failed to open image at {}", file_path));

    let bytes_stream = FramedRead::new(file_handle, BytesCodec::new());

    let form = multipart::Form::new().part(
        "image",
        multipart::Part::stream(reqwest::Body::wrap_stream(bytes_stream))
            .file_name(filename)
            .mime_str(mime)
            .expect("Invalid mime for creating an image"),
    );
    let response = app.post_multipart(format!("/image{}", query), form).await;
    assert_eq!(StatusCode::OK, response.status());
    response
}

#[test_context(TestApp)]
#[tokio::test]
async fn create_valid_image_should_work(ctx: &mut TestApp) {
    ctx.create_api_key("create_valid_image_should_work", false)
        .await;

    let response = create_image(
        ctx,
        "tests/fixtures/img/landscape_2048x1360.jpg",
        "create_valid_image_should_work.jpg",
        mime::IMAGE_JPEG.as_ref(),
        Some("Example image"),
    )
    .await;

    let json = response
        .json::<Value>()
        .await
        .expect("Cannot parse json body");

    let (root_image_url, lazy_image_url) = assert_image_output(&json);

    {
        let root_image_path = services::image::download_file(&root_image_url).await;
        let root_image_parsed =
            image::open(&root_image_path).expect("Failed to open/parse root image");
        assert!(root_image_parsed.width() <= 1920);
        assert!(root_image_parsed.height() <= 1080);
        tokio::fs::remove_file(&root_image_path)
            .await
            .expect("Failed to remove downloaded file");
    }

    {
        let lazy_image_path = services::image::download_file(&lazy_image_url).await;
        let lazy_image_parsed =
            image::open(&lazy_image_path).expect("Failed to open/parse root image");
        assert!(lazy_image_parsed.width() <= 64);
        assert!(lazy_image_parsed.height() <= 36);
        tokio::fs::remove_file(&lazy_image_path)
            .await
            .expect("Failed to remove downloaded file");
    }
}
