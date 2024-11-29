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

