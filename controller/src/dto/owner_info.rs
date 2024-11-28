use crate::dto::{ToInsertPO, ToUpdatePO};
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use common::tools::serde::empty_string_or_null_as_none;

use common::tools::time::now_local_date_time_naive;
use repository::owner_info::{InsertOwnerBasicInfoPo, UpdateOwnerBasicInfoPo};
use serde::{Deserialize, Serialize};
use validator::Validate;

///
/// 作为业主表分页查询条件
///
///
#[derive(Deserialize, Serialize, Validate, Default, Debug)]
#[serde(rename_all = "camelCase", default)]
// #[serde(default)] // 缺失字段时自动使用默认值
pub struct OwnerInfoSearchDto {
    #[validate(length(min = 0, max = 100))]
    #[serde(deserialize_with = "empty_string_or_null_as_none")]
    pub room_number: Option<String>,
    #[validate(length(min = 0, max = 100))]
    #[serde(deserialize_with = "empty_string_or_null_as_none")]
    pub owner_name: Option<String>,
}


#[derive(Deserialize, Serialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OwnerInfoUpdateDto {
    #[validate(length(min = 0, max = 100))]
    #[serde(deserialize_with = "empty_string_or_null_as_none")]
    pub room_number: Option<String>,
    #[validate(length(min = 0, max = 100))]
    #[serde(deserialize_with = "empty_string_or_null_as_none")]
    pub owner_name: Option<String>,
    #[validate(length(min = 0, max = 100))]
    #[serde(deserialize_with = "empty_string_or_null_as_none", default)]
    pub comment: Option<String>,
    #[validate(custom(function = "common::tools::validator::validate_big_decimal"))]
    #[serde(default)]
    pub room_square: Option<BigDecimal>,
    // #[serde(deserialize_with = "json_verify")]
    pub other_basic: Option<serde_json::Value>,
}


impl ToUpdatePO for OwnerInfoUpdateDto {
    type PO<'a> = UpdateOwnerBasicInfoPo<'a>;

    fn to_update_po(&self, id: i64) -> Self::PO<'_> {
        UpdateOwnerBasicInfoPo {
            id:id as i32,
            room_number: self.room_number.as_deref(),
            owner_name: self.owner_name.as_deref(),
            is_delete: None,
            comment: self.comment.as_deref(),
            room_square: self.room_square.as_ref(),
            other_basic: self.other_basic.as_ref(),
            update_time: None,
            delete_at: None,
        }
    }
}

#[derive(Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct OwnerInfoInsertDto {
    #[validate(length(min = 0, max = 100))]
    #[serde(deserialize_with = "empty_string_or_null_as_none")]
    pub room_number: Option<String>,
    #[validate(length(min = 0, max = 100))]
    #[serde(deserialize_with = "empty_string_or_null_as_none")]
    pub owner_name: Option<String>,
    #[validate(custom(function = "common::tools::validator::validate_big_decimal"))]
    #[serde(default )]
    pub room_square: Option<BigDecimal>,
    #[validate(length(min = 0, max = 100))]
    #[serde(deserialize_with = "empty_string_or_null_as_none", default )]
    pub comment: Option<String>,
    // #[serde(deserialize_with = "json_verify")]
    pub other_basic: Option<serde_json::Value>,
}

impl ToInsertPO for OwnerInfoInsertDto {
    type PO<'a> = InsertOwnerBasicInfoPo<'a>;
    fn to_insert_po(&self) -> Self::PO<'_> {
        let now = now_local_date_time_naive();
        InsertOwnerBasicInfoPo {
            room_number: self.room_number.as_deref(),
            owner_name: self.owner_name.as_deref(),
            room_square: self.room_square.as_ref(),
            create_by: Some("System"),
            update_by: Some("System"),
            create_time: Some(now),
            update_time: Some(now),
            is_delete: false,
            comment: self.comment.as_deref(),
            other_basic: self.other_basic.clone(),
            delete_at: Some(NaiveDateTime::UNIX_EPOCH)
        }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
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

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", tag = "searchType", content = "searchValue")]
pub enum OwnerInfoSearchType {
    RoomNumber(String),
    OwnerName(String)
}

