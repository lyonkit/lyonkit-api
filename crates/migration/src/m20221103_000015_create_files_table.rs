use sea_orm_migration::{prelude::*, MigrationName};

use entity::file::Entity;

use crate::utils::macros::{create_table_from_entity, exec_stmt};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20221103_000015_create_files_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        exec_stmt!(manager, r#"drop table if exists files"#)?;
        create_table_from_entity!(manager, Entity)?;

        // Set default value for created_at / updated_at columns and adds constraint
        exec_stmt!(
            manager,
            r#"alter table files
                alter column created_at set default now(),
                alter column updated_at set default now(),
                add constraint files_storage_key_unique unique (storage_key),
                alter column tags type text[],
                alter column tags set default '{{}}'::text[],
                alter column metadata type jsonb,
                alter column metadata set default '{{}}'::jsonb,
                drop constraint if exists "fk-files-namespace",
                add constraint "fk-files-namespace"
                    foreign key (namespace)
                    references namespaces (name)
                    on update cascade
                    on delete cascade
            "#
        )?;
        exec_stmt!(manager, r#"create index tags_idx on files (tags)"#)?;
        exec_stmt!(manager, r#"create index metadata_idx on files (metadata)"#)?;

        // Trigger for timestamps
        exec_stmt!(
            manager,
            r#"create trigger _100_timestamps
                before insert or update on files
                for each row execute procedure tg__timestamps();
            "#
        )?;
        exec_stmt!(
            manager,
            r#"create trigger _500_create_missing_namespace
                before insert or update on files
                for each row execute procedure public.tg__create_missing_namespace();
            "#
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
