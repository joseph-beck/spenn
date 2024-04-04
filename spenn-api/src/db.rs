use std::env;

use sea_orm::{prelude::*, Database};

pub async fn establish_conn() -> DatabaseConnection {
    let url = env::var("DB_URL").expect("DB_URL is not set in .env file");
    Database::connect(&url).await.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_establish_conn() {
        let _conn = establish_conn();
    }
}
