use bigdecimal::BigDecimal;
use crate::schema::basic::t_property_fee_detail;
use chrono::NaiveDateTime;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use management_macro::AutoOperation;
use serde::{Deserialize, Serialize};

#[derive(Selectable, Queryable, Serialize, Deserialize)]
#[diesel(table_name = t_property_fee_detail)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(rename_all = "camelCase")]
pub struct PropertyFeeDetailPo {
    pub id: i32,
    pub room_number: Option<String>,
    pub room_owner_name: Option<String>,
    pub management_fee: Option<BigDecimal>,
    pub part_fee: Option<BigDecimal>,
    pub machine_room_renovation_fee: Option<BigDecimal>,
    pub electric_fee: Option<BigDecimal>,
    pub electric_share_fee: Option<BigDecimal>,
    pub water_fee: Option<BigDecimal>,
    pub water_share_fee: Option<BigDecimal>,
    pub liquidate_fee: Option<BigDecimal>,
    pub pre_store_fee: Option<BigDecimal>,
    pub record_version: Option<String>,
    pub create_by: Option<String>,
    pub update_by: Option<String>,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
    pub is_delete: Option<bool>,
    pub delete_at: Option<NaiveDateTime>,
    pub comment: Option<String>,
    pub total_fee: Option<BigDecimal>,
}
#[derive(Identifiable, AsChangeset, AutoOperation)]
#[diesel(table_name = t_property_fee_detail)]
pub struct PropertyFeeDetailUpdatePo<'a> {
    pub id: i32,
    pub management_fee: Option<BigDecimal>,
    pub part_fee: Option<BigDecimal>,
    pub machine_room_renovation_fee: Option<BigDecimal>,
    pub electric_fee: Option<BigDecimal>,
    pub electric_share_fee: Option<BigDecimal>,
    pub water_fee: Option<BigDecimal>,
    pub water_share_fee: Option<BigDecimal>,
    pub liquidate_fee: Option<BigDecimal>,
    pub pre_store_fee: Option<BigDecimal>,
    pub update_by: Option<&'a str>,
    pub update_time: Option<NaiveDateTime>,
    pub is_delete: Option<bool>,
    pub delete_at: Option<NaiveDateTime>,
    pub comment: Option<&'a str>,
    pub total_fee: Option<BigDecimal>,
}

#[derive(Insertable, AutoOperation)]
#[diesel(table_name = t_property_fee_detail)]
pub struct PropertyFeeDetailInsertPo<'a> {
    pub room_number: &'a str,
    pub room_owner_name: &'a str,
    pub management_fee: Option<BigDecimal>,
    pub part_fee: Option<BigDecimal>,
    pub machine_room_renovation_fee: Option<BigDecimal>,
    pub electric_fee: Option<BigDecimal>,
    pub electric_share_fee: Option<BigDecimal>,
    pub water_fee: Option<BigDecimal>,
    pub water_share_fee: Option<BigDecimal>,
    pub liquidate_fee: Option<BigDecimal>,
    pub pre_store_fee: Option<BigDecimal>,
    pub record_version: &'a str,
    pub create_by: &'a str,
    pub update_by: &'a str,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
    pub is_delete: bool,
    pub delete_at: Option<NaiveDateTime>,
    pub comment: Option<&'a str>,
    pub total_fee: Option<BigDecimal>,
}
