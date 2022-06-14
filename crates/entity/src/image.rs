use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "images")]
pub struct Model {
  #[sea_orm(primary_key)]
  #[serde(skip_deserializing)]
  pub id: i32,
  #[sea_orm(column_type = "Text")]
  pub namespace: String,
  #[sea_orm(column_type = "Text", unique)]
  pub storage_key: String,
  #[sea_orm(unique)]
  pub lazy_image_id: Option<i32>,
  #[sea_orm(column_type = "Text")]
  pub alt: Option<String>,
  pub created_at: DateTimeUtc,
  pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(
    belongs_to = "crate::namespace::Entity",
    from = "Column::Namespace",
    to = "crate::namespace::Column::Name"
  )]
  Namespace,
  #[sea_orm(belongs_to = "Entity", from = "Column::LazyImageId", to = "Column::Id")]
  LazyImage,
}

impl Related<crate::namespace::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Namespace.def()
  }
}

pub struct LazyImageLink;

impl Linked for LazyImageLink {
  type FromEntity = Entity;

  type ToEntity = Entity;

  fn link(&self) -> Vec<RelationDef> {
    vec![Relation::LazyImage.def()]
  }
}

impl ActiveModelBehavior for ActiveModel {}
