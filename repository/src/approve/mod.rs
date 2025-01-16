use crate::component::page::Paginate;
use crate::schema::basic::t_approve;
use crate::schema::basic::t_approve::*;
use crate::{common_type, filter_data_enable, if_filter};
use common::data_result::AppResult;
use common::db_config::{db_get_connection, Conn};
use diesel::pg::Pg;
use diesel::{AsChangeset,  ExpressionMethods, Identifiable, Insertable, QueryDsl, Queryable, RunQueryDsl,  Selectable, SelectableHelper};
use diesel_derive_enum::DbEnum;
use management_macro::AutoOperation;
use serde::{Deserialize, Serialize};

common_type!();

#[derive(Deserialize,Serialize,Debug,DbEnum,Clone,Copy)]
#[ExistingTypePath = "crate::schema::basic::sql_types::ApproveState"]
#[serde(rename_all = "PascalCase")]
pub enum ApproveState{
    Pending,
    Approved,
    Rejected,
}
impl Default for ApproveState {
    fn default() -> Self {
        ApproveState::Pending
    }
}

impl ApproveState{
    pub fn to_string(&self) -> String {
        match self {
            ApproveState::Pending => "待审批".to_string(),
            ApproveState::Approved => "已通过".to_string(),
            ApproveState::Rejected => "已拒绝".to_string(),
        }
    }
}
#[derive(Deserialize,Serialize,Debug,DbEnum,Clone,Copy)]
#[ExistingTypePath = "crate::schema::basic::sql_types::ApproveType"]
#[serde(rename_all = "PascalCase")]
pub enum ApproveType{
    CreateUser,
    WeChartCreateUser,
    BindingRooms,
    ChangeRoomInfo,
}
impl ApproveType{
    pub fn to_string(&self) -> String {
        match self {
            ApproveType::CreateUser => "创建用户".to_string(),
            ApproveType::WeChartCreateUser => "微信小程序创建用户".to_string(),
            ApproveType::BindingRooms => "绑定房间".to_string(),
            ApproveType::ChangeRoomInfo => "修改房间信息".to_string(),
        }
    }
}
impl Default for ApproveType {
    fn default() -> Self {
        ApproveType::CreateUser
    }
}
#[derive(Queryable, Selectable, Deserialize, Serialize, Debug)]
#[diesel(table_name = t_approve)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(rename_all = "camelCase")]
pub struct ApprovePo {
    pub id: i64,
    pub order_no: String,
    pub approve_state: ApproveState,
    pub approve_type: ApproveType,
    pub approve_data: serde_json::Value,
    pub comment: Option<String>,
    pub create_by: String,
    pub update_by: String,
    pub create_time: chrono::NaiveDateTime,
    pub update_time: chrono::NaiveDateTime,
    pub is_delete: bool,
    pub account_id: String,
}
type BoxedQuery<'a> = t_approve::BoxedQuery<'a, Pg, crate::SqlType<ApprovePo>>;
impl ApprovePo{
    pub fn all<'a>() -> BoxedQuery<'a> {
        table.select(ApprovePo::as_select()).into_boxed()
    }

    pub fn by_id<'a>(p_id: i64) -> AppResult<ApprovePo> {
        let result = ApprovePo::all()
            .filter(id.eq(p_id))
            .filter(with_data_enable())
            .first(&mut db_get_connection())?;
        Ok(result)
    }
    pub fn by_order_no(p_order_no: &str) -> AppResult<ApprovePo> {
        let result = ApprovePo::all()
            .filter(order_no.eq(p_order_no))
            .filter(with_data_enable())
            .first(&mut db_get_connection())?;
        Ok(result)
    }

    pub fn by_search_param(
        (p_create_time_star, p_create_time_end): (Option<&chrono::NaiveDateTime>, Option<&chrono::NaiveDateTime>),
        p_approve_state: Option<&Vec<ApproveState>>,
        p_approve_type: Option<&ApproveType>,
        p_order_no: Option<&str>,
        p_account_id: Option<String>,
        (p_current_page, p_page_size): (i64, i64),
    ) -> AppResult<(Vec<ApprovePo>, i64)> {
        let mut  statement = Self::all();

        if_filter!(statement = with_create_time_between(p_create_time_star,p_create_time_end));
        if_filter!(statement = account_id.eq(p_account_id));
        if_filter!(statement = approve_state.eq_any(p_approve_state));
        if_filter!(statement = approve_type.eq(p_approve_type));
        if_filter!(statement = order_no.eq(p_order_no));
        filter_data_enable!(statement);
        let (data, total) = statement
            .order_by(create_time.desc())
            .paginate(p_current_page).per_page(p_page_size)
            .load_and_count_pages(&mut db_get_connection())?;
        Ok((data, total))
    }
}



#[derive(Serialize, Debug, Identifiable, AsChangeset, AutoOperation)]
#[diesel(table_name = t_approve)]
pub struct ApproveUpdatePo<'a> {
    pub id: i64,
    pub approve_state: Option<ApproveState>,
    pub comment: Option<&'a str>,
    pub update_by: Option<&'a str>,
    pub update_time: Option<chrono::NaiveDateTime>,
    pub is_delete: Option<bool>,
}


#[derive(Serialize, Debug, Insertable, AutoOperation)]
#[diesel(table_name = t_approve)]
pub struct ApproveInsertPo<'a> {
    pub order_no: String,
    pub approve_state: ApproveState,
    pub approve_type: ApproveType,
    pub approve_data: &'a serde_json::Value,
    pub comment: Option<&'a str>,
    pub create_by: &'a str,
    pub update_by: &'a str,
    pub create_time: Option<chrono::NaiveDateTime>,
    pub update_time: Option<chrono::NaiveDateTime>,
    pub is_delete: bool,
    pub account_id: &'a str,
}

impl ApproveInsertPo<'_>{
    pub fn save(&self, conn: &mut Conn) -> AppResult<ApprovePo> {
        let result = diesel::insert_into(table)
            .values(self)
            .get_result(conn)?;
        Ok(result)
    }

}


