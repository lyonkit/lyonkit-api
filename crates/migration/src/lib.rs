use async_trait::async_trait;
pub use sea_orm_migration::prelude::*;

mod m20220517_000001_create_namespace_table;
mod m20220517_000002_create_page_table;
mod m20220517_000003_create_api_keys_table;
mod m20220517_000004_create_bloks_table;
mod m20220610_000005_create_images_table;
mod m20220610_000006_create_posts_table;
mod m20220716_000007_fix_blok_priority_trigger;
mod m20220831_000008_create_blok_table;
mod m20220901_000009_add_slug_column_to_posts;
mod m20220901_000010_create_git_auths_table;
mod m20220917_000011_fix_blok_priority_trigger;
mod m20220917_000012_fix_blok_priority_trigger;
mod m20221013_000013_create_locale_table;
mod m20221023_000014_add_timestamps_to_locales_data;
mod m20221103_000015_create_files_table;
pub(crate) mod utils;

pub struct Migrator;

#[async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220517_000001_create_namespace_table::Migration),
            Box::new(m20220517_000002_create_page_table::Migration),
            Box::new(m20220517_000003_create_api_keys_table::Migration),
            Box::new(m20220517_000004_create_bloks_table::Migration),
            Box::new(m20220610_000005_create_images_table::Migration),
            Box::new(m20220610_000006_create_posts_table::Migration),
            Box::new(m20220716_000007_fix_blok_priority_trigger::Migration),
            Box::new(m20220831_000008_create_blok_table::Migration),
            Box::new(m20220901_000009_add_slug_column_to_posts::Migration),
            Box::new(m20220901_000010_create_git_auths_table::Migration),
            Box::new(m20220917_000011_fix_blok_priority_trigger::Migration),
            Box::new(m20220917_000012_fix_blok_priority_trigger::Migration),
            Box::new(m20221013_000013_create_locale_table::Migration),
            Box::new(m20221023_000014_add_timestamps_to_locales_data::Migration),
            Box::new(m20221103_000015_create_files_table::Migration),
        ]
    }
}
