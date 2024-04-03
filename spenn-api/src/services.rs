use actix_web::{get, HttpResponse, Responder};

use crate::db;

#[get("/api/v1")]
async fn get_root() -> impl Responder {
    HttpResponse::Ok().body(format!("healthy"))
}

#[get("/api/v1/mac")]
async fn get_mac() -> impl Responder {
    HttpResponse::Ok().json(db::get_mac())
}

#[cfg(test)]
mod tests {
    use actix_web::{test, App};

    use super::*;

    #[actix_web::test]
    async fn test_get_root() {
        let app = test::init_service(App::new().service(get_root)).await;
        let req = test::TestRequest::get().uri("/api/v1").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(
            resp.status().is_success(),
            "status code {:?}",
            resp.status()
        );
    }
}

