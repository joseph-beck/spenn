use actix_web::{App, HttpServer};

mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(services::get_root))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
