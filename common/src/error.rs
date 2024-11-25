use crate::data_result::AppBusinessError;
use crate::error::BaseError::BusinessError;
use actix_web::body::BoxBody;
use actix_web::{error, HttpResponse};
use diesel::r2d2::Error as R2d2Error;
use log::error;
use std::env::VarError;
use std::string::FromUtf8Error;
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

    #[error("serde json error : {0}")]
    FromUtf8Error(#[from] FromUtf8Error),

    #[error("configure error : {0}")]
    ConfigError(#[from] config::ConfigError),

    #[error("database r2d2 error")]
    DatabaseR2D2Error(#[from] R2d2Error),

    #[error("database result error : {0}")]
    DatabaseError(#[from] diesel::result::Error),
    #[error("base64 error : {0}")]
    Base64Error(#[from] base64::DecodeError),


    #[error("business error : {0}")]
    BusinessError(#[from] AppBusinessError),

}


impl error::ResponseError for BaseError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        //打印堆栈
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
pub const PARAM_NOT_SUPPORT: BaseError = BusinessError(AppBusinessError { error_msg: "param is not support", error_code: 10002 });

