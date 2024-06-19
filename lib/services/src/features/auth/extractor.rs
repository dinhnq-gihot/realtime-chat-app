use std::future::ready;

use actix_web::{
    error::ErrorUnauthorized, http::header::AUTHORIZATION, web, Error as ActixWebError, FromRequest,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use log::debug;

use crate::features::auth::models::Claims;

#[derive(Debug)]
pub struct AuthenticationToken {
    pub sub: String,
}

impl FromRequest for AuthenticationToken {
    type Error = ActixWebError;
    type Future = std::future::Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        debug!("REQUEST: {req:?}");
        let authorization_header = req.headers().get(AUTHORIZATION);
        if authorization_header.is_none() {
            return ready(Err(ErrorUnauthorized("No authentication token sent!")));
        }

        let authentication_token = authorization_header
            .unwrap()
            .to_str()
            .unwrap_or("")
            .replace("Bearer ", "")
            .to_string();
        if authentication_token.is_empty() {
            return ready(Err(ErrorUnauthorized(
                "Authentication token has foreign chars!",
            )));
        }

        let secret = req.app_data::<web::Data<String>>().unwrap();

        let token_result = decode::<Claims>(
            &authentication_token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::new(jsonwebtoken::Algorithm::HS256),
        );

        match token_result {
            Ok(token) => ready(Ok(AuthenticationToken {
                sub: token.claims.sub,
            })),
            Err(e) => ready(Err(ErrorUnauthorized(e.to_string()))),
        }
    }
}
