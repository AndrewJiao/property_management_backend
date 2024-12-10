use crate::dto::ToUpdatePO;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use common::tools::serde::empty_string_or_null_as_none;
use common::CURRENT_USE;
use repository::owner_fee::{DetailType, OwnerFeeDetailPo, OwnerFeeDetailUpdatePo, StreamId};
use serde::{Deserialize, Serialize};
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
    #[serde(deserialize_with = "empty_string_or_null_as_none")]
    pub stream_id: Option<String>,
    #[validate(length(min = 0, max = 100))]
    #[serde(deserialize_with = "empty_string_or_null_as_none")]
    pub room_number: Option<String>,
    pub detail_type: Option<DetailType>,
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
    #[serde(deserialize_with = "empty_string_or_null_as_none")]
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
    pub amount: BigDecimal,
    pub amount_balance: BigDecimal,
    pub comment: Option<String>,
    pub create_by: String,
    pub update_by: String,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
}

impl OwnerFeeDetailResultDto {
    pub fn new(po: OwnerFeeDetailPo, amount_balance: BigDecimal) -> Self {
        Self {
            id: po.id,
            stream_id: po.stream_id,
            room_number: po.room_number,
            owner_name: po.owner_name,
            detail_type: po.detail_type,
            amount: po.amount,
            comment: po.comment,
            create_by: po.create_by,
            update_by: po.update_by,
            create_time: po.create_time,
            update_time: po.update_time,
            amount_balance,
        }
    }
}
