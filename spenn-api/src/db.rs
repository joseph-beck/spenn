use std::{env, time::Duration};

use sea_orm::{prelude::*, ConnectOptions, Database};

#[derive(Debug, Clone)]
pub struct DbPool {
    pub min_connections: u32,
    pub max_connections: u32,
    pub connect_timeout: Duration,
    pub idle_timeout: Duration,
    pub max_lifetime: Duration,
    pub sql_logging: bool,
    pub pool: Option<DbConn>,
}

impl DbPool {
    pub async fn new(timeout: u64, logging: bool) -> Result<DbPool, DbErr> {
        let mut pool = DbPool {
            min_connections: 1,
            max_connections: 1024,
            connect_timeout: Duration::from_secs(timeout),
            idle_timeout: Duration::from_secs(timeout),
            max_lifetime: Duration::from_secs(timeout),
            sql_logging: logging,
            pool: None,
        };

        match pool.conn().await {
            Ok(_) => Ok(pool),
            Err(err) => Err(err),
        }
    }

    pub async fn default() -> Result<DbPool, DbErr> {
        let mut pool = DbPool {
            min_connections: 1,
            max_connections: 1024,
            connect_timeout: Duration::from_secs(8),
            idle_timeout: Duration::from_secs(8),
            max_lifetime: Duration::from_secs(8),
            sql_logging: false,
            pool: None,
        };

        match pool.conn().await {
            Ok(_) => Ok(pool),
            Err(err) => Err(err),
        }
    }

    async fn conn(&mut self) -> Result<(), DbErr> {
        let url = env::var("DATABASE_URL").expect("DB_URL is not set in .env file");
        let mut opt = ConnectOptions::new(&url.to_owned());
        opt.min_connections(self.min_connections)
            .max_connections(self.max_connections)
            .connect_timeout(self.connect_timeout)
            .idle_timeout(self.idle_timeout)
            .sqlx_logging(self.sql_logging)
            .max_lifetime(self.max_lifetime);

        match Database::connect(opt).await {
            Ok(conn) => {
                self.pool = Some(conn);
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    pub async fn ping(&self) -> Result<(), DbErr> {
        match &self.pool {
            Some(pool) => pool.ping().await,
            None => Err(sea_orm::DbErr::Custom("no database pool".to_string())),
        }
    }

    pub async fn get_conn(&self) -> Result<&DbConn, DbErr> {
        match &self.pool {
            Some(pool) => Ok(pool),
            None => Err(sea_orm::DbErr::Custom("no database pool".to_string())),
        }
    }
}

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

    fn before() {
        dotenv().ok();
        env::set_var("DATABASE_URL", "sqlite::memory:");
    }

    #[actix_web::test]
    async fn test_db_pool_new() {
        before();

        let res = DbPool::new(8, false).await;
        assert!(res.is_ok());
    }

    #[actix_web::test]
    async fn test_db_pool_default() {
        before();

        let res = DbPool::default().await;
        assert!(res.is_ok());
    }

    #[actix_web::test]

    async fn test_db_pool_conn() {
        before();

        let mut pool = DbPool {
            min_connections: 1,
            max_connections: 1024,
            connect_timeout: Duration::from_secs(8),
            idle_timeout: Duration::from_secs(8),
            max_lifetime: Duration::from_secs(8),
            sql_logging: false,
            pool: None,
        };

        assert!(pool.pool.is_none());

        let res = pool.conn().await;
        assert!(res.is_ok());

        assert!(pool.pool.is_some());
    }

    #[actix_web::test]
    async fn test_db_ping() {
        before();

        let pool = DbPool::default().await.unwrap();
        let res = pool.ping().await;
        assert!(res.is_ok());
    }

    #[actix_web::test]

    async fn test_db_pool_get_conn() {
        before();

        let pool = DbPool::default().await.unwrap();
        let res = pool.get_conn().await;
        assert!(res.is_ok());
    }

    #[actix_web::test]
    async fn test_sqlite_conn() {
        before();

        let res = sqlite_conn().await;
        assert!(res.is_ok());
    }

    #[actix_web::test]
    async fn test_ping_db() {
        before();

        let conn = sqlite_conn().await.unwrap();
        let res = ping_db(&conn).await;
        assert!(res.is_ok());
    }
}
