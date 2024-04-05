use actix_web::{get, post, web, HttpResponse, Responder};
use spenn_entity::{expense, mac};

use crate::db::{ping_db, sqlite_conn};

#[get("/api/v1")]
async fn get_root() -> impl Responder {
    HttpResponse::Ok().body(format!("healthy"))
}

#[get("/api/v1/pings")]
async fn get_ping() -> impl Responder {
    let conn = sqlite_conn().await.unwrap();
    match ping_db(&conn).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_err) => HttpResponse::BadRequest(),
    }
}

#[get("/api/v1/macs")]
async fn list_mac() -> impl Responder {
    HttpResponse::Ok().json(mac::Model::new("hello".to_string(), "hello".to_string()))
}

#[post("/api/v1/macs")]
async fn post_mac(body: web::Json<mac::Request>) -> impl Responder {
    println!("{:?}, {:?}", body, body.to_model());

    HttpResponse::NoContent()
}

#[get("/api/v1/expenses")]
async fn list_expenses() -> impl Responder {
    let expense = expense::Model::default();

    HttpResponse::Ok().json(expense)
}

#[get("/api/v1/expenses/{uuid}")]
async fn get_expense() -> impl Responder {
    let expense = expense::Model::default();

    HttpResponse::Ok().json(expense)
}

#[post("/api/v1/expenses")]
async fn post_expense(body: web::Json<expense::Request>) -> impl Responder {
    println!("{:?}, {:?}", body, body.to_model());

    HttpResponse::NoContent()
}

#[cfg(test)]
mod tests {
    use actix_web::{test, App};
    use dotenv::dotenv;
    use std::env;

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
    async fn test_get_ping() {
        dotenv().ok();
        env::set_var("DATABASE_URL", "sqlite::memory:");

        let app = test::init_service(App::new().service(get_ping)).await;
        let req = test::TestRequest::get().uri("/api/v1/pings").to_request();
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

    #[actix_web::test]
    async fn test_post_mac() {
        let app = test::init_service(App::new().service(post_mac)).await;

        let body = mac::Request {
            name: "Name".to_string(),
            address: "Address".to_string(),
        };

        let req = test::TestRequest::post()
            .uri("/api/v1/macs")
            .set_json(&body)
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert!(
            resp.status().is_success(),
            "status code {:?}",
            resp.status()
        );
    }

    #[actix_web::test]
    async fn test_list_expenses() {
        let app = test::init_service(App::new().service(list_expenses)).await;
        let req = test::TestRequest::get()
            .uri("/api/v1/expenses")
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert!(
            resp.status().is_success(),
            "status code {:?}",
            resp.status()
        );
    }

    #[actix_web::test]
    async fn test_get_expense() {
        let app = test::init_service(App::new().service(get_expense)).await;
        let req = test::TestRequest::get()
            .uri("/api/v1/expenses/123")
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert!(
            resp.status().is_success(),
            "status code {:?}",
            resp.status()
        );
    }

    #[actix_web::test]
    async fn test_post_expense() {
        let app = test::init_service(App::new().service(post_expense)).await;

        let body = expense::Request {
            name: "name".to_string(),
            expense_type: 0,
            amount: 0,
            description: "description".to_string(),
        };

        let req = test::TestRequest::post()
            .uri("/api/v1/expenses")
            .set_json(&body)
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert!(
            resp.status().is_success(),
            "status code {:?}",
            resp.status()
        );
    }
}
