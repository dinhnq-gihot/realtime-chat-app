use actix_service::ServiceFactory;
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, HttpResponse,
};
use futures_util::future::{ok, LocalBoxFuture, Ready};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

// JWT Claims
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

// Middleware struct
struct AuthMiddleware {
    secret: Rc<String>,
}

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
        ok(AuthMiddlewareMiddleware {
            service,
            secret: Rc::clone(&self.secret),
        })
    }
}

struct AuthMiddlewareMiddleware<S> {
    service: S,
    secret: Rc<String>,
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

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let secret = Rc::clone(&self.secret);
        let fut = self.service.call(req);

        Box::pin(async move {
            let req = fut.await?;

            // Extract the token from the Authorization header
            if let Some(auth_header) = req.headers().get("Authorization") {
                if let Ok(auth_str) = auth_header.to_str() {
                    if auth_str.starts_with("Bearer ") {
                        let token = &auth_str[7..];
                        let validation = Validation::new(Algorithm::HS256);

                        match decode::<Claims>(
                            token,
                            &DecodingKey::from_secret(secret.as_ref().as_bytes()),
                            &validation,
                        ) {
                            Ok(token_data) => {
                                req.extensions_mut().insert(token_data.claims);
                                return Ok(req);
                            }
                            Err(_) => {
                                return Ok(req.error_response(HttpResponse::Unauthorized().finish()))
                            }
                        }
                    }
                }
            }

            Ok(req.error_response(HttpResponse::Unauthorized().finish()))
        })
    }
}
