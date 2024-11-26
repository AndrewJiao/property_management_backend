use crate::error::{BaseError, PARAM_NOT_SUPPORT};
use base64::engine::general_purpose;
use base64::Engine;
use chrono::NaiveDateTime;
use derive_more::Display;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use validator::Validate;

//region 分页
#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PaginateSearch {
    #[validate(range(min = 1))]
    current_page: i64,
    #[validate(range(min = 1, max = 100))]
    page_size: i64,
    #[allow(dead_code)]
    order_type: Option<OrderType>,
    pub owner_name: Option<String>,
    pub param: Option<String>,
}


impl PaginateSearch {
    pub fn off_set(&self) -> i64 {
        (self.current_page - 1) * self.page_size
    }
    pub fn limit(&self) -> i64 {
        self.page_size
    }
    pub fn current_page(&self) -> i64 {
        self.current_page
    }

    ///
    /// 解析base64然后转为对象
    ///
    pub fn convert_param<T>(&self) -> AppResult<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        if let Some(ref e) = self.param {
            let decode = general_purpose::STANDARD.decode(e).map_err(BaseError::Base64Error)?;
            let decode_str = String::from_utf8(decode).map_err(BaseError::FromUtf8Error)?;
            serde_json::from_str::<T>(&decode_str).map_err(BaseError::JsonError)
        } else {
            Err(PARAM_NOT_SUPPORT)
        }
    }


    pub fn produce_page_result(&self, total: i64) -> Option<PaginateResult> {
        Some(PaginateResult {
            page_size: self.page_size,
            total_size: total,
            current_page: self.current_page,
        })
    }
}


#[derive(Deserialize, Serialize)]
pub enum OrderType { DESC, ASC }

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginateResult {
    pub page_size: i64,
    pub total_size: i64,
    pub current_page: i64,
}
//endregion

//region 标准数据返回
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppDataResult<T = ()> {
    pub code: u32,
    pub message: String,
    pub data: T,
    pub paginate_result: Option<PaginateResult>,
    pub time_stamp: NaiveDateTime,
}

//endregion


//region 错误相关
#[derive(Error, Debug, Display, Serialize)]
#[display("{error_msg:?}, code: {error_code:?}")]
#[serde(rename_all = "camelCase")]
pub struct AppBusinessError {
    pub error_msg: &'static str,
    pub error_code: u32,
}


impl AppBusinessError {
    pub fn new(error_msg: &'static str, error_code: u32) -> Self {
        Self { error_msg, error_code }
    }
}
//endregion

pub type AppResult<T> = Result<T, BaseError>;
