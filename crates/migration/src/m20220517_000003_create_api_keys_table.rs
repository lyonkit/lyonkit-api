use crate::utils::macros::{create_table_from_entity, exec_stmt};
use entity::api_key::Entity;
use sea_orm_migration::{prelude::*, MigrationName, MigrationTrait, SchemaManager};

pub struct Migration;

impl MigrationName for Migration {
  fn name(&self) -> &str {
    "m20220517_000002_create_api_keys_table"
  }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    exec_stmt!(manager, r#"drop table if exists api_keys"#)?;
    create_table_from_entity!(manager, Entity)?;

    // Set default value for created_at / updated_at columns
    exec_stmt!(
      manager,
      r#"alter table api_keys 
        alter column created_at set default now(), 
        alter column updated_at set default now(), 
        alter column key set default uuid_generate_v4(),
        drop constraint if exists "fk-api_keys-namespace",
        add constraint "fk-api_keys-namespace" foreign key (namespace) references namespaces (name) on update cascade on delete cascade
      "#
    )?;

    exec_stmt!(
      manager,
      r#"create trigger _500_create_missing_namespace before insert or update on api_keys for each row execute procedure public.tg__create_missing_namespace();"#
    )?;
    exec_stmt!(
      manager,
      r#"create trigger _100_timestamps before insert or update on api_keys for each row execute procedure tg__timestamps();"#
    )?;

    Ok(())
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().table(Entity).to_owned())
      .await
  }
}
