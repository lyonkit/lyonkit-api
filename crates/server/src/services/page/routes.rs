use crate::middlewares::api_key::ApiKey;
use crate::services::page::models::PageOutputWithBloks;
pub use crate::{
  errors::{utils::db_err_into_api_err, ApiError},
  middlewares::api_key::WriteApiKey,
  server::AppState,
  services::page::models::{PageInput, PageOutput},
};
use actix_web::{delete, get, post, put, web, Error as ActixError, HttpResponse};
use entity::page::{Column, Entity, Model};
use sea_orm::{prelude::*, ActiveValue::Set, Order};

#[get("")]
pub async fn list_pages(
  data: web::Data<AppState>,
  api_key: ApiKey,
) -> Result<HttpResponse, ActixError> {
  let pages: Vec<Model> = Entity::find()
    .filter(Column::Namespace.eq(api_key.namespace().to_owned()))
    .all(data.conn())
    .await
    .map_err(db_err_into_api_err)?;

  Ok(
    HttpResponse::Ok().json(
      pages
        .into_iter()
        .map(PageOutput::from)
        .collect::<Vec<PageOutput>>(),
    ),
  )
}

#[get("/wb/{path}*")]
pub async fn get_page_with_blok(
  data: web::Data<AppState>,
  path: web::Path<String>,
  api_key: ApiKey,
) -> Result<HttpResponse, ActixError> {
  let path = format!("/{}", path);

  let mut q = Entity::find()
    .filter(Column::Namespace.eq(api_key.namespace().to_owned()))
    .filter(Column::Path.eq(path.as_str()))
    .find_with_related(entity::blok::Entity);
  sea_orm::QueryTrait::query(&mut q).order_by(entity::blok::Column::Priority, Order::Desc);
  let result = q.all(data.conn()).await.map_err(db_err_into_api_err)?;

  let result: &(Model, Vec<entity::blok::Model>) = result.get(0).ok_or(ApiError::NotFound)?;

  Ok(HttpResponse::Ok().json(PageOutputWithBloks::from(result.to_owned())))
}

#[post("")]
pub async fn create_page(
  data: web::Data<AppState>,
  body: web::Json<PageInput>,
  api_key: WriteApiKey,
) -> Result<HttpResponse, ActixError> {
  let mut model = body.active_model();
  model.namespace = Set(api_key.namespace().into());

  Ok(
    model
      .save(data.conn())
      .await
      .map_err(db_err_into_api_err)
      .and_then(|model| Ok(HttpResponse::Ok().json(PageOutput::try_from(model)?)))?,
  )
}

#[put("/{id}")]
pub async fn update_page(
  data: web::Data<AppState>,
  path_id: web::Path<i32>,
  body: web::Json<PageInput>,
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
      .and_then(|model| Ok(HttpResponse::Ok().json(PageOutput::try_from(model)?)))?,
  )
}

#[delete("/{id}")]
pub async fn delete_page(
  data: web::Data<AppState>,
  path_id: web::Path<i32>,
  api_key: WriteApiKey,
) -> Result<HttpResponse, ActixError> {
  let id = path_id.into_inner();

  let page: Model = Entity::find()
    .filter(Column::Namespace.eq(api_key.namespace().to_owned()))
    .filter(Column::Id.eq(id))
    .one(data.conn())
    .await
    .map_err(db_err_into_api_err)?
    .ok_or(ApiError::NotFound)?;

  page
    .clone()
    .delete(data.conn())
    .await
    .map_err(db_err_into_api_err)?;

  Ok(HttpResponse::Ok().json(PageOutput::from(page)))
}
