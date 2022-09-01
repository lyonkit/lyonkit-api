use crate::errors::{utils::try_unwrap_active_value, ApiError};
use chrono::{DateTime, Utc};
use entity::post;
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PostInput {
  title: String,
  description: Option<String>,
  slug: String,
  body: serde_json::Value,
}

impl PostInput {
  pub fn active_model(&self) -> post::ActiveModel {
    post::ActiveModel {
      title: Set(self.title.to_owned()),
      description: Set(self.description.to_owned()),
      slug: Set(self.slug.to_owned()),
      body: Set(self.body.to_owned()),
      ..Default::default()
    }
  }
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PostOutput {
  id: i32,
  title: String,
  description: Option<String>,
  slug: String,
  namespace: String,
  body: serde_json::Value,
  created_at: DateTime<Utc>,
  updated_at: DateTime<Utc>,
}

impl TryFrom<post::ActiveModel> for PostOutput {
  type Error = ApiError;

  fn try_from(model: post::ActiveModel) -> Result<Self, Self::Error> {
    Ok(Self {
      id: try_unwrap_active_value(model.id)?,
      namespace: try_unwrap_active_value(model.namespace)?,
      title: try_unwrap_active_value(model.title)?,
      description: try_unwrap_active_value(model.description)?,
      slug: try_unwrap_active_value(model.slug)?,
      body: try_unwrap_active_value(model.body)?,
      created_at: try_unwrap_active_value(model.created_at)?,
      updated_at: try_unwrap_active_value(model.updated_at)?,
    })
  }
}

impl From<post::Model> for PostOutput {
  fn from(model: post::Model) -> Self {
    Self {
      id: model.id,
      namespace: model.namespace,
      title: model.title,
      description: model.description,
      slug: model.slug,
      body: model.body,
      created_at: model.created_at,
      updated_at: model.updated_at,
    }
  }
}
