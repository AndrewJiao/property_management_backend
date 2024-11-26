use chrono::NaiveDateTime;

pub fn now_local_date_time_naive() -> NaiveDateTime {
    chrono::Local::now().naive_local()
}


