use crate::schema::basic::t_room_info_detail;
use chrono::NaiveDateTime;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use management_macro::AutoOperation;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = t_room_info_detail)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(rename_all = "camelCase")]
pub struct RoomInfoDetailPo {
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
    pub is_delete: bool,
}

#[derive(Insertable, Serialize, AutoOperation)]
#[diesel(table_name = t_room_info_detail)]
pub struct RoomInfoDetailInsertPo<'a> {
    pub room_number: Option<&'a str>,
    pub water_meter_num_before: Option<&'a i64>,
    pub water_meter_num: Option<&'a i64>,
    pub water_meter_sub: Option<&'a i64>,
    pub electricity_meter_num_before: Option<&'a i64>,
    pub electricity_meter_num: Option<&'a i64>,
    pub electricity_meter_sub: Option<&'a i64>,
    pub month_version: Option<&'a str>,
    pub comment: Option<&'a str>,
    pub create_by: Option<&'a str>,
    pub update_by: Option<&'a str>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
}

#[derive(Identifiable, AsChangeset, Serialize, AutoOperation)]
#[diesel(table_name = t_room_info_detail)]
pub struct RoomInfoDetailUpdatePo<'a> {
    pub id: i64,
    pub water_meter_num_before: Option<&'a i64>,
    pub water_meter_num: Option<&'a i64>,
    pub water_meter_sub: Option<i64>,
    pub electricity_meter_num_before: Option<&'a i64>,
    pub electricity_meter_num: Option<&'a i64>,
    pub electricity_meter_sub: Option<i64>,
    pub month_version: Option<&'a str>,
    pub comment: Option<&'a str>,
    pub create_by: Option<&'a str>,
    pub update_by: Option<&'a str>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
}

impl RoomInfoDetailUpdatePo<'_> {
    pub fn re_calculate(mut self) -> Self {
        if let (Some(before), Some(now)) = (self.water_meter_num_before, self.water_meter_num) {
            self.water_meter_sub = Some(now - before);
        } else {
            self.water_meter_sub = None;
        }

        self.electricity_meter_sub = match
        (
            self.electricity_meter_num_before,
            self.electricity_meter_num
        ) {
            (Some(before), Some(now)) => Some(now - before),
            _ => None,
        };
        self
    }
}
