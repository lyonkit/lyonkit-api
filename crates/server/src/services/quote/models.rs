use actix_web::{body::BoxBody, HttpRequest, HttpResponse, Responder};
use chrono::{DateTime, Utc};
use entity::quote;
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QuoteInput {
    author: String,
    message: String,
}

impl QuoteInput {
    pub fn active_model(&self) -> quote::ActiveModel {
        quote::ActiveModel {
            author: Set(self.author.to_owned()),
            message: Set(self.message.to_owned()),
            ..Default::default()
        }
    }
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QuoteOutput {
    id: i32,
    namespace: String,
    author: String,
    message: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<quote::Model> for QuoteOutput {
    fn from(model: quote::Model) -> Self {
        Self {
            id: model.id,
            namespace: model.namespace,
            author: model.author,
            message: model.message,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

impl Responder for QuoteOutput {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(self)
    }
}
