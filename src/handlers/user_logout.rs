use actix_web::{
    cookie::{time::Duration as ActixWebDuration, Cookie},
    get, HttpResponse, Responder,
};
use serde_json;

use crate::jwt_auth;

#[get("/auth/logout")]
pub async fn logout_handler(_: jwt_auth::JwtMiddleware) -> impl Responder {
    let cookie = Cookie::build("token", "")
        .path("/")
        .max_age(ActixWebDuration::new(-1, 0))
        .http_only(true)
        .finish();
    
    return HttpResponse::Ok()
        .cookie(cookie)
        .json(serde_json::json!({ "status": "success" }));
}
