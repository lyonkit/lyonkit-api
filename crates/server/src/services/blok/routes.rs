use crate::{
  errors::{utils::db_err_into_api_err, ApiError},
  middlewares::api_key::WriteApiKey,
  server::AppState,
  services::blok::models::{BlokInput, BlokOutput},
};
use actix_web::{post, web, Error as ActixError, HttpResponse};
use entity::page::{Column as PageColumn, Entity as PageEntity};
use sea_orm::prelude::*;

#[post("")]
pub async fn create_blok(
  data: web::Data<AppState>,
  body: web::Json<BlokInput>,
  api_key: WriteApiKey,
) -> Result<HttpResponse, ActixError> {
  let model = body.active_model();

  PageEntity::find()
    .filter(PageColumn::Namespace.eq(api_key.namespace().to_owned()))
    .filter(PageColumn::Id.eq(*body.page_id()))
    .one(data.conn())
    .await
    .map_err(db_err_into_api_err)?
    .ok_or_else(|| ApiError::ReferenceNotFound("pageId".to_string()))?;

  Ok(
    model
      .save(data.conn())
      .await
      .map_err(db_err_into_api_err)
      .and_then(|model| Ok(HttpResponse::Ok().json(BlokOutput::try_from(model)?)))?,
  )
}
