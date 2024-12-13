mod record;

pub use record::try_record_data;
pub use record::OwnerFeeDetailRecordPo;
use std::cmp::Ordering;

use crate::schema::basic::t_owner_fee_detail;
use crate::schema::basic::t_owner_fee_detail::*;
use crate::tool_table::{current_date_count, CountType};
use crate::{common_type, filter_data_enable, if_filter};
use bigdecimal::BigDecimal;
use common::data_result::AppResult;
use common::db_config::auto_trait::AutoOperation;
use common::db_config::{db_get_connection, Conn};
use common::tools::time::now_local_date;
use diesel::backend::Backend;
use diesel::deserialize::{FromSql, FromSqlRow};
use diesel::dsl::auto_type;
use diesel::pg::Pg;
use diesel::serialize::{Output, ToSql};
use diesel::sql_types::Text;
use diesel::{AsChangeset, BoolExpressionMethods, Expression, ExpressionMethods, Identifiable, Insertable, QueryDsl, Queryable, RunQueryDsl, Selectable, SelectableHelper, TextExpressionMethods};
use diesel_derive_enum::DbEnum;
use management_macro::AutoOperation;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Deserialize, Serialize, Debug)]
#[diesel(table_name = t_owner_fee_detail)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(rename_all = "camelCase")]
pub struct OwnerFeeDetailPo {
    pub id: i64,
    pub stream_id: String,
    pub room_number: String,
    pub owner_name: Option<String>,
    pub detail_type: DetailType,
    pub amount: BigDecimal,
    pub comment: Option<String>,
    pub create_by: String,
    pub update_by: String,
    pub create_time: chrono::NaiveDateTime,
    pub update_time: chrono::NaiveDateTime,
    pub is_delete: bool,
    pub record_id: String,
    pub related_order_number: String,
}
common_type!();

#[auto_type(no_type_alias)]
pub fn with_stream_id<'a>(value: &'a str) -> _ {
    stream_id.eq(value)
}

#[auto_type(no_type_alias)]
pub fn with_room_number_like<'a>(value: &'a str) -> _ {
    let pattern:String = format!("{}%", value);
    room_number.like(pattern)
}

#[auto_type(no_type_alias)]
pub fn with_detail_type_in<'a>(value: &'a Vec<DetailType>) ->_
{
   detail_type.eq_any(value)
}
type BoxedQuery<'a> = t_owner_fee_detail::BoxedQuery<'a, Pg, crate::SqlType<OwnerFeeDetailPo>>;
impl OwnerFeeDetailPo {
    fn all<'a>() ->BoxedQuery<'a>{
        table.select(OwnerFeeDetailPo::as_select()).into_boxed()
    }
    pub fn search_by_param<'a>(
         param_stream_id: Option<&'a str>,
         param_room_number: Option<&'a str>,
         param_detail_type: Option<&'a Vec<DetailType>>,
         param_create_time_star: Option<&'a chrono::NaiveDateTime>,
         param_create_time_end: Option<&'a chrono::NaiveDateTime>,
         param_update_time_star: Option<&'a chrono::NaiveDateTime>,
         param_update_time_end: Option<&'a chrono::NaiveDateTime>,
    ) -> BoxedQuery<'a>{
        let mut statement = OwnerFeeDetailPo::all();
        if_filter!(statement = with_stream_id(param_stream_id));
        if_filter!(statement = with_room_number_like(param_room_number));
        if_filter!(statement = with_detail_type_in(param_detail_type));
        if_filter!(statement = with_create_time_between(param_create_time_star,param_create_time_end));
        if_filter!(statement = with_update_time_between(param_update_time_star,param_update_time_end));
        filter_data_enable!(statement);
        statement
    }

    pub fn get_by_id(param_id:i64) -> AppResult<OwnerFeeDetailPo> {
        let result = OwnerFeeDetailPo::all()
            .filter(id.eq(param_id))
            .first(&mut db_get_connection())?;
        Ok(result)
    }

    pub fn get_by_stream_record_id_list(param_stream_record_id_list:&Vec<&str>, conn:&mut Conn) -> AppResult<Vec<OwnerFeeDetailPo>> {
        let result = OwnerFeeDetailPo::all()
            .filter(record_id.eq_any(param_stream_record_id_list))
            .filter(is_delete.eq(false))
            .get_results(conn)?;
        Ok(result)
    }

    pub fn by_room_number_and_relative_order_numbers(params:&Vec<(&str,&str)>, conn:&mut Conn) -> AppResult<Vec<OwnerFeeDetailPo>> {
        let mut boxed_query= OwnerFeeDetailPo::all();
        boxed_query = params.iter().fold(boxed_query,|query,(p_room_number,p_related_order_number,)|{
            query.or_filter(room_number.eq(p_room_number).and(related_order_number.eq(p_related_order_number)))
        });
        let result = boxed_query.get_results(conn)?;
        Ok(result)
    }
}

impl Eq for OwnerFeeDetailPo {}

impl PartialEq<Self> for OwnerFeeDetailPo {
    fn eq(&self, other: &Self) -> bool {
        self.stream_id == other.stream_id
    }
}

impl PartialOrd<Self> for OwnerFeeDetailPo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other).into())
    }
}

impl Ord for OwnerFeeDetailPo {
    fn cmp(&self, other: &Self) -> Ordering {
        self.stream_id.cmp(&other.stream_id)
    }
}

#[derive(Serialize, Debug, Insertable, AutoOperation)]
#[diesel(table_name = t_owner_fee_detail)]
pub struct OwnerFeeDetailInsertPo<'a> {
    pub stream_id: &'a str,
    pub room_number: &'a str,
    pub owner_name: Option<&'a str>,
    pub detail_type: &'a DetailType,
    pub amount: &'a BigDecimal,
    pub comment: Option<&'a str>,
    pub create_by: &'a str,
    pub update_by: &'a str,
    pub create_time: Option<chrono::NaiveDateTime>,
    pub update_time: Option<chrono::NaiveDateTime>,
    pub record_id:&'a str,
    pub related_order_number:&'a str,
}

pub fn create_new_owner_fee_detail_stream<'a>(
     param_stream_id: &'a str,
     param_room_number: &'a str,
     param_owner_name: Option<&'a str>,
     param_detail_type: &'a DetailType,
     param_amount: &'a BigDecimal,
     param_record_id:&'a str,
     param_relative_order_number:&'a str,
     conn :&mut Conn
)->AppResult<OwnerFeeDetailPo>{
    let po = OwnerFeeDetailInsertPo {
        stream_id:param_stream_id,
        room_number:param_room_number,
        owner_name:param_owner_name,
        detail_type:param_detail_type,
        amount:param_amount,
        comment: None,
        create_by: "System",
        update_by: "System",
        create_time: None,
        update_time: None,
        record_id:param_record_id,
        related_order_number:param_relative_order_number,
    }.create_time();
    let result = diesel::insert_into(table).values(po).get_result::<OwnerFeeDetailPo>(conn)?;
    Ok(result)
}


#[derive(Serialize, Debug, Identifiable, AsChangeset, AutoOperation)]
#[diesel(table_name = t_owner_fee_detail)]
pub struct OwnerFeeDetailUpdatePo<'a> {
    pub id: i64,
    pub amount: Option<&'a BigDecimal>,
    pub comment: Option<&'a str>,
    pub update_by: Option<&'a str>,
    pub update_time: Option<chrono::NaiveDateTime>,
    pub is_delete: Option<&'a bool>,
}


#[derive(Deserialize, Serialize, DbEnum, Debug, Clone,PartialEq,Eq)]
#[ExistingTypePath = "crate::schema::basic::sql_types::DetailType"]
#[serde(rename_all = "PascalCase")]
pub enum DetailType {
    //物业费
    ManagementFee,
    //滞纳
    LiquidatedDamages,
    //预存
    PreStoreFee,
    //结算
    SettlementFee,
}



const STREAM_ID_PREFIX: &'static str = "HSMZ";
#[derive(Debug, Deserialize, Serialize, FromSqlRow)]
pub struct StreamId {
    pub content: String,
}
impl StreamId {}

impl Default for StreamId {
    fn default() -> Self {
        let content = format!("{}{}{}",
                              STREAM_ID_PREFIX,
                              now_local_date("%Y%m%d"),
                              current_date_count(CountType::OwnerFeeSeqNumber).unwrap_or("00001".to_string()));
        StreamId { content }
    }
}
impl From<&str> for StreamId {
    fn from(content: &str) -> Self {
        StreamId { content: content.to_string() }
    }
}

impl FromSql<Text, Pg> for StreamId {
    fn from_sql(bytes: <Pg as Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        let str = std::str::from_utf8(bytes.as_bytes())?;
        Ok(str.into())
    }
}

impl ToSql<Text, Pg> for StreamId {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
        ToSql::<Text, Pg>::to_sql(&self.content, out)
    }
}

// impl FromSqlRow<Text, Pg> for StreamId {
//     fn build_from_row<'a>(row: &impl Row<'a, Pg, Field<'a> = Text>) -> diesel::deserialize::Result<Self> {
//         row.get(1)?
//         Ok(stream_id_str.into())
//     }
// }

impl Expression for StreamId {
    type SqlType = Text;
}

