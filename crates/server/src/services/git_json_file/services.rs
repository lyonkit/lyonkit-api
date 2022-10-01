use crate::errors::ApiError;
use crate::services::git_json_file::models::GitJsonFile;
use base64::decode as decode_b64;
use lazy_static::lazy_static;
use reqwest::Client;
use tracing::error;

lazy_static! {
  pub static ref GITHUB_CLIENT: reqwest::Client =
    Client::builder().https_only(true).build().unwrap();
}

pub(crate) async fn fetch_git_json_file(
  path: &String,
  namespace: &String,
  org: &String,
  repo: &String,
  github_token: &String,
) -> Result<GitJsonFile, ApiError> {
  let url = format!("https://api.github.com/repos/{org}/{repo}/contents/{path}");
  let response = GITHUB_CLIENT
    .get(&url)
    .header("Authorization", github_token)
    .header("User-Agent", format!("LyonkitApi ({})", namespace))
    .send()
    .await
    .map_err(|err| {
      error!(
        url = &url,
        error = format!("{:?}", &err),
        "An error occured while querying git file"
      );
      ApiError::GitError
    })?;

  // On decode le body de b64 à String à JSON
  let mut body = response.json::<GitJsonFile>().await.map_err(|err| {
    error!(
      url = &url,
      error = format!("{:?}", &err),
      "Failed to parse body of retrieved git file"
    );
    ApiError::GitBodyUnparseable
  })?;

  let content = body
    .content()
    .as_str()
    .map(|c| c.to_string())
    .ok_or_else(|| "no content field".to_string())
    .and_then(|b64_content| {
      decode_b64(b64_content.replace('\n', "")).map_err(|err| format!("{:?}", err))
    })
    .and_then(|bytes| {
      serde_json::from_slice::<serde_json::Value>(bytes.as_slice())
        .map_err(|err| format!("{:?}", err))
    })
    .map_err(|err| {
      error!(
        url = &url,
        error = format!("{:?}", &err),
        "Failed to parse body of retrieved git file"
      );
      ApiError::GitBodyUnparseable
    })?;

  body.set_content(content);

  Ok(body)
}
