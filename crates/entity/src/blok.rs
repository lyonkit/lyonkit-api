use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "bloks")]
pub struct Model {
  #[sea_orm(primary_key)]
  #[serde(skip_deserializing)]
  pub id: i32,
  pub page_id: i32,
  #[sea_orm(column_type = "Text")]
  pub component_id: String,
  pub props: Json,
  #[sea_orm(default_value = "0")]
  pub priority: i32,
  pub created_at: DateTimeUtc,
  pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(
    belongs_to = "crate::page::Entity",
    from = "Column::PageId",
    to = "crate::page::Column::Id"
  )]
  Page,
}

impl Related<crate::page::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Page.def()
  }
}

impl ActiveModelBehavior for ActiveModel {}
