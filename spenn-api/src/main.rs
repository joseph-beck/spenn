use actix_web::{
    middleware::Logger,
    web::{self},
    App, HttpServer,
};
use db::sqlite_conn;
use dotenv::dotenv;
use spenn_entity::expense;
use std::env;

mod db;
mod services;

fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(services::get_root);
    cfg.service(services::get_ping);
    cfg.service(services::list_mac);
    cfg.service(services::post_mac);
    cfg.service(services::list_expenses);
    cfg.service(services::get_expense);
    cfg.service(services::post_expense);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    let conn = sqlite_conn().await.unwrap();
    let _migrate = expense::Model::migrate(&conn)
        .await
        .expect("failed to migrate model");

    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");

    HttpServer::new(|| App::new().wrap(Logger::default()).configure(init))
        .bind(format!("{host}:{port}"))?
        .run()
        .await
}
