use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "pages")]
pub struct Model {
  #[sea_orm(primary_key)]
  #[serde(skip_deserializing)]
  pub id: i32,
  #[sea_orm(column_type = "Text")]
  pub namespace: String,
  #[sea_orm(column_type = "Text")]
  pub path: String,
  pub title: String,
  pub description: Option<String>,
  pub created_at: DateTimeUtc,
  pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(has_many = "crate::blok::Entity")]
  Blok,
  #[sea_orm(
    belongs_to = "crate::namespace::Entity",
    from = "Column::Namespace",
    to = "crate::namespace::Column::Name"
  )]
  Namespace,
}

impl ActiveModelBehavior for ActiveModel {}
