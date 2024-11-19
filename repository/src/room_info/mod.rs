use crate::schema::basic::t_room_info_detail;
use chrono::NaiveDateTime;
use diesel::{Queryable, Selectable};
use serde::Deserialize;

#[derive(Queryable, Selectable, Deserialize)]
#[diesel(table_name = t_room_info_detail)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RoomInfoDetail {
    pub id: i64,
    pub room_number: Option<String>,
    pub water_meter_num_before: Option<i64>,
    pub water_meter_num: Option<i64>,
    pub water_meter_sub: Option<i64>,
    pub electricity_meter_num_before: Option<i64>,
    pub electricity_meter_num: Option<i64>,
    pub electricity_meter_sub: Option<i64>,
    pub month_version: Option<String>,
    pub comment: Option<String>,
    pub create_by: Option<String>,
    pub update_by: Option<String>,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
    pub is_delete: Option<bool>,
}