use crate::owner_info::RoomType;
use crate::price_basic::{BasicPriceType, PriceBasicPo};
use crate::schema::public::t_room_info_detail;
use crate::schema::public::t_room_info_detail::*;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use common::data_result::AppResult;
use common::db_config::{db_get_connection, Conn};
use common::tools::time::DEFAULT_TIME;
use diesel::dsl::auto_type;
use diesel::pg::Pg;
use diesel::sql_types::Bool;
use diesel::{AsChangeset, BoolExpressionMethods, ExpressionMethods, Identifiable, Insertable, QueryDsl, Queryable, RunQueryDsl, Selectable, SelectableHelper};
use log::debug;
use management_macro::AutoOperation;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;

#[auto_type(no_type_alias)]
pub fn with_not_completed()->_{
    water_meter_sub.is_null().and(electricity_meter_sub.is_null())
}

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


type BoxedQuery<'a> = t_room_info_detail::BoxedQuery<'a,Pg,crate::SqlType<RoomInfoDetailPo>>;
impl RoomInfoDetailPo {
    pub fn all<'a>() -> BoxedQuery<'a>{
        table.select(RoomInfoDetailPo::as_select()).into_boxed()
    }

    pub fn by_room_number_and_version(p_room_number: &str, p_version: &str) -> AppResult<Option<RoomInfoDetailPo>> {
         let result = table
            .select(RoomInfoDetailPo::as_select())
            .filter(room_number.eq(p_room_number))
            .filter(month_version.eq(p_version))
            .filter(is_delete.eq(false))
            .get_result(&mut db_get_connection()).ok();
        Ok(result)
    }


    pub fn by_room_number_flow(
        p_room_number: Option<Vec<&str>>
        , create_time_range: Option<(NaiveDateTime, NaiveDateTime)>
        , only_not_completed: Option<bool>
        , offset: i64, limit: i64)
        -> AppResult<Vec<RoomInfoDetailPo>> {
        let create_range_time = create_time_range.unwrap_or((*DEFAULT_TIME, *DEFAULT_TIME));
        if let Some(p_room_number) = p_room_number {
            let result = table
                .select(RoomInfoDetailPo::as_select())
                .filter(room_number.eq_any(p_room_number))
                .filter(diesel::dsl::sql::<Bool>(if create_time_range.is_none() { "TRUE" } else { "FALSE" }).or(create_time.between(create_range_time.0, create_range_time.1)))
                .filter(diesel::dsl::sql::<Bool>(if only_not_completed.unwrap_or_default() { "FALSE" } else { "TRUE" }).or(with_not_completed()))
                .filter(is_delete.eq(false))
                .offset(offset * limit)
                .limit(limit)
                .order_by(create_time.desc())
                .load(&mut db_get_connection())?;
            Ok(result)
        } else {
            Ok(vec![])
        }
    }
}



impl RoomInfoDetailPo {
    pub fn calculate_lift(&self, basic: Option<BigDecimal>, plus: Option<BigDecimal>) -> Option<BigDecimal> {
        //写一个正则，匹配A081,B203这种门牌号
        let pattern = &Regex::new(r"[A-Z](?P<floor>\d)\d{2}").unwrap();

        if let (Some(basic), Some(plus)) = (basic, plus) {
            if let Some(ref room_num) = self.room_number {
                if let Some(ref capture) = pattern.captures(room_num) {
                    //按楼层-1计算
                    let floor_num = i32::from_str(&capture["floor"]).unwrap() - 1;
                    debug!("room_number:{} floor_num:{} plus:{} basic:{}", room_num,floor_num, plus, basic);
                    return Some(basic + (floor_num * plus));
                }
            }
        }
        None
    }
}

#[derive(Insertable, Serialize, AutoOperation)]
#[diesel(table_name = t_room_info_detail)]
pub struct RoomInfoDetailInsertPo<'a> {
    pub room_number: Option<&'a str>,
    pub water_meter_num_before: Option<i64>,
    pub water_meter_num: Option<i64>,
    pub water_meter_sub: Option<i64>,
    pub electricity_meter_num_before: Option<i64>,
    pub electricity_meter_num: Option<i64>,
    pub electricity_meter_sub: Option<i64>,
    pub month_version: Option<&'a str>,
    pub comment: Option<&'a str>,
    pub create_by: Option<&'a str>,
    pub update_by: Option<&'a str>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
    pub room_owner_name: Option<&'a str>,
    pub delete_at: Option<NaiveDateTime>,
}
impl RoomInfoDetailInsertPo<'_>{
    pub fn save(self, conn: &mut Conn) -> AppResult<()> {
        let _ = diesel::insert_into(t_room_info_detail::table)
            .values(self)
            .execute(conn)?;
        Ok(()) 
    }
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
    ///
    /// 电费分为商用和民用
    /// 公摊和个人
    /// 两个维度
    /// 其中，用户电费分摊用的是房间面积
    /// 商户电费分摊用的是商户的电表数
    ///
    pub fn calculate_electric(&self, basic_config: &HashMap<BasicPriceType, PriceBasicPo>, room_square: Option<&BigDecimal>, room_type: RoomType) -> (Option<BigDecimal>, Option<BigDecimal>) {
        let (electric_price, electric_share_price) = match room_type {
            RoomType::Common => (
                basic_config.get(&BasicPriceType::ElectricFee).and_then(|info| info.basic_number.clone()),
                basic_config.get(&BasicPriceType::ElectricShareFee).and_then(|info| info.basic_number.clone())
            ),
            RoomType::Business => (
                basic_config.get(&BasicPriceType::BusinessElectricFee).and_then(|info| info.basic_number.clone()),
                basic_config.get(&BasicPriceType::BusinessElectricShareFee).and_then(|info| info.basic_number.clone())
            )
        };

        let ele_total = self.electricity_meter_sub.and_then(|electric_num| {
            electric_price.map(|electric_pri| electric_pri * electric_num)
        });

        let ele_share = match (electric_share_price, room_square, room_type) {
            (Some(share_price), Some(square), RoomType::Common) => Some(share_price * square),
            (Some(share_price), _, RoomType::Business) => Some(share_price * self.electricity_meter_sub.unwrap_or_default()),
            _ => None,
        };

        (ele_total, ele_share)
    }

    pub fn calculate_water(&self, basic_config: &HashMap<BasicPriceType, PriceBasicPo>, room_type: RoomType) -> Option<(BigDecimal, BigDecimal)> {
        let (water_price, water_share_price) = match room_type {
            RoomType::Common => (
                basic_config.get(&BasicPriceType::WaterFee).map(|info| info.basic_number.clone()).flatten(),
                basic_config.get(&BasicPriceType::WaterShareFee).map(|info| info.basic_number.clone()).flatten()
            ),
            RoomType::Business => (
                basic_config.get(&BasicPriceType::BusinessWaterFee).map(|info| info.basic_number.clone()).flatten(),
                basic_config.get(&BasicPriceType::BusinessWaterShareFee).map(|info| info.basic_number.clone()).flatten()
            )
        };

        if let (Some(water_num), Some(water_pri), Some(water_share_pri))
            = (self.water_meter_sub, water_price, water_share_price)
        {
            let water_total = water_pri * water_num;
            let water_share = water_share_pri * water_num;
            Some((water_total, water_share))
        } else {
            None
        }
    }
}


pub trait ReCalculator{
    fn re_calculate(self) -> Self;
}
impl ReCalculator for RoomInfoDetailUpdatePo<'_> {
    fn re_calculate(mut self) -> Self {
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

impl ReCalculator for RoomInfoDetailInsertPo<'_> {
    fn re_calculate(mut self) -> Self {
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

