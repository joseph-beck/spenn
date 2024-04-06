use sea_orm::{entity::prelude::*, Schema};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "macs")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[sea_orm(column_type = "Text")]
    pub uuid: Uuid,
    #[sea_orm(column_type = "Text")]
    pub name: String,
    #[sea_orm(column_type = "Text")]
    pub address: String,
}

impl Model {
    pub fn new(name: String, address: String) -> Model {
        Model {
            uuid: Uuid::new_v4(),
            name,
            address,
        }
    }

    pub fn default() -> Model {
        Model {
            uuid: Uuid::new_v4(),
            name: "name".to_string(),
            address: "address".to_string(),
        }
    }

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
    pub name: String,
    pub address: String,
}

impl Request {
    pub fn new(name: String, address: String) -> Request {
        Request { name, address }
    }

    pub fn to_model(&self) -> Model {
        Model::new(self.name.to_string(), self.address.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;
    use sea_orm::Database;
    use std::env;

    async fn conn() -> DbConn {
        dotenv().ok();
        env::set_var("DATABASE_URL", "sqlite::memory:");

        let url = env::var("DATABASE_URL").expect("DB_URL is not set in .env file");
        Database::connect(&url).await.unwrap()
    }

    #[test]
    fn test_model_new() {
        let one = Model::new("one".to_string(), "one".to_string());
        let two = Model::new("two".to_string(), "two".to_string());
        assert_ne!(one.uuid, two.uuid);

        assert!(!(one == two));
    }

    #[test]
    fn test_model_default() {
        let one = Model::default();
        let two = Model::default();
        assert_ne!(one.uuid, two.uuid)
    }

    #[actix_web::test]
    async fn test_model_migrate() {
        let conn = conn().await;
        let res = Model::migrate(&conn).await;
        assert!(res.is_ok())
    }

    #[test]
    fn test_request_new() {
        let one = Request::new("one".to_string(), "one".to_string());
        let two = Request::new("two".to_string(), "two".to_string());
        assert_ne!(one, two);

        assert!(!(one == two));
    }

    #[test]
    fn test_request_to_model() {
        let one = Request::new("one".to_string(), "one".to_string());
        let model = one.to_model();

        assert_ne!(model, Model::new("one".to_string(), "one".to_string()));
    }
}
