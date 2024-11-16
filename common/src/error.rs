use crate::error::BaseError::BusinessError;
use actix_web::body::BoxBody;
use actix_web::{error, HttpResponse};
use derive_more::Display;
use serde::Serialize;
use std::env::VarError;
use thiserror::Error;

pub type AppResult<T> = Result<T, BaseError>;

#[derive(Debug, Error)]
pub enum BaseError {
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("var error : {0}")]
    VarError(#[from] VarError),

    #[error("serde json error : {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("configure error : {0}")]
    ConfigError(#[from] config::ConfigError),

    #[error("business error")]
    BusinessError(#[from] AppBusinessError),
}

#[derive(Error, Debug, Display, Serialize)]
#[display("{error_msg:?}, code: {error_code:?}")]
pub struct AppBusinessError {
    error_msg: &'static str,
    error_code: u32,
}

impl error::ResponseError for BaseError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        match self {
            BusinessError(e) => {
                HttpResponse::InternalServerError().json(e)
            }
            _ => {
                HttpResponse::InternalServerError().json(self.to_string())
            }
        }
    }
}


pub const DATA_NOT_FOUND: BaseError = BusinessError(AppBusinessError { error_msg: "data not found", error_code: 10001 });
