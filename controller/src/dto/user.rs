use common::tools::serde::empty_string_or_null_as_none;
use common::tools::serde::empty_vec_or_null_as_none;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;
use common::const_value::SYSTEM;
use repository::user::{RoleType, UserInsertPo, UserPo, UserUpdatePo};
use crate::dto::{ToDesc, ToInsertPO, ToUpdatePO};

#[derive(Deserialize,Validate)]
#[serde(rename_all = "camelCase")]
pub struct UserCreateDto {
    #[validate(length(min = 1, max = 100))]
    pub account: String,
    #[validate(length(min = 1, max = 100))]
    pub password: String,
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    pub role_type: RoleType,
    #[validate(length(min = 1, max = 5000))]
    pub comment: Option<String>,
    #[validate(length(min = 1, max = 100))]
    pub binding_room_number: Option<Vec<String>>,
}
impl ToInsertPO for UserCreateDto{
    type PO<'a> = UserInsertPo<'a>;
    fn to_insert_po(&self) -> Self::PO<'_> {
        UserInsertPo {
            account_id: None,
            password: self.password.clone(),
            account: &self.account,
            name: &self.name,
            role_type: self.role_type,
            create_by: SYSTEM,
            update_by: SYSTEM,
            create_time: None,
            update_time: None,
            comment: self.comment.as_deref(),
            is_delete: false,
        }
    }
}


#[derive(Deserialize,Validate)]
#[serde(rename_all = "camelCase")]
pub struct UserUpdateDto {
    #[validate(length(min = 1, max = 100))]
    #[serde(default, deserialize_with = "empty_string_or_null_as_none")]
    pub name: Option<String>,
    pub role_type: RoleType,
    #[validate(length(min = 1, max = 5000))]
    #[serde(default, deserialize_with = "empty_string_or_null_as_none")]
    pub comment: Option<String>,
    #[validate(length(min = 1, max = 100))]
    #[serde(default)]
    pub binding_room_number: Option<Vec<String>>,
}
impl ToUpdatePO for UserUpdateDto{
    type PO<'a> = UserUpdatePo<'a>;

    fn to_update_po(&self, id: i64) -> Self::PO<'_> {
        UserUpdatePo {
            id,
            account: None,
            password: None,
            name: self.name.as_deref(),
            role_type: Some(self.role_type),
            update_by: None,
            update_time: None,
            comment: self.comment.as_deref(),
            is_delete: None,
        }
    }
}


#[derive(Deserialize, Validate, Default)]
#[serde(rename_all = "camelCase")]
pub struct UserSearchDto {
    #[validate(length(min = 1, max = 100))]
    #[serde(default, deserialize_with = "empty_string_or_null_as_none")]
    pub account: Option<String>,
    #[serde(default, deserialize_with = "empty_string_or_null_as_none")]
    #[validate(length(min = 1, max = 100))]
    pub name: Option<String>,
    pub role_type: Option<RoleType>,
    #[validate(length(min = 0, max = 100))]
    #[serde(default, deserialize_with = "empty_vec_or_null_as_none")]
    pub binding_room_number: Option<Vec<String>>,
    pub create_time_star: Option<NaiveDateTime>,
    pub create_time_end: Option<NaiveDateTime>,
    pub update_time_star: Option<NaiveDateTime>,
    pub update_time_end: Option<NaiveDateTime>,

}

#[derive(Serialize,Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserResultDto {
    pub account_id: String,
    pub account: String,
    pub password: String,
    pub name: String,
    pub role_type: RoleType,
    pub role_type_desc: String,
    pub create_by: String,
    pub update_by: String,
    pub create_time: chrono::NaiveDateTime,
    pub update_time: chrono::NaiveDateTime,
    pub comment: Option<String>,
    pub binding_room_number: Option<Vec<String>>,
}

impl From<UserPo> for UserResultDto{
    fn from(value: UserPo) -> Self {
        UserResultDto {
            account_id: value.account_id,
            account: value.account,
            password: value.password,
            name: value.name,
            role_type: value.role_type,
            role_type_desc: value.role_type.to_desc(),
            create_by: value.create_by,
            update_by: value.update_by,
            create_time: value.create_time,
            update_time: value.update_time,
            comment: value.comment,
            binding_room_number: None,
        }
    }
}
impl ToDesc for RoleType{
    fn to_desc(&self) -> String {
        match self {
            RoleType::Manager => "管理员".to_string(),
            RoleType::Root => "超级管理员".to_string(),
            RoleType::SubManager => "次级管理员".to_string(),
            RoleType::User => "普通用户".to_string(),
        }
    }
}


#[derive(Deserialize)]
#[serde(rename_all = "camelCase", tag = "searchType", content = "searchValue")]
pub enum SearchType {
    Account(String),
    Name(String),
    BindingRoomNumber(String),
}

