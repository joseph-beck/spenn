use actix_web::{get, HttpResponse, Responder};
use spenn_entity::mac;

#[get("/api/v1")]
async fn get_root() -> impl Responder {
    HttpResponse::Ok().body(format!("healthy"))
}

#[get("/api/v1/macs")]
async fn list_mac() -> impl Responder {
    HttpResponse::Ok().json(mac::Model::new("hello".to_string()))
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

    #[actix_web::test]
    async fn test_list_mac() {
        let app = test::init_service(App::new().service(list_mac)).await;
        let req = test::TestRequest::get().uri("/api/v1/macs").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(
            resp.status().is_success(),
            "status code {:?}",
            resp.status()
        );
    }
}
