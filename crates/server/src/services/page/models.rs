use crate::errors::{utils::try_unwrap_active_value, ApiError};
use crate::services::blok::models::BlokOutput;
use chrono::{DateTime, Utc};
use entity::page;
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PageInput {
  title: String,
  description: Option<String>,
  path: String,
}

impl PageInput {
  pub fn active_model(&self) -> page::ActiveModel {
    page::ActiveModel {
      title: Set(self.title.to_owned()),
      description: Set(self.description.to_owned()),
      path: Set(self.path.to_owned()),
      ..Default::default()
    }
  }
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PageOutput {
  id: i32,
  title: String,
  description: Option<String>,
  namespace: String,
  path: String,
  created_at: DateTime<Utc>,
  updated_at: DateTime<Utc>,
}

impl TryFrom<page::ActiveModel> for PageOutput {
  type Error = ApiError;

  fn try_from(model: page::ActiveModel) -> Result<Self, Self::Error> {
    Ok(Self {
      id: try_unwrap_active_value(model.id)?,
      namespace: try_unwrap_active_value(model.namespace)?,
      path: try_unwrap_active_value(model.path)?,
      title: try_unwrap_active_value(model.title)?,
      description: try_unwrap_active_value(model.description)?,
      created_at: try_unwrap_active_value(model.created_at)?,
      updated_at: try_unwrap_active_value(model.updated_at)?,
    })
  }
}

impl From<page::Model> for PageOutput {
  fn from(model: page::Model) -> Self {
    Self {
      id: model.id,
      namespace: model.namespace,
      path: model.path,
      title: model.title,
      description: model.description,
      created_at: model.created_at,
      updated_at: model.updated_at,
    }
  }
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PageOutputWithBloks {
  id: i32,
  title: String,
  description: Option<String>,
  namespace: String,
  path: String,
  bloks: Vec<BlokOutput>,
  created_at: DateTime<Utc>,
  updated_at: DateTime<Utc>,
}

impl TryFrom<(page::ActiveModel, Vec<entity::blok::ActiveModel>)> for PageOutputWithBloks {
  type Error = ApiError;

  fn try_from(
    model: (page::ActiveModel, Vec<entity::blok::ActiveModel>),
  ) -> Result<Self, Self::Error> {
    let (page, bloks) = model;

    Ok(Self {
      id: try_unwrap_active_value(page.id)?,
      namespace: try_unwrap_active_value(page.namespace)?,
      path: try_unwrap_active_value(page.path)?,
      title: try_unwrap_active_value(page.title)?,
      description: try_unwrap_active_value(page.description)?,
      bloks: bloks
        .into_iter()
        .map(|v| v.try_into())
        .collect::<Result<_, _>>()?,
      created_at: try_unwrap_active_value(page.created_at)?,
      updated_at: try_unwrap_active_value(page.updated_at)?,
    })
  }
}

impl From<(page::Model, Vec<entity::blok::Model>)> for PageOutputWithBloks {
  fn from(model: (page::Model, Vec<entity::blok::Model>)) -> Self {
    let (page, bloks) = model;

    Self {
      id: page.id,
      namespace: page.namespace,
      path: page.path,
      title: page.title,
      description: page.description,
      bloks: bloks.into_iter().map(|v| v.into()).collect(),
      created_at: page.created_at,
      updated_at: page.updated_at,
    }
  }
}
