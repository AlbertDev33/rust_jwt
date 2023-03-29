use actix_web::{get, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use serde_json;
use uuid::Uuid;

use super::filter_user::filter_user_record;
use crate::jwt_auth;
use crate::models::users::User;
use crate::AppState;

#[get("users/me")]
pub async fn get_me_handler(
    req: HttpRequest,
    data: web::Data<AppState>,
    _: jwt_auth::JwtMiddleware,
) -> impl Responder {
    let ext = req.extensions();
    let user_id = ext.get::<Uuid>().unwrap();

    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", user_id)
        .fetch_one(&data.db)
        .await
        .unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "data": serde_json::json!({
            "user": filter_user_record(&user)
        })
    });

    return HttpResponse::Ok().json(json_response);
}
