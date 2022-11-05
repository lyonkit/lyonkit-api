use async_trait::async_trait;
use sea_orm::{prelude::*, ActiveValue::Set, ConnectionTrait};
use serde_json::Value;

mod read;
mod update;

#[async_trait]
pub trait LocaleFixtures {
    async fn create_locale(&self, namespace: &str, lang: &str, values: serde_json::Value);
}

#[async_trait]
impl<T: ConnectionTrait> LocaleFixtures for T {
    async fn create_locale(&self, namespace: &str, lang: &str, values: Value) {
        let locales_data = entity::locale_data::ActiveModel {
            messages: Set(values),
            ..Default::default()
        };
        let locales_data = entity::locale_data::Entity::insert(locales_data)
            .exec(self)
            .await
            .expect("Failed to create fixture");

        let locale = entity::locale::ActiveModel {
            namespace: Set(namespace.to_string()),
            lang: Set(lang.to_string()),
            locale_data_id: Set(locales_data.last_insert_id),
            ..Default::default()
        };
        entity::locale::Entity::insert(locale)
            .exec(self)
            .await
            .expect("Failed to create fixture");
    }
}
