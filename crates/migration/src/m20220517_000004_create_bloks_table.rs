use crate::utils::macros::{create_table_from_entity, exec_stmt};
use entity::blok::Entity;
use sea_orm_migration::{prelude::*, MigrationName, MigrationTrait, SchemaManager};

pub struct Migration;

impl MigrationName for Migration {
  fn name(&self) -> &str {
    "m20220517_000003_create_bloks_table"
  }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    exec_stmt!(manager, r#"drop table if exists bloks"#)?;
    create_table_from_entity!(manager, Entity)?;

    // Set default value for created_at / updated_at columns and adds constraint
    exec_stmt!(
      manager,
      r#"alter table bloks 
         alter column created_at set default now(), 
         alter column updated_at set default now(), 
         alter column props type jsonb,
         add constraint valid_content check (jsonb_typeof(props) = 'object'),
         drop constraint if exists "fk-bloks-page_id",
         add constraint "fk-bloks-page_id" foreign key (page_id) references pages on update cascade on delete cascade
      "#
    )?;
    // Unique index on (page_id, priority)
    exec_stmt!(
      manager,
      r#"create unique index bloks__page_id_priority__uniq_idx on bloks(page_id, priority);"#
    )?;

    // Trigger to offset priority of other bloks
    exec_stmt!(
      manager,
      r#"drop function if exists tg_bloks__set_priority()"#
    )?;
    exec_stmt!(
      manager,
      r#"
        create function tg_bloks__set_priority() returns trigger as $$
          declare
            max_priority integer;
          begin
            -- Set default priority to maximum
            if new.priority is null or new.priority = 0 then
              select max(b1.priority) into max_priority from bloks b1 where b1.page_id = new.page_id;
              new.priority := coalesce(max_priority + 1, 0);
            end if;
        
            -- Offset other priorities if conflict
            update bloks b2 set priority = b2.priority + 1 where b1.page_id = new.page_id and b2.priority = new.priority and b2.id != new.id;
        
            return new;
          end;
        $$ language plpgsql volatile;
      "#
    )?;
    exec_stmt!(
      manager,
      r#"create trigger _500_set_display_priority before insert or update on bloks for each row execute procedure public.tg_bloks__set_priority();"#
    )?;
    exec_stmt!(
      manager,
      r#"create trigger _100_timestamps before insert or update on bloks for each row execute procedure tg__timestamps();"#
    )?;

    Ok(())
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().table(Entity).to_owned())
      .await?;

    exec_stmt!(
      manager,
      r#"drop function if exists tg_bloks__set_priority();"#
    )?;

    Ok(())
  }
}
