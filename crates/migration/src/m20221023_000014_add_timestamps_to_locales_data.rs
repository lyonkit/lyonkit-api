use crate::utils::macros::exec_stmt;
use sea_orm_migration::{prelude::*, MigrationName};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20221023_000014_add_timestamps_to_locales_data"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        exec_stmt!(
            manager,
            r#"alter table locales_data
                drop column if exists created_at,
                drop column if exists updated_at
            "#
        )?;

        exec_stmt!(
            manager,
            r#"alter table locales_data
                add column created_at timestamptz default now(),
                add column updated_at timestamptz default now()
            "#
        )?;
        exec_stmt!(
            manager,
            r#"create trigger _100_timestamps
                before insert or update on locales_data
                for each row execute procedure tg__timestamps();
            "#
        )?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        exec_stmt!(
            manager,
            r#"alter table locales_data
                drop column if exists created_at,
                drop column if exists updated_at
            "#
        )?;

        Ok(())
    }
}
