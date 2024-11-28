use crate::price_basic::{BasicPriceType, PriceBasicPo};
use crate::schema::basic::t_room_info_detail;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use management_macro::AutoOperation;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    pub room_owner_name: Option<String>,
    pub delete_at: Option<NaiveDateTime>,
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
    pub room_owner_name: Option<&'a str>,
    pub delete_at: Option<NaiveDateTime>,
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

impl<'a> RoomInfoDetailUpdatePo<'a> {
    pub fn full_filed(mut self, info_po: &'a RoomInfoDetailPo) -> Self {
        if self.water_meter_num_before.is_none() { self.water_meter_num_before = info_po.water_meter_num_before.as_ref()};
        if self.water_meter_num.is_none(){ self.water_meter_num =  info_po.water_meter_num.as_ref()};
        if self.electricity_meter_num_before.is_none(){self.electricity_meter_num_before = info_po.electricity_meter_num_before.as_ref()};
        if self.electricity_meter_num.is_none(){self.electricity_meter_num = info_po.electricity_meter_num.as_ref()};
        self
    }
}

impl RoomInfoDetailPo {
    pub fn calculate_electric(&self, basic_config: &HashMap<BasicPriceType, PriceBasicPo>) -> Option<(BigDecimal, BigDecimal)> {
        let electric_price = basic_config.get(&BasicPriceType::ElectricFee).map(|info| info.basic_number.clone()).flatten();
        let electric_share_price = basic_config.get(&BasicPriceType::ElectricShareFee).map(|info| info.basic_number.clone()).flatten();

        if let (Some(electric_num), Some(electric_pri), Some(electric_share_pri))
            = (self.electricity_meter_sub, electric_price, electric_share_price)
        {
            let ele_total = electric_pri * electric_num;
            let ele_share = ele_total.clone() * electric_share_pri;
            Some((ele_total, ele_share))
        } else {
            None
        }
    }

    pub fn calculate_water(&self, basic_config: &HashMap<BasicPriceType, PriceBasicPo>) -> Option<(BigDecimal, BigDecimal)> {
        let water_price = basic_config.get(&BasicPriceType::WaterFee).map(|info| info.basic_number.clone()).flatten();
        let water_share_price = basic_config.get(&BasicPriceType::WaterShareFee).map(|info| info.basic_number.clone()).flatten();

        if let (Some(water_num), Some(water_pri), Some(water_share_pri))
            = (self.water_meter_sub, water_price, water_share_price)
        {
            let water_total = water_pri * water_num;
            let water_share = water_total.clone() * water_share_pri;
            Some((water_total, water_share))
        } else {
            None
        }
    }
}

impl RoomInfoDetailUpdatePo<'_> {
    pub fn re_calculate(mut self) -> Self {
        if let (Some(before), Some(now)) = (self.water_meter_num_before, self.water_meter_num) {
            self.water_meter_sub = Some(now - before);
        } else if let (None, Some(now)) = (self.water_meter_num_before, self.water_meter_num) {
            self.water_meter_sub = Some(now.clone());
        }else{
            self.water_meter_sub = None;
        }

        self.electricity_meter_sub = match
        (self.electricity_meter_num_before, self.electricity_meter_num) {
            (Some(before), Some(now)) => Some(now - before),
            (None, Some(now)) => Some(now.clone()),
            _ => None,
        };
        self
    }
}

