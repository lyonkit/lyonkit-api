use actix_web::{body::BoxBody, HttpRequest, HttpResponse, Responder};
use chrono::{DateTime, Utc};
use entity::post;
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PostInput {
    title: String,
    description: Option<String>,
    slug: String,
    body: serde_json::Value,
}

impl PostInput {
    pub fn active_model(&self) -> post::ActiveModel {
        post::ActiveModel {
            title: Set(self.title.to_owned()),
            description: Set(self.description.to_owned()),
            slug: Set(self.slug.to_owned()),
            body: Set(self.body.to_owned()),
            ..Default::default()
        }
    }
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PostOutput {
    id: i32,
    title: String,
    description: Option<String>,
    slug: String,
    namespace: String,
    body: serde_json::Value,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<post::Model> for PostOutput {
    fn from(model: post::Model) -> Self {
        Self {
            id: model.id,
            namespace: model.namespace,
            title: model.title,
            description: model.description,
            slug: model.slug,
            body: model.body,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

impl Responder for PostOutput {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(self)
    }
}
