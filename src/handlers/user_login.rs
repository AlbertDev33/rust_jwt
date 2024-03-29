use actix_web::{
    cookie::{time::Duration as ActixWebDuration, Cookie},
    post, web, HttpResponse, Responder,
};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json;

use crate::models::users::{LoginUserSchema, TokenClaims, User};
use crate::AppState;

#[post("/auth/login")]
pub async fn user_login_handler(
    body: web::Json<LoginUserSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let query_result = sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", body.email)
        .fetch_optional(&data.db)
        .await
        .unwrap();

    let is_valid = query_result.to_owned().map_or(false, |user| {
        let password_hashed = PasswordHash::new(&user.password).unwrap();
        return Argon2::default()
            .verify_password(body.password.as_bytes(), &password_hashed)
            .map_or(false, |_| true);
    });

    if !is_valid {
        return HttpResponse::BadRequest()
            .json(serde_json::json!({"status": "fail", "message": "Invalid email or password"}));
    }

    let user = query_result.unwrap();

    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::minutes(60)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: user.id.to_string(),
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(data.env.jwt_secret.as_ref()),
    )
    .unwrap();

    let cookie = Cookie::build("token", token.to_owned())
        .path("/")
        .max_age(ActixWebDuration::new(60 * 60, 0))
        .http_only(true)
        .finish();

    return HttpResponse::Ok()
        .cookie(cookie)
        .json(serde_json::json!({ "status": "success", "token": token }));
}
