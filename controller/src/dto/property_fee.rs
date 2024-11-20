use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use common::tools::serde::empty_string_or_null_as_none;
use serde::Deserialize;
use validator::Validate;
// #[derive(Serialize)]
// #[serde(rename_all = "camelCase")]
// pub struct PropertyFeeDetailResultDto {
//     pub id: i32,
//     pub room_number: Option<String>,
//     pub room_owner_name: Option<String>,
//     pub management_fee: Option<BigDecimal>,
//     pub part_fee: Option<BigDecimal>,
//     pub machine_room_renovation_fee: Option<BigDecimal>,
//     pub electric_fee: Option<BigDecimal>,
//     pub electric_share_fee: Option<BigDecimal>,
//     pub water_fee: Option<BigDecimal>,
//     pub water_share_fee: Option<BigDecimal>,
//     pub liquidate_fee: Option<BigDecimal>,
//     pub pre_store_fee: Option<BigDecimal>,
//     pub recode_version: Option<String>,
//     pub create_by: Option<String>,
//     pub update_by: Option<String>,
//     pub create_time: NaiveDateTime,
//     pub update_time: NaiveDateTime,
//     pub is_delete: Option<bool>,
//     pub delete_at: Option<NaiveDateTime>,
//     pub comment: Option<String>,
//     pub total_fee: Option<BigDecimal>,
// }

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
    #[validate(length(min = 0, max = 100))]
    #[serde(deserialize_with = "empty_string_or_null_as_none")]
    pub comment: Option<String>,
}


#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PropertyFeeDetailSearchDto {
    #[validate(length(min = 0, max = 100))]
    #[serde(deserialize_with = "empty_string_or_null_as_none")]
    pub room_number: Option<String>,
    #[validate(length(min = 0, max = 100))]
    #[serde(deserialize_with = "empty_string_or_null_as_none")]
    pub room_owner_name: Option<String>,
    #[validate(length(min = 0, max = 100))]
    #[serde(deserialize_with = "empty_string_or_null_as_none")]
    pub record_version: Option<String>,
    pub create_time_begin: Option<NaiveDateTime>,
    pub create_time_end: Option<NaiveDateTime>,
    pub update_time_begin: Option<NaiveDateTime>,
    pub update_time_end: Option<NaiveDateTime>,
}
