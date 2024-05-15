use sea_orm::{DbConn, DbErr};

pub mod expense;
pub mod mac;

pub trait Migrator {
    fn migrate(db: &DbConn) -> impl std::future::Future<Output = Result<(), DbErr>> + Send;
}
