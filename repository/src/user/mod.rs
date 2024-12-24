use crate::schema::basic::t_user;
use crate::schema::basic::t_user::*;
use crate::{common_type, filter_data_enable, if_filter};
use chrono::NaiveDateTime;
use common::data_result::AppResult;
use common::db_config::db_get_connection;
use diesel::dsl::auto_type;
use diesel::pg::Pg;
use diesel::ExpressionMethods;
use diesel::{AsChangeset, Identifiable, Insertable, QueryDsl, Queryable, RunQueryDsl, Selectable, SelectableHelper, TextExpressionMethods};
use diesel_derive_enum::DbEnum;
use management_macro::AutoOperation;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, DbEnum, Debug, Clone, Copy)]
#[ExistingTypePath = "crate::schema::basic::sql_types::RoleType"]
#[serde(rename_all = "PascalCase")]
pub enum RoleType {
    Root,
    Manager,
    SubManager,
    User,
}
common_type!();

#[auto_type(no_type_alias)]
pub fn with_name_like<'a>(value: &'a str) -> _ {
    let pattern: String = format!("%{}%", value);
    name.like(pattern)
}
#[auto_type(no_type_alias)]
pub fn with_account_like<'a>(value: &'a str) -> _ {
    let pattern: String = format!("%{}%", value);
    account.like(pattern)
}

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
    pub role_type: RoleType,
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

    pub fn find_by_account(p_account: &str) -> AppResult<Vec<UserPo>> {
        let result = Self::all()
            .filter(with_account_like(p_account))
            .filter(with_data_enable())
            .get_results(&mut db_get_connection())?;
        Ok(result)
    }
    pub fn find_by_name(p_name: &str) -> AppResult<Vec<UserPo>> {
        let result = Self::all()
            .filter(with_name_like(p_name))
            .filter(with_data_enable())
            .get_results(&mut db_get_connection())?;
        Ok(result)
    }


    pub fn search<'a>(p_account: Option<&'a str>,
                      p_bind_room: Option<&'a str>,
                      p_role: Option<RoleType>,
                      p_name: Option<&'a str>,
                      create_time_star: Option<&'a NaiveDateTime>,
                      create_time_end: Option<&'a NaiveDateTime>,
                      update_time_star: Option<&'a NaiveDateTime>,
                      update_time_end: Option<&'a NaiveDateTime>,

    )
        -> BoxedQuery<'a> {
        let mut statement = Self::all();

        if_filter!(statement = account.eq(p_account));
        if_filter!(statement = role_type.eq(p_role));
        if_filter!(statement = with_name_like(p_name));
        if_filter!(statement = binding_room_number.eq(p_bind_room));
        if_filter!(statement = with_create_time_between(create_time_star, create_time_end));
        if_filter!(statement = with_update_time_between(update_time_star, update_time_end));
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
    pub role_type: RoleType,
    pub create_by: &'a str,
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
    pub role_type: Option<RoleType>,
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
