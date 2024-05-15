use chrono::{DateTime, Local};
use sea_orm::{entity::prelude::*, Schema, Set};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[sea_orm(column_type = "Text")]
    pub uuid: Uuid,
    #[sea_orm(column_type = "Text")]
    pub name: String,
    #[sea_orm(column_type = "Text")]
    pub password: String,
    #[sea_orm(column_type = "DateTime")]
    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::NaiveDateTime>,
}

impl Model {
    pub async fn migrate(db: &DbConn) -> Result<(), DbErr> {
        let backend = db.get_database_backend();
        let schema = Schema::new(backend);
        let mut create = schema.create_table_from_entity(self::Entity);

        match db.execute(backend.build(create.if_not_exists())).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
