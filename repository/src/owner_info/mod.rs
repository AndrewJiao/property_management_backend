use bigdecimal::BigDecimal;
use crate::schema::basic::t_owner_basic_info;
use chrono::NaiveDateTime;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use management_macro::AutoOperation;
use serde::{Deserialize, Serialize};

#[derive(Selectable, Queryable, Deserialize, Serialize)]
#[diesel(table_name = t_owner_basic_info)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(rename_all = "camelCase")]
pub struct OwnerBasicInfoPo {
   pub  id: i32,
   pub  room_number: String,
   pub  owner_name: Option<String>,
   pub  room_square: Option<BigDecimal>,
   pub  create_by: Option<String>,
   pub  update_by: Option<String>,
   pub  create_time: NaiveDateTime,
   pub  update_time: NaiveDateTime,
   pub  is_delete: bool,
   pub  comment: Option<String>,
   pub  other_basic: Option<serde_json::Value>,
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
