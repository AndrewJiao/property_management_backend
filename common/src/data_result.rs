use crate::error::{BaseError, PARAM_NOT_SUPPORT};
use chrono::Utc;
use derive_more::Display;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use validator::Validate;

//region 分页
#[derive(Deserialize, Validate)]
pub struct PaginateSearch<T = ()> {
    #[validate(range(min = 1))]
    current_page: i64,
    #[validate(range(min = 1, max = 10))]
    page_size: i64,
    #[allow(dead_code)]
    order_type: Option<OrderType>,
    param: Option<T>,
}
impl<T> PaginateSearch<T> {
    pub fn off_set(&self) -> i64 {
        (self.current_page - 1) * self.page_size
    }
    pub fn limit(&self) -> i64 {
        self.page_size
    }
    pub fn current_page(&self) -> i64 {
        self.current_page
    }

    pub fn value(&self) -> AppResult<Option<&T>> {
        match self.param {
            None => { Err(PARAM_NOT_SUPPORT) }
            Some(ref e) => { Ok(Some(e)) }
        }
    }

    pub fn produce_page_result(&self, total: i32) -> Option<PaginateResult> {
        Some(PaginateResult {
            page_size: self.page_size as i32,
            total_size: total,
        })
    }
}


#[derive(Deserialize, Serialize)]
pub enum OrderType { DESC, ASC }

#[derive(Serialize)]
pub struct PaginateResult {
    pub page_size: i32,
    pub total_size: i32,
}
//endregion

//region 标准数据返回
#[derive(Serialize)]
pub struct AppDataResult<T = ()> {
    pub code: u32,
    pub message: String,
    pub data: T,
    pub paginate_result: Option<PaginateResult>,
    pub time_stamp: chrono::DateTime<Utc>,
}

//endregion


//region 错误相关
#[derive(Error, Debug, Display, Serialize)]
#[display("{error_msg:?}, code: {error_code:?}")]
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
