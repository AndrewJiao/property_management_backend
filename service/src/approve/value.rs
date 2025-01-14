use serde::Deserialize;
use common::const_value::SYSTEM;
use common::tools::password::generate_password;
use repository::user::{RoleType, UserInsertPo};

///
/// 定义一个创建用户的json
///
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserCreateValue {
    pub account: String,
    pub name: String,
    pub binding_room_number: Option<Vec<String>>,
}


impl UserCreateValue {
    pub fn to_insert_po(&self) -> UserInsertPo<'_> {
        UserInsertPo {
            account_id: None,
            account: &self.account,
            password: self.account.clone(),
            name: &self.name,
            role_type: RoleType::User,
            create_by: SYSTEM,
            update_by: SYSTEM,
            create_time: None,
            update_time: None,
            comment: None,
            is_delete: false,
            relate_user_id: None,
        }
    }
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WeChartUserCreateValue {
    pub nick_name: String,
    pub binding_room_number: Option<Vec<String>>,
}

impl WeChartUserCreateValue {
    pub fn to_insert_po(&self) -> UserInsertPo<'_> {
        UserInsertPo {
            account_id: None,
            account: &self.nick_name,
            password: generate_password(12),
            name: &self.nick_name,
            role_type: RoleType::User,
            create_by: SYSTEM,
            update_by: SYSTEM,
            create_time: None,
            update_time: None,
            comment: None,
            is_delete: false,
            relate_user_id: None,
        }
    }
}

