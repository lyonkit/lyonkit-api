use crate::helpers::TestApp;
use reqwest::multipart;
use reqwest::{Response, StatusCode};
use serde_json::Value;
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use test_context::test_context;
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};
use url::Url;

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
    .expect(format!("Failed to open image at {}", file_path).as_str());

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

pub async fn download_file(url: &Url) -> String {
  fs::create_dir("tests/.output").ok();

  let filename = url
    .path_segments()
    .and_then(|v| v.last())
    .expect("Expected a filename in url");
  let response = reqwest::get(url.to_string())
    .await
    .expect("Failed to execute request to download file");
  let filepath = format!("tests/.output/{}", filename);
  assert_eq!(StatusCode::OK, response.status());
  let content = response
    .bytes()
    .await
    .expect("Cannot get bytes from server request to download file");
  tokio::fs::write(Path::new(&filepath), content)
    .await
    .expect("Cannot copy bytes to file");
  filepath
}

#[test_context(TestApp)]
#[tokio::test]
async fn create_valid_image_should_work(ctx: &mut TestApp) {
  ctx
    .create_api_key("create_valid_image_should_work", false)
    .await;

  let response = create_image(
    ctx,
    "tests/fixtures/img/landscape.jpg",
    "create_valid_image_should_work.jpg",
    mime::IMAGE_JPEG.as_ref(),
    Some("Example image"),
  )
  .await;

  let json = response
    .json::<Value>()
    .await
    .expect("Cannot parse json body");

  let root_image = json
    .as_object()
    .expect("Expected response to be a json object");

  assert_eq!(
    HashSet::from([
      "id",
      "publicUrl",
      "lazyImage",
      "alt",
      "createdAt",
      "updatedAt"
    ]),
    root_image.keys().map(|v| v.as_str()).collect(),
  );
  assert_eq!(
    Some(&Value::String(String::from("Example image"))),
    root_image.get("alt")
  );
  assert!(root_image.get("id").and_then(|v| v.as_i64()).is_some());
  assert!(root_image
    .get("createdAt")
    .and_then(|v| v.as_str())
    .is_some());
  assert!(root_image
    .get("updatedAt")
    .and_then(|v| v.as_str())
    .is_some());

  let root_image_url: Url = root_image
    .get("publicUrl")
    .and_then(|v| v.as_str())
    .expect("Expect public_url to be a string")
    .parse()
    .expect("Public URL is not a valid url");

  assert!(root_image_url
    .path_segments()
    .and_then(|v| v.last())
    .expect("Public URL should have a path")
    .ends_with(".jpeg"));

  {
    let root_image_path = download_file(&root_image_url).await;
    let root_image_parsed = image::open(&root_image_path).expect("Failed to open/parse root image");
    assert!(root_image_parsed.width() <= 1920);
    assert!(root_image_parsed.height() <= 1080);
    tokio::fs::remove_file(&root_image_path)
      .await
      .expect("Failed to remove downloaded file");
  }

  let lazy_image = root_image
    .get("lazyImage")
    .unwrap()
    .as_object()
    .expect("Expected lazyImage to be a json object");

  assert_eq!(
    HashSet::from(["id", "publicUrl", "alt", "createdAt", "updatedAt"]),
    lazy_image.keys().map(|v| v.as_str()).collect(),
  );
  assert_eq!(
    Some(&Value::String(String::from("Example image"))),
    lazy_image.get("alt")
  );
  assert!(lazy_image.get("id").and_then(|v| v.as_i64()).is_some());
  assert!(lazy_image
    .get("createdAt")
    .and_then(|v| v.as_str())
    .is_some());
  assert!(lazy_image
    .get("updatedAt")
    .and_then(|v| v.as_str())
    .is_some());

  let lazy_image_url: Url = lazy_image
    .get("publicUrl")
    .and_then(|v| v.as_str())
    .expect("Expect publicUrl to be a string")
    .parse()
    .expect("Public URL is not a valid url");

  assert!(lazy_image_url
    .path_segments()
    .and_then(|v| v.last())
    .expect("Public URL should have a path")
    .ends_with(".jpeg"));

  {
    let lazy_image_path = download_file(&lazy_image_url).await;
    let lazy_image_parsed = image::open(&lazy_image_path).expect("Failed to open/parse root image");
    assert!(lazy_image_parsed.width() <= 64);
    assert!(lazy_image_parsed.height() <= 36);
    tokio::fs::remove_file(&lazy_image_path)
      .await
      .expect("Failed to remove downloaded file");
  }
}
