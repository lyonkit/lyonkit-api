use crate::utils::macros::exec_stmt;
use sea_orm_migration::{prelude::*, MigrationName};

pub struct Migration;

impl MigrationName for Migration {
  fn name(&self) -> &str {
    "m20220716_000007_fix_blok_priority_trigger"
  }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    exec_stmt!(
      manager,
      r#"
        create or replace function tg_bloks__set_priority() returns trigger as $$
          declare
            max_priority integer;
          begin
            -- Set default priority to maximum
            if new.priority is null or new.priority = 0 then
              select max(b1.priority) into max_priority from bloks b1 where b1.page_id = new.page_id;
              new.priority := coalesce(max_priority + 1, 1);
            end if;
        
            -- Offset other priorities if conflict
            update bloks b2 set priority = b2.priority + 1 where b2.page_id = new.page_id and b2.priority = new.priority and b2.id != new.id;
        
            return new;
          end;
        $$ language plpgsql volatile;
      "#
    )?;

    Ok(())
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    exec_stmt!(
      manager,
      r#"
        create or replace function tg_bloks__set_priority() returns trigger as $$
          declare
            max_priority integer;
          begin
            -- Set default priority to maximum
            if new.priority is null or new.priority = 0 then
              select max(b1.priority) into max_priority from bloks b1 where b1.page_id = new.page_id;
              new.priority := coalesce(max_priority + 1, 0);
            end if;
        
            -- Offset other priorities if conflict
            update bloks b2 set priority = b2.priority + 1 where b2.page_id = new.page_id and b2.priority = new.priority and b2.id != new.id;
        
            return new;
          end;
        $$ language plpgsql volatile;
      "#
    )?;

    Ok(())
  }
}
