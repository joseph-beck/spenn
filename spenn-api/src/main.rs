use actix_web::{middleware::Logger, App, HttpServer};
use dotenv::dotenv;
use env_logger::init;
use std::env;

mod db;
mod models;
mod schema;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    init();

    let _conn = db::establish_connection();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(services::get_root)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
