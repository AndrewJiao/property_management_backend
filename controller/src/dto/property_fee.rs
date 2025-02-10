use crate::dto::ToUpdatePO;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use common::tools::serde::empty_string_or_null_as_none;
use repository::property_fee::{PropertyFeeDetailPo, PropertyFeeDetailUpdatePo};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use validator::Validate;

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PropertyFeeDetailUpdateDto {
    #[validate(custom(function = "common::tools::validator::validate_big_decimal"))]
    pub management_fee: Option<BigDecimal>,
    #[validate(custom(function = "common::tools::validator::validate_big_decimal"))]
    pub part_fee: Option<BigDecimal>,
    #[validate(custom(function = "common::tools::validator::validate_big_decimal"))]
    pub lift_fee: Option<BigDecimal>,
    #[validate(custom(function = "common::tools::validator::validate_big_decimal"))]
    pub machine_room_renovation_fee: Option<BigDecimal>,
    #[validate(custom(function = "common::tools::validator::validate_big_decimal"))]
    pub electric_fee: Option<BigDecimal>,
    #[validate(custom(function = "common::tools::validator::validate_big_decimal"))]
    pub electric_share_fee: Option<BigDecimal>,
    #[validate(custom(function = "common::tools::validator::validate_big_decimal"))]
    pub water_fee: Option<BigDecimal>,
    #[validate(custom(function = "common::tools::validator::validate_big_decimal"))]
    pub water_share_fee: Option<BigDecimal>,
    #[validate(custom(function = "common::tools::validator::validate_big_decimal"))]
    pub liquidate_fee: Option<BigDecimal>,
    #[validate(custom(function = "common::tools::validator::validate_big_decimal"))]
    pub pre_store_fee: Option<BigDecimal>,
    #[validate(length(min = 0, max = 1000))]
    pub comment: Option<String>,
}


#[derive(Deserialize, Validate,Debug, Clone,Default)]
#[serde(rename_all = "camelCase")]
pub struct PropertyFeeDetailSearchDto {
    #[validate(length(min = 0, max = 100))]
    pub room_number: Option<String>,
    #[validate(length(min = 0, max = 100))]
    pub room_owner_name: Option<String>,
    #[validate(length(min = 0, max = 100))]
    pub record_version: Option<String>,
    pub create_time_star: Option<NaiveDateTime>,
    pub create_time_end: Option<NaiveDateTime>,
    pub update_time_star: Option<NaiveDateTime>,
    pub update_time_end: Option<NaiveDateTime>,
    pub is_settle_down: Option<bool>,
}

impl ToUpdatePO for PropertyFeeDetailUpdateDto {
    type PO<'a> = PropertyFeeDetailUpdatePo<'a>;

    fn to_update_po(&self, id: i64) -> Self::PO<'_> {
        PropertyFeeDetailUpdatePo {
            id,
            management_fee: self.management_fee.as_ref(),
            part_fee: self.part_fee.as_ref(),
            machine_room_renovation_fee: self.machine_room_renovation_fee.as_ref(),
            lift_fee: self.lift_fee.as_ref(),
            electric_fee: self.electric_fee.as_ref(),
            electric_share_fee: self.electric_share_fee.as_ref(),
            water_fee: self.water_fee.as_ref(),
            water_share_fee: self.water_share_fee.as_ref(),
            liquidate_fee: self.liquidate_fee.as_ref(),
            pre_store_fee: self.pre_store_fee.as_ref(),
            update_by: None,
            update_time: None,
            is_delete: None,
            delete_at: None,
            comment: self.comment.as_deref(),
            total_fee: None,
            is_settle_down: None,
        }
    }
}
#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PropertyFeeDetailInitDto {
    #[validate(length(min = 0, max = 100))]
    #[serde(deserialize_with = "empty_string_or_null_as_none")]
    pub month_version: Option<String>,
}


#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PropertyFeeDetailResultDto {
    pub id: i64,
    pub room_number: Option<String>,
    pub room_owner_name: Option<String>,
    pub management_fee: Option<BigDecimal>,
    pub part_fee: Option<BigDecimal>,
    pub lift_fee: Option<BigDecimal>,
    pub machine_room_renovation_fee: Option<BigDecimal>,
    pub electric_fee: Option<BigDecimal>,
    pub electric_share_fee: Option<BigDecimal>,
    pub water_fee: Option<BigDecimal>,
    pub water_share_fee: Option<BigDecimal>,
    pub liquidate_fee: Option<BigDecimal>,
    pub pre_store_fee: Option<BigDecimal>,
    pub record_version: Option<String>,
    pub create_by: String,
    pub update_by: String,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
    pub is_delete: bool,
    pub delete_at: Option<NaiveDateTime>,
    pub comment: Option<String>,
    pub total_fee: Option<BigDecimal>,
    pub related_order_number: Option<String>,
    pub is_settle_down: bool,
}
impl From<PropertyFeeDetailPo> for PropertyFeeDetailResultDto{
    fn from(po: PropertyFeeDetailPo) -> Self {
        Self{
            id: po.id,
            room_number: po.room_number,
            room_owner_name: po.room_owner_name,
            management_fee: po.management_fee,
            part_fee: po.part_fee,
            lift_fee: po.lift_fee,
            machine_room_renovation_fee: po.machine_room_renovation_fee,
            electric_fee: po.electric_fee,
            electric_share_fee: po.electric_share_fee,
            water_fee: po.water_fee,
            water_share_fee: po.water_share_fee,
            liquidate_fee: po.liquidate_fee,
            pre_store_fee: po.pre_store_fee,
            record_version: po.record_version,
            create_by: po.create_by,
            update_by: po.update_by,
            create_time: po.create_time,
            update_time: po.update_time,
            is_delete: po.is_delete,
            delete_at: po.delete_at,
            comment: po.comment,
            total_fee: po.total_fee,
            related_order_number: None,
            is_settle_down: po.is_settle_down,
        }
    }
}
impl PropertyFeeDetailResultDto{
    pub fn from_vec(po: Vec<PropertyFeeDetailPo>, stream_map:&HashMap<String,String>) -> Vec<Self> {
        let mut results:Vec<Self> = po.into_iter().map(|e| e.into()).collect();
        let _ = results.iter_mut().for_each(|e| {
            if let (Some(room_num),Some(record)) = (&e.room_number, &e.record_version) {
                let key = &format!("{}-{}", room_num, record);
                e.related_order_number = stream_map.get(key) .map(|e| e.to_string());
            }
        });
        results
    }
}