use chrono::{Months, NaiveDateTime};
use lazy_static::lazy_static;
use std::ops::Add;

pub fn now_local_date_time_naive() -> NaiveDateTime {
    chrono::Local::now().naive_local()
}

///
/// 获取当前年月日str
///
pub fn now_local_date(fmt: &str) -> String {
    let now = now_local_date_time_naive();
    now.format(fmt).to_string()
}

///
/// 获取指定数量月份前的string
///
pub fn before_month_local_date(fmt: &str, month: u32) -> String {
    let now = now_local_date_time_naive();
    let before_month = now.checked_sub_months(Months::new(month)).expect("before_month_get_error");
    before_month.format(fmt).to_string()
}

pub fn now_utc_date_time_naive() -> NaiveDateTime {
    chrono::Utc::now().naive_utc()
}

pub fn now_utc_date_str(fmt: &str) -> String {
    let now = now_utc_date_time_naive();
    now.format(fmt).to_string()
}

//获取最早的时间 1970-01-01 00:00:00
lazy_static!(
    pub static ref DEFAULT_TIME:NaiveDateTime = NaiveDateTime::default();
);


pub fn nexted_time_stamp(stamp: i64) -> i64 {
    chrono::Utc::now().timestamp().add(stamp)
}

pub fn current_time_stamp() -> i64 {
    chrono::Utc::now().timestamp()
}

