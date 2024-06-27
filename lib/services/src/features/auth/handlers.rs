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
use chatapp_logger::debug;

#[actix_web::post("/login")]
pub async fn login(
    req: HttpRequest,
    payload: Json<LoginRequest>,
    db: Data<Database>,
) -> Result<HttpResponse> {
    debug!("REQUEST: {req:?}");

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
    debug!("{claims:#?}");
    println!("{claims:#?}");

    HttpResponse::Ok()
        .status(StatusCode::OK)
        .json(GenericResponse::<Claims> {
            msg: "Authorized".to_owned(),
            data: Some(claims),
        })
}
