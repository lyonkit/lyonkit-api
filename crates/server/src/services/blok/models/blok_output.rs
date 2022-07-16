use crate::errors::utils::try_unwrap_active_value;
use crate::errors::ApiError;
use chrono::{DateTime, Utc};
use entity::blok;
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BlokOutput {
  id: i32,
  page_id: i32,
  component_id: String,
  props: Value,
  priority: i32,
  created_at: DateTime<Utc>,
  updated_at: DateTime<Utc>,
}

impl TryFrom<blok::ActiveModel> for BlokOutput {
  type Error = ApiError;

  fn try_from(model: blok::ActiveModel) -> Result<Self, Self::Error> {
    Ok(Self {
      id: try_unwrap_active_value(model.id)?,
      page_id: try_unwrap_active_value(model.page_id)?,
      component_id: try_unwrap_active_value(model.component_id)?,
      props: try_unwrap_active_value(model.props)?,
      priority: try_unwrap_active_value(model.priority)?,
      created_at: try_unwrap_active_value(model.created_at)?,
      updated_at: try_unwrap_active_value(model.updated_at)?,
    })
  }
}

impl From<blok::Model> for BlokOutput {
  fn from(model: blok::Model) -> Self {
    Self {
      id: model.id,
      page_id: model.page_id,
      component_id: model.component_id,
      props: model.props,
      priority: model.priority,
      created_at: model.created_at,
      updated_at: model.updated_at,
    }
  }
}
