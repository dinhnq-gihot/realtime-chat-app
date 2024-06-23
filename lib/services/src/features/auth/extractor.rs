use actix_web::{http::header::AUTHORIZATION, FromRequest};
use anyhow::anyhow;
use log::debug;
use std::future::ready;

use crate::{errors::MyError, features::auth::types::Claims, utils::jwt::decode_jwt};

impl FromRequest for Claims {
    type Error = MyError;
    type Future = std::future::Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        debug!("REQUEST: {req:?}");

        if let Some(auth_header) = req.headers().get(AUTHORIZATION) {
            if let Ok(auth_str) = auth_header.to_str() {
                if let Some(token) = auth_str.strip_prefix("Bearer ") {
                    match decode_jwt(token.to_owned()) {
                        Ok(claims) => return ready(Ok(claims)),
                        Err(e) => return ready(Err(MyError::Unauthorized(e))),
                    }
                }
            }
        }

        ready(Err(MyError::Unauthorized(anyhow!(
            "Authorization header missing or malformed"
        ))))
    }
}
