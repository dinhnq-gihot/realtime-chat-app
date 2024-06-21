use std::sync::Arc;

use crate::{
    errors::{MyError, Result},
    features::users::types::GenericResponse,
    utils::{general_response::LoginResponse, jwt},
};

use super::{extractor::AuthenticationToken, types::*};
use actix_web::{
    http::StatusCode,
    web::{Data, Json},
    HttpRequest, HttpResponse,
};
use chatapp_db::{database::Database, repositories::user::Users as UsersRepository};
use tracing::{instrument, Level};

#[instrument(skip_all, level = Level::DEBUG)]
#[actix_web::post("/login")]
pub async fn login(
    req: HttpRequest,
    payload: Json<LoginRequest>,
    secret: Data<String>,
    db: Data<Database>,
) -> Result<HttpResponse> {
    tracing::debug!("REQUEST: {req:?}");

    let LoginRequest { email, password } = payload.0;

    let user_repo = UsersRepository::new(Arc::clone(&db.into_inner()));
    let user = user_repo
        .login(email, password)
        .await
        .map_err(|e| MyError::InternalError(e.into()))?;

    let token = jwt::encode_jwt(user.id, user.email, secret.into_inner().to_string())
        .map_err(|e| MyError::InternalError(e.into()))?;

    Ok(HttpResponse::Ok()
        .status(StatusCode::CREATED)
        .json(LoginResponse {
            msg: "Login successfully".into(),
            data: Some(token),
        }))
}

#[actix_web::get("/protected")]
async fn protected(auth_token: AuthenticationToken) -> HttpResponse {
    chatapp_logger::debug!("{:#?}", auth_token);

    HttpResponse::Ok()
        .status(StatusCode::OK)
        .json(GenericResponse::<String> {
            msg: "Authorized".to_owned(),
            data: None,
        })
}
