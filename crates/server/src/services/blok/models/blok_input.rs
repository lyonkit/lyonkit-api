use entity::blok;
use getset::Getters;
use sea_orm::ActiveValue::Set;
use sea_orm::NotSet;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Clone, Getters)]
#[serde(rename_all = "camelCase")]
#[getset(get = "pub")]
pub struct BlokInput {
  page_id: i32,
  component_id: String,
  props: Value,
  priority: Option<i32>,
}

impl BlokInput {
  pub fn active_model(&self) -> blok::ActiveModel {
    blok::ActiveModel {
      page_id: Set(self.page_id.to_owned()),
      component_id: Set(self.component_id.to_owned()),
      props: Set(self.props.to_owned()),
      priority: self.priority.map(Set).unwrap_or(NotSet),
      ..Default::default()
    }
  }
}
