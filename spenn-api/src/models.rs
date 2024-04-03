use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Insertable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = crate::schema::macs)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Mac {
    pub uuid: Uuid,
    pub address: String,
}
impl Mac {
    pub fn new(address: String) -> Mac {
        Mac {
            uuid: Uuid::new_v4(),
            address
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mac_new() {
        let one = Mac::new("one".to_string());
        let two = Mac::new("two".to_string());
        assert_ne!(one.uuid, two.uuid)
    }
}
