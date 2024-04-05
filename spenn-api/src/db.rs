use std::env;

use sea_orm::{prelude::*, Database};

pub async fn sqlite_conn() -> Result<DbConn, DbErr> {
    let url = env::var("DATABASE_URL").expect("DB_URL is not set in .env file");
    match Database::connect(&url).await {
        Ok(db) => Ok(db),
        Err(err) => Err(err),
    }
}

pub async fn ping_db(db: &DbConn) -> Result<(), DbErr> {
    db.ping().await
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;

    #[actix_web::test]
    async fn test_sqlite_conn() {
        dotenv().ok();
        env::set_var("DATABASE_URL", "sqlite::memory:");

        let res = sqlite_conn().await;
        assert!(res.is_ok());
    }

    #[actix_web::test]
    async fn test_ping_db() {
        dotenv().ok();
        env::set_var("DATABASE_URL", "sqlite::memory:");

        let conn = sqlite_conn().await.unwrap();
        let res = ping_db(&conn).await;
        assert!(res.is_ok());
    }
}
