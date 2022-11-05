use crate::{
    errors::{utils::MapApiError, ApiError},
    services::locale::models::{LocaleOutput, LocalesMessages},
};
use async_trait::async_trait;
use entity::{
    locale::{Column as LocaleColumn, Entity as LocaleEntity},
    locale_data::{ActiveModel as LocalDataActiveModel, Entity as LocalDataEntity},
};
use sea_orm::{prelude::*, ActiveValue::Set, ConnectionTrait};

#[async_trait]
pub trait LocaleRepository {
    async fn get_all_locales_by_namespace(
        &self,
        namespace: String,
    ) -> Result<LocalesMessages, ApiError>;

    async fn update_locale(
        &self,
        namespace: String,
        lang: String,
        new_messages: serde_json::Value,
    ) -> Result<LocaleOutput, ApiError>;
}

#[async_trait]
impl<T: ConnectionTrait> LocaleRepository for T {
    async fn get_all_locales_by_namespace(
        &self,
        namespace: String,
    ) -> Result<LocalesMessages, ApiError> {
        let locales = LocaleEntity::find()
            .filter(LocaleColumn::Namespace.eq(namespace))
            .find_also_related(LocalDataEntity)
            .all(self)
            .await
            .map_api_err()?;

        Ok(locales.into())
    }

    async fn update_locale(
        &self,
        namespace: String,
        lang: String,
        new_messages: serde_json::Value,
    ) -> Result<LocaleOutput, ApiError> {
        let (locale, locale_data) = LocaleEntity::find()
            .filter(LocaleColumn::Namespace.eq(namespace))
            .filter(LocaleColumn::Lang.eq(lang))
            .find_also_related(LocalDataEntity)
            .one(self)
            .await
            .map_api_err()?
            .ok_or(ApiError::NotFound)?;

        let mut model: LocalDataActiveModel = locale_data.ok_or(ApiError::NotFound)?.into();
        model.messages = Set(new_messages);
        model.clone().update(self).await.map_api_err()?;

        Ok((locale, model).try_into()?)
    }
}
