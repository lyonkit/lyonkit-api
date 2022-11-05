use std::collections::HashMap;

use actix_web::{body::BoxBody, HttpRequest, HttpResponse, Responder};
use chrono::{DateTime, Utc};
use entity::file::Model;
use getset::Getters;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Clone, Getters)]
#[serde(rename_all = "camelCase")]
#[getset(get = "pub")]
pub struct FileInput {
    pub content_type: Option<String>,
    pub content_length: u32,
    pub file_name: String,
    pub tags: Vec<String>,
    pub metadata: HashMap<String, String>,
}

#[derive(Serialize, Clone, Getters)]
#[serde(rename_all = "camelCase")]
#[getset(get = "pub")]
pub struct UploadFileOutput {
    pub id: i32,
    pub upload_url: String,
    pub key: String,
    pub public_url: String,
    pub tags: Vec<String>,
    pub metadata: Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Responder for UploadFileOutput {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(self)
    }
}

#[derive(Serialize, Clone, Getters)]
#[serde(rename_all = "camelCase")]
#[getset(get = "pub")]
pub struct FileOutput {
    pub id: i32,
    pub key: String,
    pub public_url: String,
    pub tags: Vec<String>,
    pub metadata: Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct FileOutputList(Vec<FileOutput>);

impl Responder for FileOutputList {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(self.0)
    }
}

pub trait IntoFileOutputList {
    fn into_file_output_list(self, base_url: &str) -> FileOutputList;
}

impl IntoFileOutputList for Vec<Model> {
    fn into_file_output_list(self, base_url: &str) -> FileOutputList {
        let files = self
            .iter()
            .map(|model| {
                let public_url = format!("{}/{}", base_url, model.storage_key());
                FileOutput {
                    id: *model.id(),
                    public_url,
                    key: model.storage_key().clone(),
                    tags: model.tags().to_owned(),
                    metadata: model.metadata().clone(),
                    created_at: *model.created_at(),
                    updated_at: *model.updated_at(),
                }
            })
            .collect();

        FileOutputList(files)
    }
}

#[derive(Debug, Deserialize, Getters)]
#[getset(get = "pub")]
pub struct FileFilter {
    tag: Option<String>,
}
