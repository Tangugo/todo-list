use sea_orm::entity::prelude::*;
use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "todos")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    #[sea_orm(column_type = "Boolean", default_value = "false")]
    pub completed: bool,
    #[sea_orm(column_type = "TimestampWithTimeZone", default_value = "CURRENT_TIMESTAMP")]
    pub created_at: DateTime<Utc>,
    #[sea_orm(column_type = "String(Some(255))", default_value = "''")]
    pub image_name: String,
    #[sea_orm(column_type = "Binary", default_value = "decode('','escape')")]
    pub image_data: Vec<u8>,
    #[sea_orm(column_type = "Json", default_value = "{}")]
    pub extra: serde_json::Value,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}