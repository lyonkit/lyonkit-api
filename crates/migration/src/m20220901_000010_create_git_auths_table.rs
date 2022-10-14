use crate::utils::macros::{create_table_from_entity, exec_stmt};
use entity::git_auth::Entity;
use sea_orm_migration::{prelude::*, MigrationName};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220901_000010_create_git_auths_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        exec_stmt!(manager, r#"drop table if exists git_auths"#)?;

        create_table_from_entity!(manager, Entity)?;

        // Set default value for created_at / updated_at columns and adds constraint
        exec_stmt!(
            manager,
            r#"alter table git_auths 
         alter column created_at set default now(), 
         alter column updated_at set default now(),
         alter column editable_files type jsonb,
         add constraint editable_files_is_array check (jsonb_typeof(editable_files) = 'array'),
         drop constraint if exists "fk-git_auths-namespace",
         add constraint "fk-git_auths-namespace" foreign key (namespace) references namespaces (name) on update cascade on delete cascade
      "#
        )?;

        // Triggers
        exec_stmt!(
            manager,
            r#"create trigger _100_timestamps before insert or update on git_auths for each row execute procedure tg__timestamps();"#
        )?;
        exec_stmt!(
            manager,
            r#"create trigger _500_create_missing_namespace before insert or update on git_auths for each row execute procedure public.tg__create_missing_namespace();"#
        )?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Entity).to_owned())
            .await?;

        Ok(())
    }
}
