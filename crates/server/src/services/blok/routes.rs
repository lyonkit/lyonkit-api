use super::models::{BlokInput, BlokOutput};
use crate::errors::utils::MapApiError;
use crate::middlewares::api_key::ApiKey;
use crate::services::blok::models::BlokPatchInput;
use crate::utils::serde_json_patch::Patch::Value;
use crate::{errors::ApiError, middlewares::api_key::WriteApiKey, server::AppState};
use actix_web::{delete, get, patch, post, put, web, Error as ActixError, HttpResponse};
use entity::blok::{Column, Entity, Model};
use entity::page::{Column as PageColumn, Entity as PageEntity};
use sea_orm::prelude::*;
use sea_orm::ActiveValue::Set;
use sea_orm::IntoActiveModel;

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
        .map_api_err()?
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
        .map_api_err()?
        .ok_or_else(|| ApiError::ReferenceNotFound("pageId".to_string()))?;

    Ok(model
        .save(data.conn())
        .await
        .map_api_err()
        .and_then(|model| Ok(HttpResponse::Ok().json(BlokOutput::try_from(model)?)))?)
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
        .map_api_err()?
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
        .map_api_err()?
        .ok_or_else(|| ApiError::ReferenceNotFound("pageId".to_string()))?;

    let mut model = body.active_model();
    model.id = Set(id);

    Ok(model
        .save(data.conn())
        .await
        .map_api_err()
        .and_then(|model| Ok(HttpResponse::Ok().json(BlokOutput::try_from(model)?)))?)
}

#[patch("/{id}")]
pub async fn patch_blok(
    data: web::Data<AppState>,
    path_id: web::Path<i32>,
    body: web::Json<BlokPatchInput>,
    api_key: WriteApiKey,
) -> Result<HttpResponse, ActixError> {
    let id = path_id.into_inner();

    if body.page_id().is_null() {
        return Err(ApiError::PatchNotNullable(String::from("pageId")).into());
    }

    if body.component_id().is_null() {
        return Err(ApiError::PatchNotNullable(String::from("componentId")).into());
    }

    if body.props().is_null() {
        return Err(ApiError::PatchNotNullable(String::from("props")).into());
    }

    if body.priority().is_null() {
        return Err(ApiError::PatchNotNullable(String::from("priority")).into());
    }

    if body.page_id().is_missing()
        && body.component_id().is_missing()
        && body.props().is_missing()
        && body.priority().is_missing()
    {
        return Err(ApiError::PatchAtLeastOneField.into());
    }

    let mut blok = Entity::find()
        .find_also_related(PageEntity)
        .filter(Column::Id.eq(id))
        .one(data.conn())
        .await
        .map_api_err()?
        .and_then(|(blok, page)| page.map(|p| (blok, p)))
        .and_then(|(blok, page)| {
            if &page.namespace == api_key.namespace() {
                return Some(blok);
            }
            None
        })
        .ok_or(ApiError::NotFound)?
        .into_active_model();

    if let Value(page_id) = body.page_id() {
        PageEntity::find()
            .filter(PageColumn::Namespace.eq(api_key.namespace().to_owned()))
            .filter(PageColumn::Id.eq(*page_id))
            .one(data.conn())
            .await
            .map_api_err()?
            .ok_or_else(|| ApiError::ReferenceNotFound("pageId".to_string()))?;

        blok.page_id = Set(*page_id);
    }

    if let Value(component_id) = body.component_id() {
        blok.component_id = Set(component_id.clone());
    }

    if let Value(props) = body.props() {
        blok.props = Set(props.clone());
    }

    if let Value(priority) = body.priority() {
        blok.priority = Set(*priority);
    }

    blok.id = Set(id);

    Ok(blok
        .save(data.conn())
        .await
        .map_api_err()
        .and_then(|model| Ok(HttpResponse::Ok().json(BlokOutput::try_from(model)?)))?)
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
        .map_api_err()?
        .and_then(|(blok, page)| page.map(|p| (blok, p)))
        .and_then(|(blok, page)| {
            if &page.namespace == api_key.namespace() {
                return Some(blok);
            }
            None
        })
        .ok_or(ApiError::NotFound)?;

    blok.clone().delete(data.conn()).await.map_api_err()?;

    Ok(HttpResponse::Ok().json(BlokOutput::from(blok)))
}
