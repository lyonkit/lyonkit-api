use crate::errors::utils::db_err_into_api_err;
use crate::errors::ApiError;
use crate::middlewares::api_key::{ApiKey, WriteApiKey};
use crate::server::AppState;
use crate::services::quote::models::{QuoteInput, QuoteOutput};
use actix_web::{delete, get, post, put, web, Error as ActixError, HttpResponse};
use entity::quote::{Column, Entity, Model};
use sea_orm::prelude::*;
use sea_orm::ActiveValue::Set;

#[get("")]
pub async fn list_quotes(
  data: web::Data<AppState>,
  api_key: ApiKey,
) -> Result<HttpResponse, ActixError> {
  let quotes: Vec<Model> = Entity::find()
    .filter(Column::Namespace.eq(api_key.namespace().to_owned()))
    .all(data.conn())
    .await
    .map_err(db_err_into_api_err)?;

  Ok(
    HttpResponse::Ok().json(
      quotes
        .into_iter()
        .map(QuoteOutput::from)
        .collect::<Vec<QuoteOutput>>(),
    ),
  )
}

#[post("")]
pub async fn create_quote(
  data: web::Data<AppState>,
  body: web::Json<QuoteInput>,
  api_key: WriteApiKey,
) -> Result<HttpResponse, ActixError> {
  let mut model = body.active_model();
  model.namespace = Set(api_key.namespace().into());

  Ok(
    model
      .save(data.conn())
      .await
      .map_err(db_err_into_api_err)
      .and_then(|model| Ok(HttpResponse::Ok().json(QuoteOutput::try_from(model)?)))?,
  )
}

#[put("/{id}")]
pub async fn update_quote(
  data: web::Data<AppState>,
  path_id: web::Path<i32>,
  body: web::Json<QuoteInput>,
  api_key: WriteApiKey,
) -> Result<HttpResponse, ActixError> {
  let id = path_id.into_inner();

  // Page must exists to be replaced
  Entity::find()
    .filter(Column::Namespace.eq(api_key.namespace().to_owned()))
    .filter(Column::Id.eq(id))
    .one(data.conn())
    .await
    .map_err(db_err_into_api_err)?
    .ok_or(ApiError::NotFound)?;

  let mut model = body.active_model();
  model.namespace = Set(api_key.namespace().into());
  model.id = Set(id);

  Ok(
    model
      .save(data.conn())
      .await
      .map_err(db_err_into_api_err)
      .and_then(|model| Ok(HttpResponse::Ok().json(QuoteOutput::try_from(model)?)))?,
  )
}

#[delete("/{id}")]
pub async fn delete_quote(
  data: web::Data<AppState>,
  path_id: web::Path<i32>,
  api_key: WriteApiKey,
) -> Result<HttpResponse, ActixError> {
  let id = path_id.into_inner();

  let quote: Model = Entity::find()
    .filter(Column::Namespace.eq(api_key.namespace().to_owned()))
    .filter(Column::Id.eq(id))
    .one(data.conn())
    .await
    .map_err(db_err_into_api_err)?
    .ok_or(ApiError::NotFound)?;

  quote
    .clone()
    .delete(data.conn())
    .await
    .map_err(db_err_into_api_err)?;

  Ok(HttpResponse::Ok().json(QuoteOutput::from(quote)))
}
