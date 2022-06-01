use crate::utils::macros::{create_table_from_entity, exec_stmt};
use entity::namespace::Entity;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
  fn name(&self) -> &str {
    "m20220517_000001_create_namespace_table"
  }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    exec_stmt!(manager, r#"drop table if exists namespaces"#)?;
    create_table_from_entity!(manager, Entity)?;

    // Set default value for created_at / updated_at columns
    exec_stmt!(
      manager,
      r#"alter table namespaces alter column created_at set default now()"#
    )?;

    // Create tg__timestamps trigger to set automatically created_at and updated_at
    exec_stmt!(
      manager,
      r#"drop function if exists tg__create_missing_namespace();"#
    )?;
    exec_stmt!(
      manager,
      r#"
        create or replace function tg__timestamps() returns trigger as $$
          begin
            NEW.created_at = (case when TG_OP = 'INSERT' then NOW() else OLD.created_at end);
            NEW.updated_at = (case when TG_OP = 'UPDATE' and OLD.updated_at >= NOW() then OLD.updated_at + interval '1 millisecond' else NOW() end);
            return NEW;
          end;
        $$ language plpgsql volatile set search_path to pg_catalog, public, pg_temp;
      "#
    )?;
    exec_stmt!(
      manager,
      r#"create trigger _100_timestamps before insert or update on namespaces for each row execute procedure tg__timestamps();"#
    )?;

    Ok(())
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().table(Entity).to_owned())
      .await
  }
}
