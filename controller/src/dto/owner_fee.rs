use std::collections::HashSet;
use crate::dto::owner_fee::StreamAddDetailType::NoMatch;
use crate::dto::ToUpdatePO;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use log::info;
use common::tools::serde::empty_string_or_null_as_none;
use common::CURRENT_USE;
use repository::owner_fee::{CalculateType, DetailType, OwnerFeeDetailPo, OwnerFeeDetailUpdatePo, StreamId};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OwnerFeeDetailDto {
    pub id: i64,
    pub stream_id: StreamId,
    pub room_number: String,
    pub owner_name: Option<String>,
    pub detail_type: DetailType,
    pub amount: BigDecimal,
    pub comment: Option<String>,
    pub create_by: String,
    pub update_by: String,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
    pub is_delete: bool,
}

#[derive(Deserialize, Serialize, Validate, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct OwnerFeeDetailSearchDto {
    #[validate(length(min = 0, max = 100))]
    #[serde(default, deserialize_with = "empty_string_or_null_as_none")]
    pub stream_id: Option<String>,
    #[serde(default, deserialize_with = "empty_string_or_null_as_none")]
    #[validate(length(min = 0, max = 100))]
    pub room_number: Option<String>,
    pub room_numbers: Option<Vec<String>>,
    pub detail_type: Option<Vec<DetailType>>,
    pub create_time_star: Option<NaiveDateTime>,
    pub create_time_end: Option<NaiveDateTime>,
    pub update_time_star: Option<NaiveDateTime>,
    pub update_time_end: Option<NaiveDateTime>,
}

#[derive(Deserialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct OwnerFeeDetailUpdateDto {
    #[validate(custom(function = "common::tools::validator::validate_big_decimal"))]
    pub amount: Option<BigDecimal>,
    #[validate(length(min = 0, max = 100))]
    #[serde(default, deserialize_with = "empty_string_or_null_as_none")]
    pub comment: Option<String>,
}
impl ToUpdatePO for OwnerFeeDetailUpdateDto {
    type PO<'a> = OwnerFeeDetailUpdatePo<'a>;

    fn to_update_po(&self, id: i64) -> OwnerFeeDetailUpdatePo {
        OwnerFeeDetailUpdatePo {
            id,
            amount: self.amount.as_ref(),
            comment: self.comment.as_deref(),
            update_by: Some(CURRENT_USE),
            update_time: None,
            is_delete: None,
            settle_down_order_number: None,
        }
    }
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OwnerFeeDetailResultDto {
    pub id: i64,
    pub stream_id: String,
    pub room_number: String,
    pub owner_name: Option<String>,
    pub detail_type: DetailType,
    pub calculate_type:CalculateType,
    pub detail_type_desc:&'static str,
    pub amount: BigDecimal,
    pub amount_balance: BigDecimal,
    pub comment: Option<String>,
    pub create_by: String,
    pub update_by: String,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
    pub related_order_number: String,
    pub settle_type: SettleType
}

impl OwnerFeeDetailResultDto {
    pub fn new(po: OwnerFeeDetailPo, amount_balance: BigDecimal, all_hash_relative_stream_data_id: &HashSet<String>) -> Self {
        let has_settled:bool = all_hash_relative_stream_data_id.contains(po.stream_id.as_str());
        Self {
            id: po.id,
            stream_id: po.stream_id,
            room_number: po.room_number,
            owner_name: po.owner_name,
            detail_type_desc: po.detail_type.desc(),
            calculate_type:po.detail_type.calculate_type(),
            settle_type: SettleType::check_settle_type(&po.detail_type, has_settled),
            detail_type: po.detail_type,
            amount: po.amount,
            comment: po.comment,
            create_by: po.create_by,
            update_by: po.update_by,
            create_time: po.create_time,
            update_time: po.update_time,
            amount_balance,
            related_order_number: po.related_order_number,
        }
    }
}

///
/// 是否已结算,用于前端展示是否需要结算
///
#[derive(Deserialize, Serialize, Debug)]
pub enum SettleType{
    //未结算
    Settled,
    //已结算
    NotSettle,
    //无需结算
    NoNeedSettle
}
impl SettleType{
    fn check_settle_type(detail_type: &DetailType, has_settled:bool)->Self{
        match detail_type {
            DetailType::ManagementFee|DetailType::LiquidatedDamages => {
                if has_settled {
                    SettleType::Settled
                } else {
                    SettleType::NotSettle
                }
            }
            DetailType::PreStoreFee | DetailType::SettlementFee | DetailType::PreStoreDeduction | DetailType::AdjustOrder => SettleType::NoNeedSettle
        }
    }

}

pub enum StreamAddDetailType {
    //物业费
    ManagementFee(OwnerFeeAssignedAddDto),
    //批量物业费
    ManagementFeeBatch(OwnerFeeAssignedAddDtos),
    //调整单
    AdjustOrder(OwnerFeeAssignedManuallyAddDto),
    //结算
    SettlementFee(OwnerFeeAssignedManuallySettleDto),
    //
    NoMatch
}


impl From<serde_json::Value> for StreamAddDetailType{
    fn from(value: Value) -> Self {
        //必须要有detailType
        let detail_type = value["detailType"].as_str().unwrap_or("");
        info!("detail_type:{}", detail_type);
        match detail_type {
            "ManagementFee" => serde_json::from_value(value).map(|e| StreamAddDetailType::ManagementFee(e)).unwrap_or(NoMatch),
            "ManagementFeeBatch" => serde_json::from_value(value).map(|e| StreamAddDetailType::ManagementFeeBatch(e)).unwrap_or(NoMatch),
            "AdjustOrder" => serde_json::from_value(value).map(|e| StreamAddDetailType::AdjustOrder(e)).unwrap_or(NoMatch),
            "SettlementFee" => serde_json::from_value(value).map(|e| StreamAddDetailType::SettlementFee(e)).unwrap_or(NoMatch),
            _ => NoMatch,
        }
    }
}

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct OwnerFeeAssignedManuallySettleDto {
    #[validate(length(min = 0, max = 100))]
    pub stream_id: String,
    pub settle_amount: BigDecimal,
}


#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct OwnerFeeAssignedManuallyAddDto {
    #[validate(custom(function = "common::tools::validator::validate_big_decimal"))]
    pub amount: BigDecimal,
    #[validate(length(min = 0, max = 100))]
    pub room_number: String,
}

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct OwnerFeeAssignedAddDto {
    #[validate(length(min = 0, max = 100))]
    pub room_number: String,
    #[validate(length(min = 0, max = 100))]
    pub version: String,
}
#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct OwnerFeeAssignedAddDtos {
    #[validate(length(min = 0, max = 100))]
    pub version: String,
}
