use crate::errors::ApiError;
use crate::middlewares::api_key::{ApiKey, WriteApiKey};
use crate::server::AppState;
use crate::services::locale::models::{LocaleOutput, LocalesMessages};
use crate::services::locale::repository::LocaleRepository;
use actix_web::web::Path;
use actix_web::{get, put, web};
use serde_json::Value;

#[get("")]
pub async fn get_locales(
    data: web::Data<AppState>,
    api_key: ApiKey,
) -> Result<LocalesMessages, ApiError> {
    let namespace = api_key.namespace().to_owned();
    let locales = data.conn().get_all_locales_by_namespace(namespace).await?;
    Ok(locales)
}

#[put("/{lang}")]
pub async fn update_locale(
    data: web::Data<AppState>,
    api_key: WriteApiKey,
    lang: Path<String>,
    messages: web::Json<Value>,
) -> Result<LocaleOutput, ApiError> {
    let namespace = api_key.namespace().to_owned();
    let lang = lang.into_inner();
    let messages = messages.into_inner();

    let locale_output = data.conn().update_locale(namespace, lang, messages).await?;

    Ok(locale_output)
}
