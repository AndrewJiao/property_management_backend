use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;
use repository::room_info::RoomInfoDetailUpdatePo;
use crate::dto::ToUpdatePO;

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RoomInfoDetailSearchDto {
    #[validate(length(min = 0, max = 100))]
    pub room_number: Option<String>,
    #[validate(length(min = 0, max = 100))]
    pub month_version: Option<String>,
    pub create_time_star: Option<NaiveDateTime>,
    pub create_time_end: Option<NaiveDateTime>,
    pub update_time_end: Option<NaiveDateTime>,
    pub update_time_star: Option<NaiveDateTime>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomInfoDetailResultDto {
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
}

//理论上由系统新增
// #[derive(Deserialize, Validate)]
// pub struct RoomInfoDetailInsertPo<'a> {
//     pub room_number: Option<&'a str>,
//     pub water_meter_num_before: Option<&'a i64>,
//     pub water_meter_num: Option<&'a i64>,
//     pub water_meter_sub: Option<&'a i64>,
//     pub electricity_meter_num_before: Option<&'a i64>,
//     pub electricity_meter_num: Option<&'a i64>,
//     pub electricity_meter_sub: Option<&'a i64>,
//     pub month_version: Option<&'a str>,
//     pub comment: Option<&'a str>,
//     pub create_by: Option<&'a str>,
//     pub update_by: Option<&'a str>,
//     pub create_time: Option<NaiveDateTime>,
//     pub update_time: Option<NaiveDateTime>,
// }

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RoomInfoDetailUpdateDto {
    #[validate(range(min = 0))]
    pub water_meter_num_before: Option<i64>,
    #[validate(range(min = 0))]
    pub water_meter_num: Option<i64>,
    #[validate(range(min = 0))]
    pub water_meter_sub: Option<i64>,
    #[validate(range(min = 0))]
    pub electricity_meter_num_before: Option<i64>,
    #[validate(range(min = 0))]
    pub electricity_meter_num: Option<i64>,
    #[validate(range(min = 0))]
    pub electricity_meter_sub: Option<i64>,
}
impl ToUpdatePO for RoomInfoDetailUpdateDto {
    type PO<'a> = RoomInfoDetailUpdatePo<'a>;

    fn to_update_po(&self, id: i32) -> Self::PO<'_> {
        RoomInfoDetailUpdatePo {
            id: id as i64,
            water_meter_num_before: self.water_meter_num_before.as_ref(),
            water_meter_num: self.water_meter_num.as_ref(),
            water_meter_sub: self.water_meter_sub,
            electricity_meter_num_before: self.electricity_meter_num_before.as_ref(),
            electricity_meter_num: self.electricity_meter_num.as_ref(),
            electricity_meter_sub: self.electricity_meter_sub,
            month_version: None,
            comment: None,
            create_by: None,
            update_by: None,
            create_time: None,
            update_time: None,
        }
    }
}
