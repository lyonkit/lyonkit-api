use crate::utils::macros::exec_stmt;
use sea_orm_migration::{prelude::*, MigrationName};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220917_000012_fix_blok_priority_trigger"
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
            updated_ids integer[];
          begin          
            set constraints all deferred;
            
            -- Set default priority to maximum
            if new.priority is null then
              select max(b1.priority) into max_priority from bloks b1 where b1.page_id = new.page_id;
              new.priority := coalesce(max_priority + 1, 0);
            end if;
            
            if (tg_op = 'INSERT' or (tg_op = 'UPDATE' and old.priority != new.priority)) and exists(select 1 from bloks where page_id = new.page_id and priority = new.priority) then
              with t as (update bloks b2 set priority = b2.priority + 1 where b2.page_id = new.page_id and b2.priority >= new.priority and b2.id is distinct from new.id returning id)         
              select array_agg(id) into updated_ids from t;
              
              raise notice 'Offset priority for rows IDS : %', updated_ids;
            end if;
        
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
            updated_ids integer[];
          begin          
            set constraints all deferred;
            
            -- Set default priority to maximum
            if new.priority is null or new.priority = 0 then
              select max(b1.priority) into max_priority from bloks b1 where b1.page_id = new.page_id;
              new.priority := coalesce(max_priority + 1, 0);
            end if;
            
            if exists(select 1 from bloks where page_id = new.page_id and priority = new.priority) then
              with t as (update bloks b2 set priority = b2.priority + 1 where b2.page_id = new.page_id and b2.priority >= new.priority and b2.id is distinct from new.id returning id)         
              select array_agg(id) into updated_ids from t;
              
              raise notice 'Offset priority for rows IDS : %', updated_ids;
            end if;
        
            return new;
          end;
        $$ language plpgsql volatile;
      "#
        )?;

        Ok(())
    }
}
