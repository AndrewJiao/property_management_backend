use crate::schema::basic::t_user_relate_room::*;
use crate::schema::basic::t_user_relate_room;
use common::data_result::AppResult;
use common::db_config::{db_get_connection, Conn};
use diesel::{ExpressionMethods, Insertable, QueryDsl, Queryable, RunQueryDsl, Selectable, SelectableHelper};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = t_user_relate_room)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(rename_all = "camelCase")]
pub struct UserRelateRoomPo {
    id: i64,
    relate_account_id: String,
    relate_number: String,
}
impl UserRelateRoomPo{
    pub fn bind(p_account_id: &str, p_room_numbers: &Vec<&str>, conn: &mut Conn) -> AppResult<Vec<UserRelateRoomPo>> {
        if p_room_numbers.is_empty(){
            return Ok(vec![]);
        }
        let new_insert:Vec<UserRelateRoomInsertPo> = p_room_numbers.iter()
            .map(|p_room_number| {
                UserRelateRoomInsertPo{
                    relate_account_id: p_account_id,
                    relate_number: p_room_number,
                }
            }).collect();
        let result = diesel::insert_into(table)
            .values(&new_insert)
            .get_results(conn)?;
        Ok(result)
    }
    pub fn unbind(any_id: &Vec<&str>, conn: &mut Conn) -> AppResult<usize> {
        //判空
        if any_id.is_empty(){
            return Ok(0);
        }
        let result = diesel::delete(table)
            .filter(relate_account_id.eq_any(any_id))
            .filter(relate_number.eq_any(any_id))
            .execute(conn)?;
        Ok(result)
    }

    pub fn by_account_id(p_account_id:Vec<&str>)->AppResult<Vec<UserRelateRoomPo>>{
        let result = table.select(UserRelateRoomPo::as_select())
            .filter(relate_account_id.eq_any(p_account_id))
            .get_results(&mut db_get_connection())?;
        Ok(result)
    }
    pub fn by_room_number(p_room_number: &str) -> AppResult<Vec<UserRelateRoomPo> > {
        let result = table.select(UserRelateRoomPo::as_select())
            .filter(relate_number.eq(p_room_number))
            .get_results(&mut db_get_connection())?;
        Ok(result)
    }

}
#[derive(Serialize, Insertable)]
#[diesel(table_name = t_user_relate_room)]
#[serde(rename_all = "camelCase")]
struct UserRelateRoomInsertPo<'a> {
    relate_account_id: &'a str,
    relate_number: &'a str,
}