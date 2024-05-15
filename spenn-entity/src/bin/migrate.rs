use dotenv::dotenv;
use sea_orm::Database;
use spenn_entity::{expense, Migrator};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db = Database::connect(&url).await.unwrap();

    expense::Model::migrate(&db)
        .await
        .expect("failed to migrate expense");

    Ok(())
}
