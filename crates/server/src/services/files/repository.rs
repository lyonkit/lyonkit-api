use std::collections::HashMap;

use async_trait::async_trait;
use sea_orm::{prelude::*, sea_query::Expr, ActiveModelTrait, ActiveValue::Set, ConnectionTrait};
use serde_json::{Map, Value};
use uuid::Uuid;

use entity::file::{ActiveModel, Column, Entity, Model};

use crate::{
    errors::{utils::MapApiError, ApiError},
    services::files::models::{FileInput, FileUpdateInput},
};

#[async_trait]
pub trait FilesRepository {
    async fn create_file(&self, namespace: &str, file: FileInput) -> Result<Model, ApiError>;
    async fn find_all_by_tag(
        &self,
        namespace: &str,
        tag: &Option<String>,
    ) -> Result<Vec<Model>, ApiError>;
    async fn update_file(
        &self,
        namespace: &str,
        id: &i32,
        payload: FileUpdateInput,
    ) -> Result<Model, ApiError>;
    async fn delete_file(&self, namespace: &str, id: &i32) -> Result<Model, ApiError>;
}

fn string_map_to_json(map: &HashMap<String, String>) -> Value {
    let mut value = Map::new();

    for (key, str_value) in map {
        value.insert(key.to_string(), Value::String(str_value.to_owned()));
    }

    Value::Object(value)
}

#[async_trait]
impl<T: ConnectionTrait> FilesRepository for T {
    async fn create_file(&self, namespace: &str, input: FileInput) -> Result<Model, ApiError> {
        let storage_key = format!(
            "{}_{}",
            Uuid::new_v4().to_string().replace('-', ""),
            input.file().file_name()
        );
        let file_model = entity::file::ActiveModel {
            namespace: Set(namespace.to_owned()),
            storage_key: Set(storage_key),
            tags: Set(input.tags().to_owned()),
            metadata: Set(string_map_to_json(input.metadata())),
            ..Default::default()
        };

        let inserted_file = file_model.insert(self).await.map_api_err()?;
        Ok(inserted_file)
    }

    async fn find_all_by_tag(
        &self,
        namespace: &str,
        tag: &Option<String>,
    ) -> Result<Vec<Model>, ApiError> {
        let mut query = Entity::find().filter(Column::Namespace.eq(namespace.to_owned()));

        if let Some(t) = tag {
            query = query.filter(Expr::cust_with_values(
                r#"$1 = any(tags)"#,
                vec![t.to_string()],
            ));
        }

        let files = query.all(self).await.map_api_err()?;

        Ok(files)
    }

    async fn update_file(
        &self,
        namespace: &str,
        id: &i32,
        payload: FileUpdateInput,
    ) -> Result<Model, ApiError> {
        let mut active_model: ActiveModel = Entity::find()
            .filter(Column::Namespace.eq(namespace.to_owned()))
            .filter(Column::Id.eq(id.to_owned()))
            .one(self)
            .await?
            .ok_or(ApiError::NotFound)?
            .into();

        if let Some(tags) = payload.tags() {
            active_model.tags = Set(tags.to_owned());
        }

        if let Some(metadata) = payload.metadata() {
            active_model.metadata = Set(string_map_to_json(metadata));
        }

        if let Some(file) = payload.file() {
            let storage_key = format!(
                "{}_{}",
                Uuid::new_v4().to_string().replace('-', ""),
                file.file_name()
            );
            active_model.storage_key = Set(storage_key);
        }

        let model = active_model.update(self).await?;

        Ok(model)
    }

    async fn delete_file(&self, namespace: &str, id: &i32) -> Result<Model, ApiError> {
        let model = Entity::find()
            .filter(Column::Namespace.eq(namespace.to_owned()))
            .filter(Column::Id.eq(id.to_owned()))
            .one(self)
            .await?
            .ok_or(ApiError::NotFound)?;

        model.clone().delete(self).await?;
        Ok(model)
    }
}
