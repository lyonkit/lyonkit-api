use crate::errors::utils::TryUnwrapActiveValue;
use crate::errors::ApiError;
use actix_web::body::BoxBody;
use actix_web::{HttpRequest, HttpResponse, Responder};
use chrono::Utc;
use entity::locale::Model as LocaleModel;
use entity::locale_data::{ActiveModel as LocalDataActiveModel, Model as LocaleDataModel};
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct LocalesMessages(HashMap<String, Value>);

impl From<Vec<(LocaleModel, Option<LocaleDataModel>)>> for LocalesMessages {
    fn from(entities: Vec<(LocaleModel, Option<LocaleDataModel>)>) -> Self {
        let mut locales_messages = HashMap::new();

        for entity in entities {
            if let Some(locale_data) = entity.1 {
                let lang = entity.0.lang().to_owned();
                let messages = locale_data.messages().to_owned();
                locales_messages.insert(lang, messages);
            }
        }

        LocalesMessages(locales_messages)
    }
}

impl Responder for LocalesMessages {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(self)
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LocaleOutput {
    id: i32,
    namespace: String,
    lang: String,
    messages: Value,
    created_at: chrono::DateTime<Utc>,
    updated_at: chrono::DateTime<Utc>,
}

impl TryFrom<(LocaleModel, LocalDataActiveModel)> for LocaleOutput {
    type Error = ApiError;

    fn try_from(
        (locale, locale_data): (LocaleModel, LocalDataActiveModel),
    ) -> Result<Self, Self::Error> {
        Ok(LocaleOutput {
            id: locale.id().to_owned(),
            namespace: locale.namespace().to_owned(),
            lang: locale.lang().to_owned(),
            messages: locale_data.messages.try_unwrap_av()?,
            created_at: locale.created_at().to_owned(),
            updated_at: locale.updated_at().to_owned(),
        })
    }
}

impl Responder for LocaleOutput {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(self)
    }
}
