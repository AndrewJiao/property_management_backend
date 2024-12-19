use chrono::NaiveDateTime;

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


pub fn now_utc_date_time_naive() -> NaiveDateTime {
    chrono::Utc::now().naive_utc()
}

pub fn now_utc_date_str(fmt: &str) -> String {
    let now = now_utc_date_time_naive();
    now.format(fmt).to_string()
}

