use std::{f32::consts::E, str::FromStr};

use actix_web::{get, post, web, HttpResponse, Responder};
use sea_orm::{EntityTrait, QueryOrder};
use spenn_entity::{expense, mac};
use uuid::Uuid;

use crate::{
    db::{ping_db, sqlite_conn},
    AppState,
};

#[get("/api/v1")]
async fn get_root() -> impl Responder {
    HttpResponse::Ok().body(format!("healthy"))
}

#[get("/api/v1/pings")]
async fn get_ping(app_state: web::Data<AppState>) -> impl Responder {
    match app_state.db_pool.ping().await {
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
async fn list_expenses(app_state: web::Data<AppState>) -> impl Responder {
    let result = expense::Entity::find()
        .order_by_asc(expense::Column::Name)
        .all(app_state.db_pool.get_conn().await.unwrap())
        .await;

    match result {
        Ok(expenses) => HttpResponse::Ok().json(expenses),
        Err(err) => {
            HttpResponse::BadRequest().body(format!("error fetching from the database {:?}", err))
        }
    }
}

#[get("/api/v1/expenses/{uuid}")]
async fn get_expense(app_state: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let uuid = path.into_inner();
    let result = expense::Entity::find_by_id(Uuid::from_str(&uuid).unwrap())
        .one(app_state.db_pool.get_conn().await.unwrap())
        .await;

    match result {
        Ok(option) => match option {
            Some(expense) => HttpResponse::Ok().json(expense),
            None => HttpResponse::NoContent().body("no "),
        },
        Err(err) => {
            HttpResponse::BadRequest().body(format!("error fetching from the database {:?}", err))
        }
    }
}

#[post("/api/v1/expenses")]
async fn post_expense(
    app_state: web::Data<AppState>,
    body: web::Json<expense::Request>,
) -> impl Responder {
    let active_model = body.to_active_model();
    let result = expense::Entity::insert(active_model)
        .exec(app_state.db_pool.get_conn().await.unwrap())
        .await;

    match result {
        Ok(_) => HttpResponse::NoContent(),
        Err(_err) => HttpResponse::BadRequest(),
    }
}

#[cfg(test)]
mod tests {
    use actix_web::{test, web::Data, App};
    use dotenv::dotenv;
    use std::env;

    use super::*;

    fn before() {
        dotenv().ok();
        env::set_var("DATABASE_URL", "sqlite::memory:");
    }

    #[actix_web::test]
    async fn test_get_root() {
        before();

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
        before();

        let state = AppState::new().await;
        let app = test::init_service(
            App::new()
                .app_data(Data::new(state.clone()))
                .service(get_ping),
        )
        .await;
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
        before();

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
        before();

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
        before();

        let state = AppState::new().await;
        expense::Model::migrate(state.db_pool.get_conn().await.unwrap())
            .await
            .expect("failed to migrate model");
        let app = test::init_service(
            App::new()
                .app_data(Data::new(state.clone()))
                .service(list_expenses),
        )
        .await;
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
        before();

        let state = AppState::new().await;
        expense::Model::migrate(state.db_pool.get_conn().await.unwrap())
            .await
            .expect("failed to migrate model");
        let app = test::init_service(
            App::new()
                .app_data(Data::new(state.clone()))
                .service(get_expense),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/api/v1/expenses/67e55044-10b1-426f-9247-bb680e5fe0c8")
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
        before();

        let state = AppState::new().await;
        expense::Model::migrate(state.db_pool.get_conn().await.unwrap())
            .await
            .expect("failed to migrate model");
        let app = test::init_service(
            App::new()
                .app_data(Data::new(state.clone()))
                .service(post_expense),
        )
        .await;

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
