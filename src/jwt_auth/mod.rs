use actix_web::HttpMessage;
use actix_web::{
    dev::Payload, error::ErrorUnauthorized, http, web, Error as ActixWebError, FromRequest,
    HttpRequest,
};
use core::fmt::Display;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Serialize;
use serde_json;
use std::future::{ready, Ready};
use uuid::Uuid;

use crate::models::users::TokenClaims;
use crate::AppState;

#[derive(Debug, Serialize)]
struct ErrorResponse {
    status: String,
    message: String,
}

impl Display for ErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", serde_json::to_string(&self).unwrap());
    }
}

pub struct JwtMiddleware {
    pub user_id: Uuid,
}

impl FromRequest for JwtMiddleware {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let data = req.app_data::<web::Data<AppState>>().unwrap();

        let token = req
            .headers()
            .get(http::header::AUTHORIZATION)
            .map(|header| header.to_str().unwrap().split_at(7).1.to_string());

        if token.is_none() {
            let json_error = ErrorResponse {
                status: "fail".to_string(),
                message: "You are not logged in, please provide token".to_string(),
            };
            return ready(Err(ErrorUnauthorized(json_error)));
        }

        let claims = match decode::<TokenClaims>(
            &token.unwrap(),
            &DecodingKey::from_secret(data.env.jwt_secret.as_ref()),
            &Validation::default(),
        ) {
            Ok(c) => c.claims,
            Err(_) => {
                let json_error = ErrorResponse {
                    status: "fail".to_string(),
                    message: "Invalid token".to_string(),
                };
                return ready(Err(ErrorUnauthorized(json_error)));
            }
        };

        let user_id = Uuid::parse_str(claims.sub.as_str()).unwrap();
        req.extensions_mut().insert::<Uuid>(user_id.to_owned());
        return ready(Ok(JwtMiddleware { user_id }));
    }

    fn extract(req: &actix_web::HttpRequest) -> Self::Future {
        Self::from_request(req, &mut actix_web::dev::Payload::None)
    }
}
