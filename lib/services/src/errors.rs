use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use anyhow::Error as AnyhowError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MyError {
    #[error("Internal Server Error: {0}")]
    InternalError(#[from] AnyhowError),
}

impl ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        #[derive(Debug, Deserialize, Serialize)]
        struct ErrorResponse<U> {
            pub msg: String,
            pub data: Option<U>,
        }

        HttpResponse::InternalServerError().json(ErrorResponse::<String> {
            msg: self.to_string(),
            data: None,
        })
    }

    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

pub type Result<T> = std::result::Result<T, MyError>;
