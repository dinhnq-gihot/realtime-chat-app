use super::{extractor::AuthenticationToken, models::*};
use actix_web::{
    http::StatusCode,
    web::{Data, Json},
    HttpRequest, HttpResponse,
};
use jsonwebtoken::{encode, EncodingKey, Header};
use tracing::{instrument, Level};

#[instrument(skip_all, level = Level::DEBUG)]
#[actix_web::post("/login")]
pub async fn login(
    req: HttpRequest,
    payload: Json<LoginRequest>,
    secret: Data<String>,
) -> HttpResponse {
    tracing::debug!("REQUEST: {req:?}");

    let LoginRequest { email, password: _ } = payload.0;

    let claims = Claims {
        sub: email,
        exp: (chrono::Utc::now() + chrono::Duration::days(1)).timestamp() as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap();

    let resp = LoginResponse {
        msg: "success".into(),
        token,
    };

    HttpResponse::Ok().status(StatusCode::CREATED).json(resp)
}

#[actix_web::get("/protected")]
async fn protected(auth_token: AuthenticationToken) -> HttpResponse {
    chatapp_logger::debug!("{:#?}", auth_token);

    HttpResponse::Ok().status(StatusCode::OK).json(Response {
        msg: "Authorized".to_owned(),
    })
}
