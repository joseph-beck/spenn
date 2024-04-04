use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "macs")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[sea_orm(column_type = "Uuid")]
    pub uuid: Uuid,
    #[sea_orm(column_type = "Text")]
    pub address: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn new(address: String) -> Model {
        Model {
            uuid: Uuid::new_v4(),
            address,
        }
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
}
