use crate::schema::basic::t_user_fast_login;
use crate::schema::basic::t_user_fast_login::*;
use common::data_result::AppResult;
use common::db_config::{db_get_connection, Conn};
use diesel::{ExpressionMethods, QueryDsl, Queryable, RunQueryDsl, Selectable, SelectableHelper};
use serde::{Deserialize, Serialize};
#[derive(Queryable, Selectable, Deserialize, Serialize, Clone)]
#[diesel(table_name = t_user_fast_login)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(rename_all = "camelCase")]
pub struct UserFastLoginPo {
    pub id: i64,
    pub account_id: String,
    pub create_time: chrono::NaiveDateTime,
}
impl UserFastLoginPo {
    pub fn add_user_fast_login(p_account_id: &str, conn: &mut Conn) -> AppResult<()> {
        if Self::is_fast_login(p_account_id) {
            return Ok(());
        }
        diesel::insert_into(table)
            .values(account_id.eq(p_account_id))
            .execute(conn)?;
        Ok(())
    }
    pub fn delete_user_fast_login(p_account_id: &str, conn: &mut Conn) -> AppResult<()> {
        diesel::delete(table)
            .filter(account_id.eq(p_account_id))
            .execute(conn)?;
        Ok(())
    }
    pub fn delete_all(conn: &mut Conn) -> AppResult<()> {
        diesel::delete(table).execute(conn)?;
        Ok(())
    }
    pub fn is_fast_login(p_account_id: &str) -> bool {
        let conn = &mut db_get_connection();
        table.select(UserFastLoginPo::as_select())
            .filter(account_id.eq(p_account_id))
            .get_results::<UserFastLoginPo>(conn).ok()
            .map(|e| !e.is_empty())
            .unwrap_or_default()
    }
}

