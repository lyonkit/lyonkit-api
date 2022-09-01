use crate::utils::macros::exec_stmt;
use sea_orm_migration::{prelude::*, MigrationName};

pub struct Migration;

impl MigrationName for Migration {
  fn name(&self) -> &str {
    "m20220901_000009_add_slug_column_to_posts"
  }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    exec_stmt!(
      manager,
      "alter table posts drop column if exists slug, add column slug text not null unique"
    )?;

    Ok(())
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    exec_stmt!(manager, "alter table posts drop column if exists slug")?;

    Ok(())
  }
}
