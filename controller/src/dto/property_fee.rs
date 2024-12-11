use common::tools::serde::empty_string_or_null_as_none;
use crate::dto::ToUpdatePO;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use repository::property_fee::PropertyFeeDetailUpdatePo;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PropertyFeeDetailUpdateDto {
    #[validate(custom(function = "common::tools::validator::validate_big_decimal"))]
    pub management_fee: Option<BigDecimal>,
    #[validate(custom(function = "common::tools::validator::validate_big_decimal"))]
    pub part_fee: Option<BigDecimal>,
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
}

impl ToUpdatePO for PropertyFeeDetailUpdateDto {
    type PO<'a> = PropertyFeeDetailUpdatePo<'a>;

    fn to_update_po(&self, id: i64) -> Self::PO<'_> {
        PropertyFeeDetailUpdatePo {
            id,
            management_fee: self.management_fee.as_ref(),
            part_fee: self.part_fee.as_ref(),
            machine_room_renovation_fee: self.machine_room_renovation_fee.as_ref(),
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
