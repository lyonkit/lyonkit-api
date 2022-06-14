use crate::utils::macros::{create_table_from_entity, exec_stmt};
use entity::image::Entity;
use sea_orm_migration::{prelude::*, MigrationName, MigrationTrait, SchemaManager};

pub struct Migration;

impl MigrationName for Migration {
  fn name(&self) -> &str {
    "m20220610_000005_create_images_table"
  }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    exec_stmt!(manager, r#"drop table if exists images"#)?;
    create_table_from_entity!(manager, Entity)?;

    // Set default value for created_at / updated_at columns and adds constraint
    exec_stmt!(
      manager,
      r#"alter table images 
         alter column created_at set default now(), 
         alter column updated_at set default now(),
         drop constraint if exists "fk-images-lazy_image_id",
         add constraint "fk-images-lazy_image_id" foreign key (lazy_image_id) references images on update cascade on delete cascade
      "#
    )?;

    // Trigger for timestamps
    exec_stmt!(
      manager,
      r#"create trigger _100_timestamps before insert or update on images for each row execute procedure tg__timestamps();"#
    )?;
    exec_stmt!(
      manager,
      r#"create trigger _500_create_missing_namespace before insert or update on images for each row execute procedure public.tg__create_missing_namespace();"#
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
