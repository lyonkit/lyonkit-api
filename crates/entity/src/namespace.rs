use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "namespaces")]
pub struct Model {
  #[sea_orm(primary_key)]
  #[serde(skip_deserializing)]
  pub id: i32,
  #[sea_orm(column_type = "Text", unique)]
  pub name: String,
  pub created_at: DateTimeUtc,
  pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(has_many = "crate::api_key::Entity")]
  ApiKey,
}

impl ActiveModelBehavior for ActiveModel {}
