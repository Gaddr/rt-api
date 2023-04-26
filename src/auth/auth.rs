use core::fmt;
use std::future::{ready, Ready};

use actix_web::error::ErrorUnauthorized;
use actix_web::http::header::HeaderMap;
use actix_web::web::Data;
use actix_web::{dev::Payload, Error as ActixWebError};
use actix_web::{web, FromRequest, HttpMessage, HttpRequest};
use jsonwebtoken::{decode, Algorithm, DecodingKey, TokenData, Validation};
use serde::Serialize;

use crate::auth::models::GenericClaims;
use crate::AppState;

use super::errors::AuthErrors;

#[derive(Debug, Serialize)]
struct ErrorResponse {
    status: String,
    message: String,
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

pub struct JwtMiddleware {
    pub user: GenericClaims,
}

impl FromRequest for JwtMiddleware {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let data = req.app_data::<web::Data<AppState>>().unwrap();

        let jwt = match jwt_from_header(&req.headers()) {
            Ok(jwt) => jwt,
            Err(err) => return ready(Err(ErrorUnauthorized(err))),
        };

        let user = match verify_token(&data, &jwt) {
            Ok(decoded_token) => decoded_token.claims,
            Err(err) => return ready(Err(ErrorUnauthorized(err))),
        };

        req.extensions_mut().insert::<GenericClaims>(user.clone());

        ready(Ok(JwtMiddleware { user }))
    }
}

pub fn verify_token(
    state: &Data<AppState>,
    token: &str,
) -> Result<TokenData<GenericClaims>, AuthErrors> {
    let mut validation = Validation::default();
    validation.leeway = 60;
    validation.validate_exp = true;

    let decoded_token = match decode::<GenericClaims>(
        &token,
        &DecodingKey::from_secret(&state.environment.secret_key.as_ref()),
        &Validation::new(Algorithm::HS256),
    ) {
        Ok(token) => token,
        Err(err) => {
            println!("{}", err.to_string());
            return Err(AuthErrors::DecodeTokenError);
        }
    };

    return Ok(decoded_token);
}

pub fn jwt_from_header(headers: &HeaderMap) -> Result<String, AuthErrors> {
    let bearer = "Bearer ";

    let header = match headers.get("Authorization") {
        Some(v) => v,
        None => return Err(AuthErrors::AuthMissing),
    };

    let auth_header = match std::str::from_utf8(header.as_bytes()) {
        Ok(v) => v,
        Err(_) => return Err(AuthErrors::AuthMissing),
    };

    if !auth_header.starts_with(bearer) {
        return Err(AuthErrors::InvalidAuthHeaderError);
    }

    Ok(auth_header.trim_start_matches(bearer).to_owned())
}
