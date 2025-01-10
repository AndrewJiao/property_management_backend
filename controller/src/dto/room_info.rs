use crate::dto::{ToInsertPO, ToUpdatePO};
use chrono::NaiveDateTime;
use repository::room_info::{RoomInfoDetailInsertPo, RoomInfoDetailUpdatePo};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Validate, Default,Debug)]
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

#[derive(Deserialize, Validate, Default,Debug)]
#[serde(rename_all = "camelCase")]
pub struct RoomInfoDetailOffsetSearchDto {
    #[serde(default ,deserialize_with = "common::tools::serde::empty_vec_or_null_as_none")]
    pub room_number: Option<Vec<String>>,
    pub create_time_star: Option<NaiveDateTime>,
    pub create_time_end: Option<NaiveDateTime>,

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
    // #[validate(range(min = 0))]
    // pub water_meter_sub: Option<i64>,
    #[validate(range(min = 0))]
    pub electricity_meter_num_before: Option<i64>,
    #[validate(range(min = 0))]
    pub electricity_meter_num: Option<i64>,
    // #[validate(range(min = 0))]
    // pub electricity_meter_sub: Option<i64>,
    #[validate(length(min = 0, max = 100))]
    pub comment: Option<String>,
}
impl ToUpdatePO for RoomInfoDetailUpdateDto {
    type PO<'a> = RoomInfoDetailUpdatePo<'a>;

    fn to_update_po(&self, id: i64) -> Self::PO<'_> {
        RoomInfoDetailUpdatePo {
            id,
            water_meter_num_before: self.water_meter_num_before.as_ref(),
            water_meter_num: self.water_meter_num.as_ref(),
            water_meter_sub: None,
            electricity_meter_num_before: self.electricity_meter_num_before.as_ref(),
            electricity_meter_num: self.electricity_meter_num.as_ref(),
            electricity_meter_sub: None,
            month_version: None,
            comment: self.comment.as_deref(),
            create_by: None,
            update_by: None,
            create_time: None,
            update_time: None,
        }
    }
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", tag = "searchType", content = "searchValue")]
pub enum RoomInfoSearchType {
    //查询月份版本
    MonthVersion(String),
    //预查上月水电
    PreSearchBefore(String)
}

#[derive(Serialize,Deserialize,Validate)]
#[serde(rename_all = "camelCase")]
pub struct RoomInfoManuallyInsertDto {
    #[validate(length(min = 0, max = 100))]
    pub room_number: String,
    #[validate(range(min = 0))]
    pub water_meter_num_before: Option<i64>,
    #[validate(range(min = 0))]
    pub water_meter_num: Option<i64>,
    #[validate(range(min = 0))]
    pub electricity_meter_num_before: Option<i64>,
    #[validate(range(min = 0))]
    pub electricity_meter_num: Option<i64>,
    #[validate(length(min = 0, max = 100))]
    pub month_version: String,
    #[validate(length(min = 0, max = 5000))]
    pub comment: Option<String>,
}
impl ToInsertPO for RoomInfoManuallyInsertDto {
    type PO<'a> = RoomInfoDetailInsertPo<'a>;

    fn to_insert_po(&self) -> Self::PO<'_> {
        RoomInfoDetailInsertPo {
            room_number: Some(self.room_number.as_str()),
            water_meter_num_before: self.water_meter_num_before,
            water_meter_num: self.water_meter_num,
            water_meter_sub: None,
            electricity_meter_num_before: self.electricity_meter_num_before,
            electricity_meter_num: self.electricity_meter_num,
            electricity_meter_sub: None,
            month_version: Some(&self.month_version),
            comment: self.comment.as_deref(),
            create_by: None,
            update_by: None,
            create_time: None,
            update_time: None,
            room_owner_name: None,
            delete_at: None,
        }
    }
}
