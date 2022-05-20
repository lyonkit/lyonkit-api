use crate::{
  errors::{
    utils::{db_err_into_api_err, try_unwrap_active_value},
    ApiError,
  },
  middlewares::api_key::WriteApiKey,
  AppState,
};
use actix_web::{post, web, Error, HttpResponse};
use chrono::{DateTime, Utc};
use entity::page;
use sea_orm::{entity::*, ActiveValue::Set};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreatePageInput {
  title: String,
  description: Option<String>,
  path: String,
}

impl CreatePageInput {
  fn active_model_from(&self, namespace: String) -> page::ActiveModel {
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

impl TryFrom<page::ActiveModel> for CreatePageOutput {
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

#[post("")]
pub async fn create_page(
  data: web::Data<AppState>,
  body: web::Json<CreatePageInput>,
  api_key: WriteApiKey,
) -> Result<HttpResponse, Error> {
  Ok(
    body
      .active_model_from(api_key.namespace().into())
      .save(&data.conn)
      .await
      .map_err(db_err_into_api_err)
      .and_then(|model| Ok(HttpResponse::Ok().json(CreatePageOutput::try_from(model)?)))?,
  )
}
