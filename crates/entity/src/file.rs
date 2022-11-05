use getset::Getters;
use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, DeriveEntityModel, Deserialize, Serialize, Getters)]
#[sea_orm(table_name = "files")]
#[getset(get = "pub")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    #[sea_orm(column_type = "Text")]
    pub namespace: String,
    #[sea_orm(column_type = "Text")]
    pub storage_key: String,
    pub tags: Vec<String>,
    pub metadata: Json,
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
}

impl Related<crate::namespace::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Namespace.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
