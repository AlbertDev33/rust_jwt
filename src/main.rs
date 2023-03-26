use std::env::{set_var, var_os};
use std::io::Result;

use actix_web::middleware::Logger;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde_json::json;
use sqlx::{Pool, Postgres};

mod config;
mod models;
mod jwt_auth;

use config::Config;

pub struct AppState {
    db: Pool<Postgres>,
    env: Config,
}

const PORT: u16 = 3333;

#[get("/api/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "JWT Authentication in Rust using Actix-web, Postgres and SQLX";
    let response = HttpResponse::Ok().json(json!({ "status": "success", "message": MESSAGE }));
    return response;
}

#[actix_web::main]
async fn main() -> Result<()> {
    if var_os("RUST_LOG").is_none() {
        set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    println!("🚀 Server started successfully at the port {}", PORT);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(health_checker_handler)
    })
    .bind(("127.0.0.1", PORT))?
    .run()
    .await
}
