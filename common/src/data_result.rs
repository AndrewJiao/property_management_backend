use crate::db_config::db_get_connection;
use crate::error::BaseError;
use chrono::Utc;
use derive_more::Display;
use diesel::backend::Backend;
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
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
    order_type: Option<OrderType>,
    param: Option<T>,
}
impl PaginateSearch {
    pub fn off_set(&self) -> i64 {
        (self.current_page - 1) * self.page_size
    }
    pub fn limit(&self) -> i64 {
        self.page_size
    }


    pub fn execute_search<QDsl, Po, DB, Fun>(&self, queryDsl: QDsl, fun: Fun) -> AppResult<(Vec<Po>, PaginateResult)>
    where
        QDsl: QueryDsl,
        Po: SelectableHelper<DB>,
        DB: Backend,
        Fun: Fn(QDsl) -> QDsl,
    {
        let mut connection = db_get_connection();
        let result: Vec<Po> =
            fun(queryDsl.clone())
                .select(Po::as_select())
                .offset(self.off_set())
                .limit(self.limit())
                .load(&mut connection)?;
        (
            result,
            PaginateResult {
                page_size: 0,
                total_size: 0,
            }
        )
    }
}


#[derive(Deserialize, Serialize)]
pub enum OrderType { DESC, ASC }

#[derive(Serialize)]
pub struct PaginateResult {
    pub page_size: u32,
    pub total_size: u32,
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
