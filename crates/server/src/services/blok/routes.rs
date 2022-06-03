use crate::middlewares::api_key::ApiKey;
use crate::{
  errors::{utils::db_err_into_api_err, ApiError},
  middlewares::api_key::WriteApiKey,
  server::AppState,
  services::blok::models::{BlokInput, BlokOutput},
};
use actix_web::{delete, get, post, put, web, Error as ActixError, HttpResponse};
use entity::blok::{Column, Entity, Model};
use entity::page::{Column as PageColumn, Entity as PageEntity};
use sea_orm::prelude::*;
use sea_orm::ActiveValue::Set;

#[get("/{id}")]
pub async fn get_blok(
  data: web::Data<AppState>,
  path_id: web::Path<i32>,
  api_key: ApiKey,
) -> Result<HttpResponse, ActixError> {
  let id = path_id.into_inner();

  let blok: Model = Entity::find()
    .find_also_related(PageEntity)
    .filter(Column::Id.eq(id))
    .one(data.conn())
    .await
    .map_err(db_err_into_api_err)?
    .and_then(|(blok, page)| page.map(|p| (blok, p)))
    .and_then(|(blok, page)| {
      if &page.namespace == api_key.namespace() {
        return Some(blok);
      }
      None
    })
    .ok_or(ApiError::NotFound)?;

  Ok(HttpResponse::Ok().json(BlokOutput::from(blok)))
}

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

#[put("/{id}")]
pub async fn update_blok(
  data: web::Data<AppState>,
  path_id: web::Path<i32>,
  body: web::Json<BlokInput>,
  api_key: WriteApiKey,
) -> Result<HttpResponse, ActixError> {
  let id = path_id.into_inner();

  Entity::find()
    .find_also_related(PageEntity)
    .filter(Column::Id.eq(id))
    .one(data.conn())
    .await
    .map_err(db_err_into_api_err)?
    .and_then(|(blok, page)| page.map(|p| (blok, p)))
    .and_then(|(blok, page)| {
      if &page.namespace == api_key.namespace() {
        return Some(blok);
      }
      None
    })
    .ok_or(ApiError::NotFound)?;

  PageEntity::find()
    .filter(PageColumn::Namespace.eq(api_key.namespace().to_owned()))
    .filter(PageColumn::Id.eq(*body.page_id()))
    .one(data.conn())
    .await
    .map_err(db_err_into_api_err)?
    .ok_or_else(|| ApiError::ReferenceNotFound("pageId".to_string()))?;

  let mut model = body.active_model();
  model.id = Set(id);

  Ok(
    model
      .save(data.conn())
      .await
      .map_err(db_err_into_api_err)
      .and_then(|model| Ok(HttpResponse::Ok().json(BlokOutput::try_from(model)?)))?,
  )
}

#[delete("/{id}")]
pub async fn delete_blok(
  data: web::Data<AppState>,
  path_id: web::Path<i32>,
  api_key: ApiKey,
) -> Result<HttpResponse, ActixError> {
  let id = path_id.into_inner();

  let blok = Entity::find()
    .find_also_related(PageEntity)
    .filter(Column::Id.eq(id))
    .one(data.conn())
    .await
    .map_err(db_err_into_api_err)?
    .and_then(|(blok, page)| page.map(|p| (blok, p)))
    .and_then(|(blok, page)| {
      if &page.namespace == api_key.namespace() {
        return Some(blok);
      }
      None
    })
    .ok_or(ApiError::NotFound)?;

  blok
    .clone()
    .delete(data.conn())
    .await
    .map_err(db_err_into_api_err)?;

  Ok(HttpResponse::Ok().json(BlokOutput::from(blok)))
}
