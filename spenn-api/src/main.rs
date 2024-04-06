use actix_web::{
    middleware::Logger,
    web::{self, Data},
    App, HttpServer,
};
use db::DbPool;
use dotenv::dotenv;
use std::env;

mod db;
mod services;

#[derive(Debug, Clone)]
struct AppState {
    pub app_name: String,
    pub db_pool: DbPool,
}

impl AppState {
    pub async fn new() -> AppState {
        AppState {
            app_name: "spenn".to_string(),
            db_pool: DbPool::default().await.unwrap(),
        }
    }
}

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

    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");

    let state = AppState::new().await;

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(state.clone()))
            .wrap(Logger::default())
            .configure(init)
    })
    .bind(format!("{host}:{port}"))?
    .run()
    .await
}
