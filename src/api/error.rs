use actix_web::{http::StatusCode, error::ResponseError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("the requested resource could not be found")]
    NotFound,

    #[error("internal server error")]
    Opaque(#[from] anyhow::Error),
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<butane::Error> for ApiError {
    fn from(error: butane::Error) -> Self {
        match error {
            butane::Error::NoSuchObject => Self::NotFound,
            _ => Self::Opaque(error.into()),
        }
    }
}
