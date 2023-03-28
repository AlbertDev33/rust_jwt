use actix_web::{post, web, HttpResponse, Responder};
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use rand_core::OsRng;
use serde_json;
use sqlx::{query, Row};

use crate::models::users::{RegisterUserSchema, User};
use crate::AppState;

use super::filter_user::filter_user_record;

#[post("/auth/register")]
async fn register_user_handler(
    body: web::Json<RegisterUserSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let exists: bool = query("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)")
        .bind(body.email.to_owned())
        .fetch_one(&data.db)
        .await
        .unwrap()
        .get(0);

    if exists {
        return HttpResponse::Conflict().json(serde_json::json!({ "status": "fail", "message": "User with that email already exists" }));
    }

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(body.password.as_bytes(), &salt)
        .expect("Error while hashing password")
        .to_string();

    let query_result = sqlx::query_as!(
        User,
        "INSERT INTO users (name,email,password) VALUES ($1, $2, $3) RETURNING *",
        body.name.to_string(),
        body.email.to_string().to_lowercase(),
        hashed_password
    )
    .fetch_one(&data.db)
    .await;

    // let query_result = sqlx::query!(
    //     "INSERT INTO users (name,email,password) VALUES ($1, $2, $3) RETURNING *",
    //     body.name.to_string(),
    //     body.email.to_string().to_lowercase(),
    //     hashed_password
    // )
    // .fetch_one(&data.db)
    // .await
    // .map(|op| User {
    //     id: op.id,
    //     name: op.name,
    //     email: op.email,
    //     password: op.password,
    //     role: op.role,
    //     photo: op.photo,
    //     verified: op.verified,
    //     created_at: op.created_at,
    //     updated_at: op.updated_at,
    //     deleted_at: op.deleted_at,
    // });

    match query_result {
        Ok(user) => {
            let user_response = serde_json::json!({ "status": "success", "data": serde_json::json!({ "user": filter_user_record(&user) }) });
            return HttpResponse::Ok().json(user_response);
        }
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error", "message": format!("{:?}", e)}));
        }
    }
}
