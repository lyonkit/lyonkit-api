use crate::errors::utils::try_unwrap_active_value;
use crate::errors::ApiError;
use chrono::{DateTime, Utc};
use entity::quote;
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QuoteInput {
  author: String,
  message: String,
}

impl QuoteInput {
  pub fn active_model(&self) -> quote::ActiveModel {
    quote::ActiveModel {
      author: Set(self.author.to_owned()),
      message: Set(self.message.to_owned()),
      ..Default::default()
    }
  }
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QuoteOutput {
  id: i32,
  namespace: String,
  author: String,
  message: String,
  created_at: DateTime<Utc>,
  updated_at: DateTime<Utc>,
}

impl TryFrom<quote::ActiveModel> for QuoteOutput {
  type Error = ApiError;

  fn try_from(model: quote::ActiveModel) -> Result<Self, Self::Error> {
    Ok(Self {
      id: try_unwrap_active_value(model.id)?,
      namespace: try_unwrap_active_value(model.namespace)?,
      author: try_unwrap_active_value(model.author)?,
      message: try_unwrap_active_value(model.message)?,
      created_at: try_unwrap_active_value(model.created_at)?,
      updated_at: try_unwrap_active_value(model.updated_at)?,
    })
  }
}

impl From<quote::Model> for QuoteOutput {
  fn from(model: quote::Model) -> Self {
    Self {
      id: model.id,
      namespace: model.namespace,
      author: model.author,
      message: model.message,
      created_at: model.created_at,
      updated_at: model.updated_at,
    }
  }
}
