use std::sync::Arc;

use crate::{
    errors::{MyError, Result},
    features::users::types::GenericResponse,
    utils::{general_response::LoginResponse, jwt},
};

use super::types::*;
use actix_web::{
    http::StatusCode,
    web::{Data, Json},
    HttpRequest, HttpResponse,
};
use chatapp_db::{database::Database, repositories::user::Users as UsersRepository};
use tracing::{instrument, Level};

#[actix_web::post("/login")]
#[instrument(skip_all, level = Level::DEBUG)]
pub async fn login(
    req: HttpRequest,
    payload: Json<LoginRequest>,
    db: Data<Database>,
) -> Result<HttpResponse> {
    tracing::debug!("REQUEST: {req:?}");

    let LoginRequest { email, password } = payload.0;

    let user_repo = UsersRepository::new(Arc::clone(&db.into_inner()));
    let user = user_repo
        .login(email, password)
        .await
        .map_err(MyError::InternalError)?;

    let token = jwt::encode_jwt(user.id, user.email).map_err(MyError::InternalError)?;

    Ok(HttpResponse::Ok()
        .status(StatusCode::CREATED)
        .json(LoginResponse {
            msg: "Login successfully".into(),
            data: Some(token),
        }))
}

#[actix_web::get("/protected")]
async fn protected(claims: Claims) -> HttpResponse {
    chatapp_logger::debug!("{:#?}", claims);
    println!("{claims:#?}");

    HttpResponse::Ok()
        .status(StatusCode::OK)
        .json(GenericResponse::<Claims> {
            msg: "Authorized".to_owned(),
            data: Some(claims),
        })
}
