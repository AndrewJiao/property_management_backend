use std::collections::HashMap;
use std::ops::Mul;
use bigdecimal::BigDecimal;
use crate::schema::basic::t_owner_basic_info;
use chrono::NaiveDateTime;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use management_macro::AutoOperation;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::price_basic::{BasicPriceType, PriceBasicPo};

#[derive(Selectable, Queryable, Deserialize, Serialize)]
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
}

impl OwnerBasicInfoPo {
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
            Some(Value::Object(ref map)) => Some((
                map.get("carNumber").map(|value| value.as_u64()).flatten().unwrap_or(0),
                map.get("motorCycleNumber").map(|value| value.as_u64()).flatten().unwrap_or(0)
            )),
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
}
