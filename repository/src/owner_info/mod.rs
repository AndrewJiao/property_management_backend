use crate::price_basic::{BasicPriceType, PriceBasicPo};
use crate::schema::basic::t_owner_basic_info;
use crate::schema::basic::t_owner_basic_info::*;
use crate::SqlType;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use common::data_result::AppResult;
use common::db_config::{db_get_connection, Conn};
use common::error::DB_UPDATE_ERROR;
use diesel::pg::Pg;
use diesel::{AsChangeset, ExpressionMethods, Identifiable, Insertable, QueryDsl, Queryable, RunQueryDsl, Selectable, SelectableHelper};
use management_macro::AutoOperation;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Selectable, Queryable, Deserialize, Serialize, Debug)]
#[diesel(table_name = t_owner_basic_info)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(rename_all = "camelCase")]
pub struct OwnerBasicInfoPo {
    pub id: i32,
    pub room_number: String,
    pub owner_name: Option<String>,
    pub room_square: Option<BigDecimal>,
    pub create_by: Option<String>,
    pub update_by: Option<String>,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
    pub is_delete: bool,
    pub comment: Option<String>,
    pub other_basic: Option<serde_json::Value>,
    pub delete_at: Option<NaiveDateTime>,
    pub amount_balance: BigDecimal,

}

impl OwnerBasicInfoPo {
    pub fn by_room_number_flow(p_room_number: Option<&Vec<String>>, p1: i64, p2: i64) -> AppResult<Vec<OwnerBasicInfoPo>> {
        if let Some(p_room_number) = p_room_number {
            let result = Self::all()
                .filter(room_number.eq_any(p_room_number))
                .filter(is_delete.eq(false))
                .select(OwnerBasicInfoPo::as_select())
                .order_by(create_time.desc())
                .offset(p1)
                .limit(p2)
                .load::<OwnerBasicInfoPo>(&mut db_get_connection())?;
            Ok(result)
        } else {
            Ok(vec![])
        }
    }
}

type BoxedQuery<'a> = t_owner_basic_info::BoxedQuery<'a, Pg, SqlType<OwnerBasicInfoPo>>;
impl OwnerBasicInfoPo {
    pub fn all<'a>() -> BoxedQuery<'a> {
        table.select(OwnerBasicInfoPo::as_select()).into_boxed()
    }

    pub fn all_result() -> AppResult<Vec<OwnerBasicInfoPo>> {
        Ok(Self::all().filter(is_delete.eq(false)).load::<OwnerBasicInfoPo>(&mut db_get_connection())?)
    }

    pub fn by_room_number(param:&str,conn : &mut Conn)-> AppResult<OwnerBasicInfoPo>{
        Ok(Self::all().filter(room_number.eq(param)).filter(is_delete.eq(false)).first(conn)?)
    }

    pub fn calculate_management_fee(&self, basic_config: &HashMap<BasicPriceType, PriceBasicPo>) -> Option<BigDecimal> {
        let new_basic_price = basic_config.get(&BasicPriceType::ManagementFee).map(|info| info.basic_number.clone()).flatten();
        if let (Some(square), Some(fee)) = (&self.room_square, new_basic_price) {
            Some(fee * square)
        } else {
            None
        }
    }

    pub fn calculate_part_fee(&self, basic_config: &HashMap<BasicPriceType, PriceBasicPo>) -> Option<BigDecimal> {
        let new_car_basic_price = basic_config.get(&BasicPriceType::CarPartFee).map(|info| info.basic_number.clone()).flatten();
        let motor_basic_price = basic_config.get(&BasicPriceType::MotorCyclePartFee).map(|info| info.basic_number.clone()).flatten();

        if let (Some((car_num, motor_num)), (Some(ref car_pri), Some(ref motor_pri)))
            = (self.get_vehicle_num(), (new_car_basic_price, motor_basic_price))
        {
            Some((car_pri * car_num) + (motor_pri * motor_num))
        } else {
            None
        }
    }

    fn get_vehicle_num(&self) -> Option<(u64, u64)> {
        match self.other_basic {
            Some(Value::Object(ref map)) => {
                println!("carNumber={:?}", map.get("carNumber"));
                println!("motorCycleNumber={:?}", map.get("motorCycleNumber"));
                return Some((
                    map.get("carNumber").map(|value| value.as_u64()).flatten().unwrap_or(0),
                    map.get("motorCycleNumber").map(|value| value.as_u64()).flatten().unwrap_or(0),
                ));
            }
            _ => Some((0, 0))
        }
    }
}

#[derive(Identifiable, AsChangeset, Serialize, AutoOperation)]
#[diesel(table_name = t_owner_basic_info)]
pub struct UpdateOwnerBasicInfoPo<'a> {
    pub id: i32,
    pub room_number: Option<&'a str>,
    pub owner_name: Option<&'a str>,
    pub is_delete: Option<bool>,
    pub comment: Option<&'a str>,
    pub room_square: Option<&'a BigDecimal>,
    pub other_basic: Option<&'a serde_json::Value>,
    pub update_time: Option<NaiveDateTime>,
    pub delete_at: Option<NaiveDateTime>,
}

#[derive(Serialize, AutoOperation, Insertable)]
#[diesel(table_name = t_owner_basic_info)]
pub struct InsertOwnerBasicInfoPo<'a> {
    pub room_number: Option<&'a str>,
    pub owner_name: Option<&'a str>,
    pub room_square: Option<&'a BigDecimal>,
    pub create_by: Option<&'a str>,
    pub update_by: Option<&'a str>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
    pub is_delete: bool,
    pub comment: Option<&'a str>,
    pub other_basic: Option<serde_json::Value>,
    pub delete_at: Option<NaiveDateTime>,
    pub amount_balance: BigDecimal,
}

pub fn update_amount(param_id:i32, amount:&BigDecimal, conn:&mut Conn)->AppResult<()>{
    let update_count = diesel::update(table)
        .set(amount_balance.eq(amount))
        .filter(id.eq(param_id))
        .execute(conn)?;
    if update_count!=1 { Err(DB_UPDATE_ERROR()) }else { Ok(()) }
}
