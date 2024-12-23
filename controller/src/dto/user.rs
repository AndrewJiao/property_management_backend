use crate::dto::{ToInsertPO, ToUpdatePO};
use common::const_value::SYSTEM;
use repository::user::{RoleType, UserInsertPo, UserUpdatePo};
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize,Validate)]
#[serde(rename_all = "camelCase")]
pub struct UserCreateDto {
    #[validate(length(min = 1, max = 100))]
    pub account: String,
    #[validate(length(min = 1, max = 100))]
    pub password: String,
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    pub role: RoleType,
    #[validate(length(min = 1, max = 5000))]
    pub comment: Option<String>,
    #[validate(length(min = 1, max = 100))]
    pub binding_room_number: Option<String>,
}
impl ToInsertPO for UserCreateDto{
    type PO<'a> = UserInsertPo<'a>;
    fn to_insert_po(&self) -> Self::PO<'_> {
        UserInsertPo {
            account_id: None,
            password: self.password.clone(),
            account: &self.account,
            name: &self.name,
            role: self.role,
            update_by: SYSTEM,
            create_time: None,
            update_time: None,
            comment: self.comment.as_deref(),
            binding_room_number: self.binding_room_number.as_deref(),
            is_delete: false,
        }
    }
}


#[derive(Deserialize,Validate)]
#[serde(rename_all = "camelCase")]
pub struct UserUpdateDto {
    #[validate(length(min = 1, max = 100))]
    pub name: Option<String>,
    pub role: RoleType,
    #[validate(length(min = 1, max = 5000))]
    pub comment: Option<String>,
    #[validate(length(min = 1, max = 100))]
    pub binding_room_number: Option<String>,
}
impl ToUpdatePO for UserUpdateDto{
    type PO<'a> = UserUpdatePo<'a>;

    fn to_update_po(&self, id: i64) -> Self::PO<'_> {
        UserUpdatePo {
            id,
            account: None,
            password: None,
            name: self.name.as_deref(),
            role: Some(self.role),
            update_by: None,
            update_time: None,
            comment: self.comment.as_deref(),
            binding_room_number: self.binding_room_number.as_deref(),
            is_delete: None,
        }
    }
}


#[derive(Deserialize, Validate, Default)]
#[serde(rename_all = "camelCase")]
pub struct UserSearchDto {
    #[validate(length(min = 1, max = 100))]
    pub account: Option<String>,
    #[validate(length(min = 1, max = 100))]
    pub name: Option<String>,
    pub role: Option<RoleType>,
    #[validate(length(min = 1, max = 100))]
    pub binding_room_number: Option<String>,
}
