use chrono::{Datelike, Local, Months};
use common::data_result::AppResult;
use common::db_config::db_get_connection;
use common::error::BUSINESS_ERROR;
use common::tools::time::now_local_date_time_naive;
use common::CURRENT_USE;
use diesel::{insert_into, ExpressionMethods, QueryDsl, RunQueryDsl};
use regex::Regex;
use repository::owner_info::OwnerBasicInfoPo;
use repository::room_info::{RoomInfoDetailInsertPo, RoomInfoDetailPo};
use repository::schema::public::t_room_info_detail::dsl::t_room_info_detail;
use std::collections::{HashMap, HashSet};


///
/// 根据入参的month_version获取上一个月的month_version
/// eg. HSMZ-2025-5 -> (HSMZ-2025-5, Option(HSMZ-2025-4))
///
pub fn verify_month_version_and_get_last_month_version(month_version: &str) -> Option<(String, String)> {
    Regex::new(r"\w+-(?P<year>\d+)-(?P<month>\d+)").unwrap().captures(month_version)
        .map(|cap| {
            if let (Some(year), Some(month)) = (cap.name("year").map(|e| e.as_str()), cap.name("month").map(|m| m.as_str())) {
                let year = year.parse::<i32>().unwrap_or_default();
                let month = month.parse::<u32>().unwrap_or_default();
                Some(Local::now().with_year(year).map(|e| e.with_month(month)).flatten().unwrap())
            } else {
                None
            }
        }).flatten()
        .map(|time| {
            let last_version = init_current_month_version(time.checked_sub_months(Months::new(1)).unwrap());
            Some((month_version.to_string(), last_version))
        }).flatten()
}

pub fn init_room_data(month_version: &str) -> AppResult<()> {
    let (current_version,last_version) = if let Some((current_version, last_version)) = verify_month_version_and_get_last_month_version(month_version) {
        if current_version == last_version {
            return Err(BUSINESS_ERROR("月份版本不合法", 23));
        }
        (current_version, last_version)
    }else{
        return Err(BUSINESS_ERROR("月份版本不合法", 23));
    };

    //获取上月的读数
    let last_room_info_data;
    {
        use repository::schema::public::t_room_info_detail::*;
        last_room_info_data = table
            .filter(is_delete.eq(false))
            .filter(month_version.eq(last_version))
            .get_results::<RoomInfoDetailPo>(&mut db_get_connection())?;
    }

    // 获取业主数据
    let owner_info_data;
    {
        use repository::schema::public::t_owner_basic_info::*;
        owner_info_data = table
            .filter(is_delete.eq(false))
            .get_results::<OwnerBasicInfoPo>(&mut db_get_connection())?;
    }
    //获取已有水电数据
    let exists_room_info;
    {
        use repository::schema::public::t_room_info_detail::*;
        exists_room_info = table
            .filter(is_delete.eq(false))
            .filter(month_version.eq(&current_version))
            .select(room_number)
            .get_results::<Option<String>>(&mut db_get_connection())?
            .into_iter().flat_map(|e| {
            match e {
                None => { None }
                Some(e) => { Some(e) }
            }
        }).collect::<HashSet<String>>();
    }
    // 将last_room_info_data transform to map
    let last_room_info_map = last_room_info_data.iter()
        // .filter(|room| room.room_number != None)
        .flat_map(|room| {
            if let Some(ref num) = room.room_number {
                Some((num.as_str(), room))
            } else {
                None
            }
        }).collect::<HashMap<&str, &RoomInfoDetailPo>>();


    let now = Some(now_local_date_time_naive());
    let data_init = owner_info_data.iter()
        .filter(|e| !exists_room_info.contains(&e.room_number))
        .map(|owner_info| {
            let last_room_data = last_room_info_map.get(owner_info.room_number.as_str());
            RoomInfoDetailInsertPo {
                room_number: Some(owner_info.room_number.as_str()),
                room_owner_name: owner_info.owner_name.as_deref(),
                water_meter_num_before: last_room_data.map(|e| e.water_meter_num).flatten(),
                water_meter_num: None,
                water_meter_sub: None,
                electricity_meter_num_before: last_room_data.map(|e| e.electricity_meter_num).flatten(),
                electricity_meter_num: None,
                electricity_meter_sub: None,
                month_version: Some(current_version.as_str()),
                comment: None,
                create_by: Some(CURRENT_USE),
                update_by: Some(CURRENT_USE),
                create_time: now,
                update_time: now,
                delete_at: Some(chrono::NaiveDateTime::UNIX_EPOCH),
            }
        }).collect::<Vec<RoomInfoDetailInsertPo>>();

    insert_into(t_room_info_detail)
        .values(data_init)
        .execute(&mut db_get_connection())?;


    //基于业主数据和上月读数，生成新的水电数据
    Ok(())
}


///
/// 获取指定月份的版本
///
pub fn init_current_month_version(time: chrono::DateTime<Local>) -> String {
    //获取当前月份
    let month = time.month();
    let year = time.year();
    //目前只有海上明珠小区
    format!("HSMZ-{}-{}", year, month)
}