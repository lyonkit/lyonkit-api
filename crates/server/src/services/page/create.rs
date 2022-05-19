use crate::{errors::ApiError, middlewares::api_key::WriteApiKey, AppState};
use actix_web::{post, web, Error, HttpResponse};
use chrono::{DateTime, Utc};
use entity::page;
use sea_orm::{entity::*, ActiveValue::Set};
use serde::{Deserialize, Serialize};
use tracing::error;

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreatePageInput {
  title: String,
  description: Option<String>,
  path: String,
}

impl CreatePageInput {
  fn active_model_from_namespace(&self, namespace: String) -> page::ActiveModel {
    page::ActiveModel {
      namespace: Set(namespace),
      title: Set(self.title.to_owned()),
      description: Set(self.description.to_owned()),
      path: Set(self.path.to_owned()),
      ..Default::default()
    }
  }
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreatePageOutput {
  id: i32,
  title: String,
  description: Option<String>,
  namespace: String,
  path: String,
  created_at: DateTime<Utc>,
  updated_at: DateTime<Utc>,
}

impl CreatePageOutput {
  fn from_active_model(model: page::ActiveModel) -> Self {
    Self {
      id: model.id.unwrap(),
      namespace: model.namespace.unwrap(),
      path: model.path.unwrap(),
      title: model.title.unwrap(),
      description: model.description.unwrap(),
      created_at: model.created_at.unwrap(),
      updated_at: model.updated_at.unwrap(),
    }
  }
}

#[post("")]
pub async fn create_page(
  data: web::Data<AppState>,
  body: web::Json<CreatePageInput>,
  api_key: WriteApiKey,
) -> Result<HttpResponse, Error> {
  Ok(
    body
      .active_model_from_namespace(api_key.namespace().into())
      .save(&data.conn)
      .await
      .map(|model| HttpResponse::Ok().json(CreatePageOutput::from_active_model(model)))
      .map_err(|e| {
        error!(
          error_message = format!("{:?}", e).as_str(),
          "An error occured while saving page"
        );
        ApiError::DbError
      })?,
  )
}
