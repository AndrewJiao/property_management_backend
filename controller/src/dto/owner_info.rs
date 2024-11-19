use crate::dto::ToUpdatePO;
use chrono::NaiveDateTime;
use common::tools::serde::empty_string_or_null_as_none;
use common::tools::serde::json_verify;
use repository::owner_info::{AppJson, UpdateOwnerBasicInfoPo};
use serde::{Deserialize, Serialize};
use validator::Validate;

///
/// 作为业主表分页查询条件
///
///
#[derive(Deserialize, Serialize, Validate)]
pub struct OwnerInfoSearchDto {
    #[validate(length(min = 0, max = 100))]
    #[serde(deserialize_with = "empty_string_or_null_as_none")]
    pub room_number: Option<String>,
    #[validate(length(min = 0, max = 100))]
    #[serde(deserialize_with = "empty_string_or_null_as_none")]
    pub owner_name: Option<String>,
}


#[derive(Deserialize, Serialize, Validate)]
pub struct OwnerInfoUpdateDto {
    pub id: i32,
    #[validate(length(min = 0, max = 100))]
    #[serde(deserialize_with = "empty_string_or_null_as_none")]
    pub room_number: Option<String>,
    #[validate(length(min = 0, max = 100))]
    #[serde(deserialize_with = "empty_string_or_null_as_none")]
    pub owner_name: Option<String>,
    #[validate(length(min = 0, max = 100))]
    #[serde(deserialize_with = "empty_string_or_null_as_none")]
    pub comment: Option<String>,
    #[serde(deserialize_with = "json_verify")]
    pub other_basic: Option<serde_json::Value>,
}
// impl ToUpdatePO for OwnerInfoUpdateDto {
//     type PO<'a> = UpdateOwnerBasicInfoPo<'a>;
//
//     fn to_update_po(&self, id: i32) -> Self::PO<'_> {
//         let other_basic = match self.other_basic {
//             None => { None }
//             Some(ref value) => { Some(AppJson(value.clone())) }
//         };
//         UpdateOwnerBasicInfoPo {
//             id,
//             room_number: self.room_number.as_deref(),
//             owner_name: self.owner_name.as_deref(),
//             is_delete: None,
//             comment: self.comment.as_deref(),
//             other_basic,
//         }
//     }
// }

#[derive(Deserialize, Serialize)]
pub struct OwnerInfoResultDto {
    id: i32,
    room_number: String,
    owner_name: Option<String>,
    room_square: Option<String>,
    create_by: Option<String>,
    update_by: Option<String>,
    create_time: Option<NaiveDateTime>,
    update_time: Option<NaiveDateTime>,
    comment: Option<String>,
    other_basic: Option<serde_json::Value>,
}


