use crate::common_type;
use crate::schema::basic::t_owner_fee_detail_record::*;
use crate::schema::basic::t_owner_fee_detail_record;
use crate::tool_table::{current_date_count_with_conn, CountType};
use bigdecimal::BigDecimal;
use common::const_value::SETTINGS;
use common::db_config::auto_trait::AutoOperation;
use common::db_config::Conn;
use diesel::dsl::auto_type;
use diesel::pg::Pg;
use diesel::sql_types::Text;
use diesel::{define_sql_function, ExpressionMethods,  Insertable, QueryDsl, Queryable, RunQueryDsl, Selectable, SelectableHelper};
use management_macro::AutoOperation;
use serde::{Deserialize, Serialize};
use common::data_result::AppResult;

#[derive(Queryable, Selectable, Deserialize, Serialize, Debug)]
#[diesel(table_name = t_owner_fee_detail_record)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(rename_all = "camelCase")]
pub struct OwnerFeeDetailRecordPo {
    pub id: i64,
    pub record_id: String,
    pub room_number: String,
    pub count: i32,
    pub amount_balance: BigDecimal,
    pub comment: Option<String>,
    pub create_by: String,
    pub update_by: String,
    pub create_time: chrono::NaiveDateTime,
    pub update_time: chrono::NaiveDateTime,
    pub is_delete: bool,
}


common_type!();

define_sql_function!(fn canon_record_id(x:Text)->Text);
#[allow(dead_code)]
#[auto_type(no_type_alias)]
pub fn with_record_id<'a>(value: &'a str) -> _
{
    canon_record_id(record_id).eq(value)
}
define_sql_function!(fn canon_owner_fee_room_number(x:Text)->Text);
#[allow(dead_code)]
#[auto_type(no_type_alias)]
pub fn with_room_number<'a>(value: &'a str) -> _ {
    canon_owner_fee_room_number(room_number).eq(value)
}

type BoxedQuery<'a> = t_owner_fee_detail_record::BoxedQuery<'a, Pg, crate::SqlType<OwnerFeeDetailRecordPo>>;
impl OwnerFeeDetailRecordPo {
    fn all<'a>() -> BoxedQuery<'a> {
        table.select(OwnerFeeDetailRecordPo::as_select()).into_boxed()
    }

    pub fn by_room_number(param_room_number: &str) -> BoxedQuery {
        OwnerFeeDetailRecordPo::all()
            .filter(room_number.eq(param_room_number))
    }
    pub fn newest(param_room_number: &str) -> BoxedQuery {
        Self::by_room_number(param_room_number)
            .filter(is_delete.eq(false))
            .order_by(create_time.desc())
            .limit(1)
    }
    pub fn by_record_id_list(param_record_id_list: &Vec<&str>,conn:&mut Conn) -> AppResult<Vec<OwnerFeeDetailRecordPo>> {
        let result = OwnerFeeDetailRecordPo::all()
            .filter(record_id.eq_any(param_record_id_list))
            .filter(is_delete.eq(false))
            .get_results(conn)?;
        Ok(result)
    }
}

#[derive(Insertable, Serialize, Debug, AutoOperation)]
#[diesel(table_name = t_owner_fee_detail_record)]
pub struct OwnerFeeDetailRecordInsertPo<'a> {
    pub record_id: &'a str,
    pub room_number: &'a str,
    pub count: i32,
    pub amount_balance: &'a BigDecimal,
    pub comment: Option<&'a str>,
    pub create_by: &'a str,
    pub update_by: &'a str,
    pub create_time: Option<chrono::NaiveDateTime>,
    pub update_time: Option<chrono::NaiveDateTime>,
    pub is_delete: bool,

}

fn new_record_id(conn:&mut Conn) -> AppResult<String> {
    let result = current_date_count_with_conn(CountType::OwnerFeeRecordSeqNumber,conn)?;
    Ok(result)
}

///
/// 执行流水记录
/// 1.如果达到阈值，就新增新的流水记录
/// 2.如果没有达到，就更新最新的流水记录
///
///
pub fn try_record_data(
    p_amount_balance: &BigDecimal,
    p_room_number: &str,
    conn: &mut Conn) -> AppResult<OwnerFeeDetailRecordPo> {
    let newest_record = OwnerFeeDetailRecordPo::newest(p_room_number)
        .get_result(conn);
    let is_new = newest_record.is_err() || newest_record.as_ref().is_ok_and(|e| e.count >= SETTINGS.app_config.record_max);
    //判断阈值
    if is_new {
        //新增
        let new_record_id =new_record_id(conn)?;
        let new_po = OwnerFeeDetailRecordInsertPo {
            record_id: &new_record_id,
            room_number: p_room_number,
            count: 1,
            amount_balance: p_amount_balance,
            comment: None,
            create_by: "system",
            update_by: "system",
            create_time: None,
            update_time: None,
            is_delete: false,
        }.create_time();
        let result = diesel::insert_into(table)
            .values(vec!(&new_po))
            .get_result::<OwnerFeeDetailRecordPo>(conn)?;
        Ok(result)
    }else{
        let record = newest_record?;
        //修改 count+1 | amount_balance change
        let result = diesel::update(table)
            .filter(id.eq(record.id))
            .set((amount_balance.eq(p_amount_balance),count.eq(record.count + 1)))
            .get_result::<OwnerFeeDetailRecordPo>(conn)?;
        Ok(result)
    }
}


