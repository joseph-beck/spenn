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

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Request {
    pub uuid: Uuid,
    pub name: String,
    pub password: String,
    pub created_at: Option<chrono::NaiveDateTime>,
}

impl Request {
    pub fn new(name: String, password: String) -> Request {
        Request {
            uuid: Uuid::new_v4(),
            name,
            password,
            created_at: Some(Local::now().naive_local()),
        }
    }

    pub fn to_model(&self) -> Model {
        Model {
            uuid: self.uuid,
            name: self.name.to_string(),
            password: self.password.to_string(),
            created_at: self.created_at,
        }
    }

    pub fn to_active_model(&self) -> ActiveModel {
        ActiveModel {
            uuid: Set(Uuid::new_v4().to_owned()),
            name: Set(self.name.to_owned()),
            password: Set(self.password.to_owned()),
            created_at: Set(self.created_at.to_owned())
        }
    }
}

#[cfg(test)]
mod tests {

}
