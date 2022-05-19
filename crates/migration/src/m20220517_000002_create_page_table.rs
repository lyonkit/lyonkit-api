use crate::utils::macros::{create_table_from_entity, exec_stmt};
use entity::page::Entity;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
  fn name(&self) -> &str {
    "m20220517_000001_create_page_table"
  }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    exec_stmt!(manager, r#"drop table if exists pages"#)?;
    create_table_from_entity!(manager, Entity)?;

    // Set default value for created_at / updated_at columns
    exec_stmt!(
      manager,
      r#"alter table pages 
        alter column created_at set default now(), 
        alter column updated_at set default now(),
        drop constraint if exists "fk-pages-namespace",
        add constraint "fk-pages-namespace" foreign key (namespace) references namespaces (name) on update cascade on delete cascade
      "#
    )?;

    // Create unique indexe
    exec_stmt!(
      manager,
      r#"create unique index page__namespace_path__uniq_idx on pages(namespace, path);"#
    )?;

    // Create path constraint
    exec_stmt!(
      manager,
      r#"alter table pages add constraint valid_path check (path ~* '^\/(?:[a-z0-9-]+\/)*[a-z0-9-]+$');"#
    )?;

    // Trigger to insert missing namespace
    exec_stmt!(
      manager,
      r#"drop function if exists tg__create_missing_namespace()"#
    )?;
    exec_stmt!(
      manager,
      r#"
        create function tg__create_missing_namespace() returns trigger as $$
          declare
            namespace_id integer;
          begin
            -- Create if missing
            insert into namespaces (name) values (new.namespace) on conflict (name) do nothing;
        
            return new;
          end;
        $$ language plpgsql volatile;
      "#
    )?;

    exec_stmt!(
      manager,
      r#"create trigger _500_create_missing_namespace before insert or update on pages for each row execute procedure public.tg__create_missing_namespace();"#
    )?;
    exec_stmt!(
      manager,
      r#"create trigger _100_timestamps before insert or update on pages for each row execute procedure tg__timestamps();"#
    )?;

    Ok(())
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().table(Entity).to_owned())
      .await
  }
}
