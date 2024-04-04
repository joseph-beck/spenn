use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "macs")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[sea_orm(column_type = "Uuid")]
    pub uuid: Uuid,
    #[sea_orm(column_type = "Text")]
    pub name: String,
    pub expense_type: u64,
    pub amount: u64,
    #[sea_orm(column_type = "Text")]
    pub description: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn new(name: String) -> Model {
        Model {
            uuid: Uuid::new_v4(),
            name,
            expense_type: 1,
            amount: 0,
            description: "description".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Request {
    pub name: String,
    pub expense_type: u64,
    pub amount: u64,
    pub description: String,
}

impl Request {
    pub fn new(name: String) -> Request {
        Request {
            name,
            expense_type: 1,
            amount: 0,
            description: "description".to_string(),
        }
    }

    pub fn to_model(&self) -> Model {
        Model::new(self.name.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_new() {
        let one = Model::new("one".to_string());
        let two = Model::new("two".to_string());
        assert_ne!(one.uuid, two.uuid)
    }

    #[test]
    fn test_request_new() {
        let one = Request::new("one".to_string());
        let two = Request::new("two".to_string());
        assert_ne!(one, two);

        assert!(!(one == two));
    }

    #[test]
    fn test_request_to_model() {
        let one = Request::new("one".to_string());
        let model = one.to_model();

        assert_ne!(model, Model::new("one".to_string()));
    }
}
