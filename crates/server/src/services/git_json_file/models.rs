use derive_builder::Builder;
use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};

#[derive(Getters, Setters, Deserialize, Serialize)]
#[getset(get = "pub", set = "pub")]
pub struct GitJsonFile {
  sha: String,
  content: serde_json::Value,
}

#[derive(Getters, Deserialize, Serialize, Builder)]
#[getset(get = "pub")]
pub struct GitCommitPayload {
  message: String,
  branch: String,
  sha: String,
  content: String,
}
