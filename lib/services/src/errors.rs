use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use anyhow::Error as AnyhowError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MyError {
    #[error("Internal Server Error: {0}")]
    InternalError(#[source] AnyhowError),

    #[error("Unauthorized Error: {0}")]
    Unauthorized(#[source] AnyhowError),

    #[error("Resources not found!")]
    NotFound,
}

impl ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        #[derive(Debug, Deserialize, Serialize)]
        struct ErrorResponse<U> {
            pub msg: String,
            pub data: Option<U>,
        }

        match self {
            Self::InternalError(e) => {
                HttpResponse::InternalServerError().json(ErrorResponse::<String> {
                    msg: e.to_string(),
                    data: None,
                })
            }
            Self::Unauthorized(e) => HttpResponse::Unauthorized().json(ErrorResponse::<String> {
                msg: e.to_string(),
                data: None,
            }),
            Self::NotFound => HttpResponse::NotFound().json(ErrorResponse::<String> {
                msg: "not found".into(),
                data: None,
            }),
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Self::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Self::NotFound => StatusCode::NOT_FOUND,
        }
    }
}

pub type Result<T> = std::result::Result<T, MyError>;
