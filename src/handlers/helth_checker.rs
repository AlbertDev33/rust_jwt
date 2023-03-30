use actix_web::{get, HttpResponse, Responder};
use serde_json;

#[get("/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "JWT Authentication in Rust using Actix-web, Postgres and SQLX";
    let response = HttpResponse::Ok().json(serde_json::json!({ "status": "success", "message": MESSAGE }));
    return response;
}