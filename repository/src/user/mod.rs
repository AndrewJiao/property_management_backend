use crate::common_type;
use crate::component::page::Paginate;
use crate::schema::basic::t_user;
use crate::schema::basic::t_user::*;
use crate::user::relate::UserRelateRoomPo;
use actix_web::{HttpMessage, HttpRequest};
use chrono::NaiveDateTime;
use common::data_result::AppResult;
use common::db_config::{db_get_connection, AppConn, Conn};
use common::tools::jwt::{AccountInfo, JwtTokenInfoTrait};
use common::tools::time::DEFAULT_TIME;
use diesel::dsl::auto_type;
use diesel::pg::Pg;
use diesel::r2d2::PooledConnection;
use diesel::sql_types::Bool;
use diesel::{AsChangeset, BoolExpressionMethods, Identifiable, Insertable, QueryDsl, Queryable, RunQueryDsl, Selectable, SelectableHelper, TextExpressionMethods};
use diesel::{ExpressionMethods, JoinOnDsl};
use diesel_derive_enum::DbEnum;
use management_macro::AutoOperation;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use common::error::ACCOUNT_NOT_EXIST;
use crate::owner_info::OwnerBasicInfoPo;

pub mod relate;

#[derive(Deserialize, Serialize, DbEnum, Debug, Clone, Copy)]
#[ExistingTypePath = "crate::schema::basic::sql_types::RoleType"]
#[serde(rename_all = "PascalCase")]
pub enum RoleType {
    Root,
    Manager,
    SubManager,
    User,
}
impl Default for RoleType {
    fn default() -> Self {
        RoleType::User
    }
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

#[derive(Queryable, Selectable, Deserialize, Serialize, Clone)]
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
    pub relate_user_id: Option<String>,
}

impl UserPo {
    pub fn by_relate_user_id(code: String) -> AppResult<UserPo> {
        let result = table
            .filter(relate_user_id.eq(code))
            .filter(with_data_enable())
            .first(&mut db_get_connection())?;
        Ok(result)
    }
}

type BoxedQuery<'a> = t_user::BoxedQuery<'a, Pg, crate::SqlType<UserPo>>;
impl UserPo {
    pub fn all<'a>() -> BoxedQuery<'a> {
        table.select(UserPo::as_select()).into_boxed()
    }
    pub fn by_account(p_account: &str) -> AppResult<UserPo> {
        let result = Self::all()
            .filter(account.eq(p_account)).filter(is_delete.eq(false))
            .filter(with_data_enable())
            .first(&mut db_get_connection())?;
        Ok(result)
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

    ///
    /// 如果是管理员，就返回所有的roomNumber
    ///
    pub fn current_user_info(service_request: &HttpRequest) -> AppResult<(UserPo, Option<Vec<String>>)> {
        if let Some(account_info) = service_request.extensions().get::<AccountInfo>() {
            let is_user = account_info.role_type == "User";
            let result = Self::all()
                .filter(account_id.eq(&account_info.account_id))
                .filter(with_data_enable())
                .get_result(&mut db_get_connection())?;
            let relation_map = if is_user {
                UserRelateRoomPo::by_account_id(vec![&account_info.account_id]).ok()
                    .map(|item| item.into_iter().map(|item| item.relate_number).collect::<Vec<String>>())
            } else {
                Some(OwnerBasicInfoPo::all_result()?
                    .into_iter().map(|item| item.room_number).collect::<Vec<String>>())
            };

            Ok((result, relation_map))
        } else {
            Err(ACCOUNT_NOT_EXIST())
        }
    }


    pub fn search<'a>(
        p_account: Option<&'a str>,
        p_role: Option<&'a Vec<RoleType>>,
        p_name: Option<&'a str>,
        p_binding_room_number: Option<&Vec<String>>,
        create_time_star: Option<&'a NaiveDateTime>,
        create_time_end: Option<&'a NaiveDateTime>,
        update_time_star: Option<&'a NaiveDateTime>,
        update_time_end: Option<&'a NaiveDateTime>,
        current_page: i64,
        page_size: i64,
    )
        -> AppResult<(Vec<(UserPo, Option<Vec<String>>)>, i64)> {
        use crate::schema::basic::t_user_relate_room;
        let (result, total)  = table
            .left_join(t_user_relate_room::table.on(t_user_relate_room::relate_account_id.eq(t_user::account_id)))
            .filter(diesel::dsl::sql::<Bool>(if p_account.is_none() { "TRUE" } else { "FALSE" }).or(with_account_like(p_account.unwrap_or_default())))
            .filter(diesel::dsl::sql::<Bool>(if p_name.is_none() { "TRUE" } else { "FALSE" }).or(with_name_like(p_name.unwrap_or_default())))
            .filter(diesel::dsl::sql::<Bool>(if p_binding_room_number.is_none() { "TRUE" } else { "FALSE" }).or(t_user_relate_room::relate_number.eq_any(p_binding_room_number.unwrap_or(&vec!["a".to_string()]))))
            .filter(diesel::dsl::sql::<Bool>(if create_time_star.is_none() { "TRUE" } else { "FALSE" }).or(create_time.between(create_time_star.unwrap_or(&DEFAULT_TIME), create_time_end.unwrap_or(&DEFAULT_TIME))))
            .filter(diesel::dsl::sql::<Bool>(if update_time_star.is_none() { "TRUE" } else { "FALSE" }).or(update_time.between(update_time_star.unwrap_or(&DEFAULT_TIME), update_time_end.unwrap_or(&DEFAULT_TIME))))
            .filter(diesel::dsl::sql::<Bool>(if p_role.is_none() { "TRUE" } else { "FALSE" }).or(role_type.eq_any(p_role.unwrap_or(&vec![RoleType::User]))))
            .filter(with_data_enable())
            .select(UserPo::as_select())
            .group_by( (id, account_id, account, password, name, role_type, create_by, update_by, create_time, update_time, comment, is_delete))
            .order_by(create_time.desc())
            .paginate(current_page)
            .per_page(page_size)
            .load_and_count_pages(&mut db_get_connection())?;

        let acc_ids = result.iter().map(|item| {
            item.account_id.as_str()
        }).collect::<Vec<&str>>();
        let mut relation_map =
            UserRelateRoomPo::by_account_id(acc_ids)?
                .into_iter().map(|item| (item.relate_account_id, item.relate_number))
                .fold(HashMap::new(), |mut acc, (key, value)| {
                    acc.entry(key).or_insert(vec![]).push(value);
                    acc
                });
        let result = result.into_iter().map(|item| {
            let acc_temp_id = item.account_id.clone();
            (item, relation_map.remove(&acc_temp_id))
        }).collect();
        Ok((result, total))
    }
}


///
/// 用于生成jwttoken
///
impl JwtTokenInfoTrait for UserPo{
    fn create_info(self) -> AccountInfo {
        AccountInfo {
            account: self.account,
            name: self.name,
            account_id: self.account_id,
            role_type: format!("{:?}", self.role_type),
        }
    }
}
impl PartialEq<Self> for UserPo {
    fn eq(&self, other: &Self) -> bool {
        self.account_id == other.account_id
    }
}
impl Hash for UserPo{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.account_id.hash(state);
    }
}
impl Eq for UserPo {}

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
    pub is_delete: bool,
    pub relate_user_id: Option<String>,
}
impl UserInsertPo<'_> {
    pub fn save(self,conn:&mut Conn) ->AppResult<UserPo>{
        let result = diesel::insert_into(t_user::table)
            .values(self)
            .get_result(conn)?;
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
    pub is_delete: Option<bool>,
}

impl<'a> UserUpdatePo<'a> {
    pub fn change_password(p_account: &String, new_password: String, conn: &mut PooledConnection<AppConn>) -> AppResult<UserPo> {
        let result = diesel::update(table)
            .filter(account.eq(p_account))
            .set(password.eq(new_password))
            .get_result(conn)?;
        Ok(result)


    }
}

pub fn delete_by_id(p_id: i64, conn: &mut Conn) -> AppResult<UserPo> {
    let result = diesel::update(t_user::table)
        .filter(with_id_filter(p_id))
        .set(is_delete.eq(true))
        .get_result(conn)?;
    Ok(result)
}
