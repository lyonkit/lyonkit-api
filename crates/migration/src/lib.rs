use async_trait::async_trait;
pub use sea_orm_migration::prelude::*;

mod m20220517_000001_create_namespace_table;
mod m20220517_000002_create_page_table;
mod m20220517_000003_create_api_keys_table;
mod m20220517_000004_create_bloks_table;
mod m20220610_000005_create_images_table;
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
    ]
  }
}
