use crate::utils::serde_json_patch::Patch;
use getset::Getters;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Clone, Getters)]
#[serde(rename_all = "camelCase")]
#[getset(get = "pub")]
pub struct BlokPatchInput {
  #[serde(default)]
  page_id: Patch<i32>,
  #[serde(default)]
  component_id: Patch<String>,
  #[serde(default)]
  props: Patch<Value>,
  #[serde(default)]
  priority: Patch<i32>,
}
