use crate::utils::macros::exec_stmt;
use sea_orm_migration::{prelude::*, MigrationName};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220917_000011_fix_blok_priority_trigger"
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

        // Only execute trigger with one recursion limit
        exec_stmt!(
            manager,
            "drop trigger if exists _500_set_display_priority on bloks;"
        )?;
        exec_stmt!(
            manager,
            r#"create trigger _500_set_display_priority before insert or update on bloks for each row when (pg_trigger_depth() = 0) execute procedure public.tg_bloks__set_priority();"#
        )?;

        // Use constraint based uniqueness instead of index
        exec_stmt!(
            manager,
            r#"drop index if exists bloks__page_id_priority__uniq_idx;"#
        )?;
        exec_stmt!(
            manager,
            r#"alter table bloks drop constraint if exists page_id_priority__uniq, add constraint page_id_priority__uniq unique (page_id, priority) deferrable initially immediate;"#
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
            set constraints all deferred;
          
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

        exec_stmt!(
            manager,
            "drop trigger if exists _500_set_display_priority on bloks;"
        )?;
        exec_stmt!(
            manager,
            r#"create trigger _500_set_display_priority before insert or update on bloks for each row execute procedure public.tg_bloks__set_priority();"#
        )?;

        exec_stmt!(
            manager,
            "alter table bloks drop constraint if exists page_id_priority__uniq;"
        )?;
        exec_stmt!(
            manager,
            r#"create unique index if not exists bloks__page_id_priority__uniq_idx on bloks(page_id, priority);"#
        )?;

        Ok(())
    }
}
