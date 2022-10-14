use crate::utils::macros::{create_table_from_entity, exec_stmt};
use entity::locale::Entity;
use entity::locale_data::Entity as DataEntity;
use sea_orm_migration::{prelude::*, MigrationName};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20221013_000013_create_locale_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        exec_stmt!(manager, r#"drop table if exists locales"#)?;
        exec_stmt!(manager, r#"drop table if exists locales_data"#)?;
        create_table_from_entity!(manager, DataEntity)?;
        create_table_from_entity!(manager, Entity)?;

        // Set default value for created_at / updated_at columns and adds constraint
        exec_stmt!(
            manager,
            r#"alter table locales
                alter column created_at set default now(),
                alter column updated_at set default now(),
                drop constraint if exists "fk-locales-namespace",
                add constraint "fk-locales-namespace"
                    foreign key (namespace)
                    references namespaces (name)
                    on update cascade
                    on delete cascade,
                drop constraint if exists "fk-locales-locale_data_id",
                add constraint "fk-locales-locale_data_id"
                    foreign key (locale_data_id)
                    references locales_data
                    on update cascade
                    on delete cascade,
                add constraint "locale-namespace-lang-uniq-idx"
                    unique (namespace, lang)
            "#
        )?;
        exec_stmt!(
            manager,
            r#"alter table locales_data
                alter column messages type jsonb
            "#
        )?;

        // Trigger for timestamps
        exec_stmt!(
            manager,
            r#"create trigger _100_timestamps
                before insert or update on locales
                for each row execute procedure tg__timestamps();
            "#
        )?;
        exec_stmt!(
            manager,
            r#"create trigger _500_create_missing_namespace
                before insert or update on locales
                for each row execute procedure public.tg__create_missing_namespace();
            "#
        )?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Entity).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(DataEntity).to_owned())
            .await?;

        Ok(())
    }
}
