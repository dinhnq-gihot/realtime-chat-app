use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    http::header::AUTHORIZATION,
    Error, HttpMessage,
};
use futures_util::future::{ok, LocalBoxFuture, Ready};
use std::task::{Context, Poll};

use crate::utils::jwt::decode_jwt;

// Middleware struct
pub struct AuthMiddleware;

// Middleware factory
impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddlewareMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddlewareMiddleware { service })
    }
}

pub struct AuthMiddlewareMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        
        if let Some(auth_header) = req.request().headers().get(AUTHORIZATION) {
            if let Ok(auth_str) = auth_header.to_str() {
                if let Some(token) = auth_str.strip_prefix("Bearer ") {
                    match decode_jwt(token.to_owned()) {
                        Ok(token_data) => {
                            req.request().extensions_mut().insert(token_data);
                            return Box::pin(async move {
                                let fut = self.service.call(req);
                                let service_resp = fut.await?;
                                Ok(service_resp)
                            });
                        }
                        Err(e) => {
                            return Box::pin(async move {
                                Err(actix_web::error::ErrorUnauthorized(e.to_string()))
                            })
                        }
                    }
                }
            }
        }

        Box::pin(async move {
            Err(actix_web::error::ErrorUnauthorized(
                "Authorization header missing or malformed",
            ))
        })
    }
}
