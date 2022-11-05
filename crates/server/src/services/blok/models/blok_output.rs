use actix_web::{body::BoxBody, HttpRequest, HttpResponse, Responder};
use chrono::{DateTime, Utc};
use entity::blok;
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BlokOutput {
    id: i32,
    page_id: i32,
    component_id: String,
    props: Value,
    priority: i32,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<blok::Model> for BlokOutput {
    fn from(model: blok::Model) -> Self {
        Self {
            id: model.id,
            page_id: model.page_id,
            component_id: model.component_id,
            props: model.props,
            priority: model.priority,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

impl Responder for BlokOutput {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(self)
    }
}
