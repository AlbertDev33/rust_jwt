use std::env::{set_var, var_os};
use std::io::Result;

use actix_cors::Cors;
use actix_web::http::header;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

mod config;
mod handlers;
mod jwt_auth;
mod models;

use config::Config;
use handlers::routes;

pub struct AppState {
    db: Pool<Postgres>,
    env: Config,
}

const PORT: u16 = 3333;

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
            print!("✅Connection to the database is successful!\n");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    println!("🚀 Server started successfully at the port {}", PORT);

    HttpServer::new(move || {
        Cors::default()
            .allowed_origin("*")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();
        App::new()
            .app_data(web::Data::new(AppState {
                db: pool.clone(),
                env: config.clone(),
            }))
            .configure(routes::config)
            .wrap(Logger::default())
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", PORT))?
    .run()
    .await
}
