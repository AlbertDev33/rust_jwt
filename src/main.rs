use std::env::{set_var, var_os};
use std::io::Result;

use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use serde_json::json;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

mod config;
mod handlers;
mod jwt_auth;
mod models;

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
    dotenv().ok();
    env_logger::init();

    let config = Config::init();
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
    {
        Ok(pool) => {
            print!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    println!("🚀 Server started successfully at the port {}", PORT);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                db: pool.clone(),
                env: config.clone(),
            }))
            .wrap(Logger::default())
            .service(health_checker_handler)
    })
    .bind(("127.0.0.1", PORT))?
    .run()
    .await
}
