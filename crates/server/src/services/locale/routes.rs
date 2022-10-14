use crate::errors::utils::MapApiError;
use crate::errors::ApiError;
use crate::middlewares::api_key::{ApiKey, WriteApiKey};
use crate::server::AppState;
use crate::services::locale::models::{LocaleOutput, LocalesMessages};
use actix_web::web::Path;
use actix_web::{get, put, web, Error as ActixError};
use entity::locale::{Column as LocaleColumn, Entity as LocaleEntity, Model as LocaleModel};
use entity::locale_data::{
    ActiveModel as LocalDataActiveModel, Entity as LocalDataEntity, Model as LocaleDataModel,
};
use sea_orm::prelude::*;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, QueryFilter};
use serde_json::Value;

#[get("")]
pub async fn get_locales(
    data: web::Data<AppState>,
    api_key: ApiKey,
) -> Result<LocalesMessages, ActixError> {
    let locales: Vec<(LocaleModel, Option<LocaleDataModel>)> = LocaleEntity::find()
        .filter(LocaleColumn::Namespace.eq(api_key.namespace().clone()))
        .find_also_related(LocalDataEntity)
        .all(data.conn())
        .await
        .map_api_err()?;

    Ok(locales.into())
}

#[put("/{lang}")]
pub async fn update_locale(
    data: web::Data<AppState>,
    api_key: WriteApiKey,
    lang: Path<String>,
    messages: web::Json<Value>,
) -> Result<LocaleOutput, ActixError> {
    let (locale, locale_data) = LocaleEntity::find()
        .filter(LocaleColumn::Namespace.eq(api_key.namespace().to_owned()))
        .filter(LocaleColumn::Lang.eq(lang.into_inner()))
        .find_also_related(LocalDataEntity)
        .one(data.conn())
        .await
        .map_api_err()?
        .ok_or(ApiError::NotFound)?;

    let mut model: LocalDataActiveModel = locale_data.ok_or(ApiError::NotFound)?.into();
    model.messages = Set(messages.into_inner());
    model.clone().update(data.conn()).await.map_api_err()?;

    Ok((locale, model).try_into()?)
}
