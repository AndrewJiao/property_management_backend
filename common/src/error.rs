use crate::data_result::AppBusinessError;
use crate::error::BaseError::BusinessError;
use actix_web::body::BoxBody;
use actix_web::{error, HttpResponse};
use anyhow::{anyhow, Error};
use diesel::r2d2::Error as R2d2Error;
use lazy_static::lazy_static;
use log::error;
use regex::Regex;
use serde::Serialize;
use std::env::VarError;
use std::string::FromUtf8Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BaseError {
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("actix error : {0}")]
    ActorError(#[from] actix::MailboxError),

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

    #[error("error :{0}")]
    AnyhowError(#[from] anyhow::Error),
}

// impl From<anyhow::Error> for BaseError {
//     fn from(value: Error) -> Self {
//         BaseError::AnyhowError(value)

//     }
// }
///
/// eg:errorMsg = 无权限 code = 10005
///
#[derive(Serialize)]
struct ErrorResponse{
    pub message: String,
    pub source: String,
    pub code: i32
}


lazy_static!(
static ref  ERROR_PATTERN: Regex = regex::Regex::new(r"errorMsg = (?<errorMsg>\S+)\s+code = (?<code>\d+)").expect("regex error");
);

impl From<&anyhow::Error> for ErrorResponse{
    fn from(value: &Error) -> Self {
        let ori_error_msg = value.to_string();
        let error_info = ERROR_PATTERN.captures(&ori_error_msg).map(|cap|{
            let msg = cap.name("errorMsg").map(|e|e.as_str());
            let code = cap.name("code").map(|e|e.as_str());
            (msg,code)
        }).unwrap_or_default();

        if let (Some(msg),Some(code)) = error_info{
            ErrorResponse{
                message: msg.to_string(),
                source: value.source().map(|e|e.to_string()).unwrap_or_default(),
                code: code.parse::<i32>().unwrap_or_default(),
            }
        } else {
            ErrorResponse {
                message: ori_error_msg,
                source: value.source().map(|e|e.to_string()).unwrap_or_default(),
                code: 110,
            }
        }


    }
}



impl error::ResponseError for BaseError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        //打印堆栈
        match self {
            BusinessError(e) => {
                HttpResponse::InternalServerError().json(e)
            }
            BaseError::AnyhowError(e)=>{
                let response:ErrorResponse = e.into();
                HttpResponse::InternalServerError().json(response)
            }
            _ => {
                HttpResponse::InternalServerError().body(self.to_string())
            }
        }
    }
}

pub const DATA_NOT_FOUND: fn() -> anyhow::Error = || anyhow!("data not found");
pub const PARAM_NOT_SUPPORT: fn() -> anyhow::Error = || anyhow!( "param not support");
pub const DB_UPDATE_ERROR: fn() -> anyhow::Error = || anyhow!("db update error");
pub const DB_INSERT_ERROR: fn() -> anyhow::Error = || anyhow!("db insert error");
pub const DB_DELETE_ERROR: fn() -> anyhow::Error = || anyhow!("db delete error");
pub const BUSINESS_ERROR: fn(&str, u32) -> anyhow::Error = |msg, code| anyhow!("errorMsg = {} code = {}",msg,code);
pub const APP_ERROR: fn(&str) -> anyhow::Error = |msg| anyhow!("system_error = {}",msg);
//费用明细
pub const BUSINESS_ERROR_NO_PROPERTY_FEE_INFO: fn() -> anyhow::Error = || anyhow!("errorMsg = 当前单据没有费用明细 code = 10001");
pub const BUSINESS_ERROR_OWNER_FEE_DETAIL_EXIST: fn() -> anyhow::Error = || anyhow!("errorMsg = 费用已生成 code = 10002");
//common
pub const DATA_NOT_EXIST: fn() -> anyhow::Error = || anyhow!("errorMsg = 数据不存在 code = 00001");
pub const DATA_HAS_EXIST: fn() -> anyhow::Error = || anyhow!("errorMsg = 数据已存在 code = 00002");
//user
pub const USER_NOT_EXIST: fn() -> anyhow::Error = || anyhow!("errorMsg = 用户不存在 code = 10001");
pub const USER_PASSWORD_ERROR: fn() -> anyhow::Error = || anyhow!("errorMsg = 账户或密码错误 code = 10002");
pub const USER_ACCOUNT_EXIST: fn() -> anyhow::Error = || anyhow!("errorMsg = 用户账号已存在 code = 10003");
pub const NO_AUTH: fn() -> anyhow::Error = || anyhow!("errorMsg = 无权限 code = 10005");
//approve
pub const APPROVE_NOT_EXIST: fn() -> anyhow::Error = || anyhow!("errorMsg = 审批不存在 code = 20001");
pub const APPROVE_STATE_ERROR: fn() -> anyhow::Error = || anyhow!("errorMsg = 审批状态错误 code = 20002");
pub const ROOM_HAS_BEEN_BIND: fn() -> anyhow::Error = || anyhow!("errorMsg = 当前房间号已被绑定 code = 20003");
//账户信息不存
pub const ACCOUNT_NOT_EXIST: fn() -> anyhow::Error = || anyhow!("errorMsg = 账户信息不存在 code = 30001");
pub const ACCOUNT_EXIST: fn() -> anyhow::Error = || anyhow!("errorMsg = 账户已创建 code = 30002");
pub const ACCOUNT_NOT_SUPPORT_FAST_LOGIN: fn() -> anyhow::Error = || anyhow!("errorMsg = 当前账号不允许快速登录 code = 30003");
//外部接口error
pub const WE_CHART_SNS_ERROR: fn() -> anyhow::Error = || anyhow!("errorMsg = 微信认证错误 code = 40001");
//房间
pub const ROOM_IS_NOT_EXIST: fn() -> anyhow::Error = || anyhow!("errorMsg = 指定的房号不存在 code = 50001");


