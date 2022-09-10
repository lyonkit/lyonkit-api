use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Getters, Setters, Deserialize, Serialize)]
#[getset(get = "pub", set = "pub")]
pub struct GitJsonFile {
  sha: String,
  content: serde_json::Value,
}

#[derive(Getters, Deserialize, Serialize, TypedBuilder)]
#[getset(get = "pub")]
pub struct GitCommitPayload {
  message: String,
  branch: String,
  sha: String,
  content: String,
}
