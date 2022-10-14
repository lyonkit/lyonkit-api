use actix_web::http::StatusCode;
use serde_json::Value;
use std::collections::HashSet;
use std::path::Path;
use url::Url;

mod create;
mod delete;
mod read;

pub(crate) fn assert_image_output(json: &Value) -> (Url, Url) {
    let root_image = json
        .as_object()
        .expect("Expected image to be a json object");

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
    assert!(root_image.get("alt").and_then(|v| v.as_str()).is_some());
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

    let lazy_image = root_image
        .get("lazyImage")
        .unwrap()
        .as_object()
        .expect("Expected lazyImage to be a json object");

    assert_eq!(
        HashSet::from(["id", "publicUrl", "alt", "createdAt", "updatedAt"]),
        lazy_image.keys().map(|v| v.as_str()).collect(),
    );
    assert!(lazy_image.get("alt").and_then(|v| v.as_str()).is_some());
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

    (root_image_url, lazy_image_url)
}

pub(crate) async fn download_file(url: &Url) -> String {
    tokio::fs::create_dir("tests/.output").await.ok();

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
