use crate::errors::utils::MapApiError;
use crate::middlewares::api_key::ApiKey;
pub use crate::{
    errors::{utils::db_err_into_api_err, ApiError},
    middlewares::api_key::WriteApiKey,
    server::AppState,
    services::post::models::{PostInput, PostOutput},
};
use actix_web::{delete, get, post, put, web, Error as ActixError, HttpResponse};
use entity::post::{Column, Entity, Model};
use sea_orm::{prelude::*, ActiveValue::Set, TryIntoModel};

#[get("")]
pub async fn list_posts(
    data: web::Data<AppState>,
    api_key: ApiKey,
) -> Result<HttpResponse, ActixError> {
    let posts: Vec<Model> = Entity::find()
        .filter(Column::Namespace.eq(api_key.namespace().to_owned()))
        .all(data.conn())
        .await
        .map_api_err()?;

    Ok(HttpResponse::Ok().json(
        posts
            .into_iter()
            .map(PostOutput::from)
            .collect::<Vec<PostOutput>>(),
    ))
}

#[get("/{id}")]
pub async fn get_post(
    data: web::Data<AppState>,
    path_id: web::Path<i32>,
    api_key: ApiKey,
) -> Result<HttpResponse, ActixError> {
    let id = path_id.into_inner();

    let post = Entity::find()
        .filter(Column::Namespace.eq(api_key.namespace().to_owned()))
        .filter(Column::Id.eq(id))
        .one(data.conn())
        .await
        .map_api_err()?
        .ok_or(ApiError::NotFound)?;

    Ok(HttpResponse::Ok().json(PostOutput::from(post)))
}

#[get("/s/{slug}")]
pub async fn get_post_by_slug(
    data: web::Data<AppState>,
    path_slug: web::Path<String>,
    api_key: ApiKey,
) -> Result<HttpResponse, ActixError> {
    let slug = path_slug.into_inner();

    let post = Entity::find()
        .filter(Column::Namespace.eq(api_key.namespace().to_owned()))
        .filter(Column::Slug.eq(slug))
        .one(data.conn())
        .await
        .map_api_err()?
        .ok_or(ApiError::NotFound)?;

    Ok(HttpResponse::Ok().json(PostOutput::from(post)))
}

#[post("")]
pub async fn create_post(
    data: web::Data<AppState>,
    body: web::Json<PostInput>,
    api_key: WriteApiKey,
) -> Result<PostOutput, ApiError> {
    let mut model = body.active_model();
    model.namespace = Set(api_key.namespace().into());

    Ok(model
        .save(data.conn())
        .await
        .map_api_err()?
        .try_into_model()?
        .into())
}

#[put("/{id}")]
pub async fn update_post(
    data: web::Data<AppState>,
    path_id: web::Path<i32>,
    body: web::Json<PostInput>,
    api_key: WriteApiKey,
) -> Result<PostOutput, ApiError> {
    let id = path_id.into_inner();

    // Page must exists to be replaced
    Entity::find()
        .filter(Column::Namespace.eq(api_key.namespace().to_owned()))
        .filter(Column::Id.eq(id))
        .one(data.conn())
        .await
        .map_api_err()?
        .ok_or(ApiError::NotFound)?;

    let mut model = body.active_model();
    model.namespace = Set(api_key.namespace().into());
    model.id = Set(id);

    Ok(model
        .save(data.conn())
        .await
        .map_api_err()?
        .try_into_model()?
        .into())
}

#[delete("/{id}")]
pub async fn delete_post(
    data: web::Data<AppState>,
    path_id: web::Path<i32>,
    api_key: WriteApiKey,
) -> Result<HttpResponse, ActixError> {
    let id = path_id.into_inner();

    let post: Model = Entity::find()
        .filter(Column::Namespace.eq(api_key.namespace().to_owned()))
        .filter(Column::Id.eq(id))
        .one(data.conn())
        .await
        .map_api_err()?
        .ok_or(ApiError::NotFound)?;

    post.clone().delete(data.conn()).await.map_api_err()?;

    Ok(HttpResponse::Ok().json(PostOutput::from(post)))
}
