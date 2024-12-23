use crate::schema::basic::t_user;
use crate::schema::basic::t_user::*;
use crate::{common_type, filter_data_enable, if_filter};
use common::data_result::AppResult;
use common::db_config::db_get_connection;
use diesel::pg::Pg;
use diesel::{AsChangeset, Identifiable, Insertable, QueryDsl, Queryable, RunQueryDsl, Selectable, SelectableHelper};
use diesel::{ExpressionMethods};
use diesel_derive_enum::DbEnum;
use management_macro::AutoOperation;
use serde::{Deserialize, Serialize};

#[derive(Debug, DbEnum,Deserialize,Serialize,Copy,Clone)]
#[ExistingTypePath = "crate::schema::basic::sql_types::RoleType"]
pub enum RoleType {
    Manager,
    SubManager,
    User,
}
common_type!();

#[derive(Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = t_user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(rename_all = "camelCase")]
pub struct UserPo {
    pub id: i64,
    pub account_id: String,
    pub account: String,
    pub password: String,
    pub name: String,
    pub role: RoleType,
    pub create_by: String,
    pub update_by: String,
    pub create_time: chrono::NaiveDateTime,
    pub update_time: chrono::NaiveDateTime,
    pub comment: Option<String>,
    pub is_delete: bool,
    pub binding_room_number: Option<String>,
}
type BoxedQuery<'a> = t_user::BoxedQuery<'a, Pg, crate::SqlType<UserPo>>;
impl UserPo {
    pub fn all<'a>() -> BoxedQuery<'a> {
        table.select(UserPo::as_select()).into_boxed()
    }
    pub fn by_account(p_account: &str) -> Option<UserPo> {
        let result = Self::all()
            .filter(account.eq(p_account)).filter(is_delete.eq(false))
            .filter(with_data_enable())
            .first(&mut db_get_connection())
            .ok();
        result
    }

    pub fn search<'a>(p_account: std::option::Option<&'a str>, p_bind_room: std::option::Option<&'a str>, p_role: Option<RoleType>, p_name: Option<&'a str>)
        -> BoxedQuery<'a> {
        let mut statement = Self::all();
        if_filter!(statement = account.eq(p_account));
        if_filter!(statement = role.eq(p_role));
        if_filter!(statement = name.eq(p_name));
        if_filter!(statement = binding_room_number.eq(p_bind_room));
        filter_data_enable!(statement);
        statement
    }
}

#[derive(Insertable, Deserialize, Serialize, AutoOperation)]
#[diesel(table_name = t_user)]
#[serde(rename_all = "camelCase")]
pub struct UserInsertPo<'a> {
    pub account_id: Option<String>,
    pub account: &'a str,
    pub password: String,
    pub name: &'a str,
    pub role: RoleType,
    pub update_by: &'a str,
    pub create_time: Option<chrono::NaiveDateTime>,
    pub update_time: Option<chrono::NaiveDateTime>,
    pub comment: Option<&'a str>,
    pub binding_room_number: Option<&'a str>,
    pub is_delete: bool,
}
impl UserInsertPo<'_> {
    pub fn save(self) ->AppResult<UserPo>{
         let result = diesel::insert_into(t_user::table)
            .values(self)
            .get_result(&mut db_get_connection())?;
        Ok(result)
    }
}

#[derive(Identifiable, AsChangeset, Deserialize, Serialize, AutoOperation)]
#[diesel(table_name = t_user)]
#[serde(rename_all = "camelCase")]
pub struct UserUpdatePo<'a> {
    pub id: i64,
    pub account: Option<&'a str>,
    pub password: Option<&'a str>,
    pub name: Option<&'a str>,
    pub role: Option<RoleType>,
    pub update_by: Option<&'a str>,
    pub update_time: Option<chrono::NaiveDateTime>,
    pub comment: Option<&'a str>,
    pub binding_room_number: Option<&'a str>,
    pub is_delete: Option<bool>,
}

pub fn delete_by_id(p_id: i64) -> AppResult<()> {
    diesel::update(t_user::table)
        .filter(with_id_filter(p_id))
        .set(is_delete.eq(true))
        .execute(&mut db_get_connection())?;
    Ok(())
}
