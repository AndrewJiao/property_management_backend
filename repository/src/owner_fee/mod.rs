mod record;
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
use diesel::{define_sql_function, AsChangeset, Expression, ExpressionMethods, Identifiable, Insertable, QueryDsl, Queryable, RunQueryDsl, Selectable, SelectableHelper};
use diesel_derive_enum::DbEnum;
use management_macro::AutoOperation;
pub use record::try_record_data;
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
}
common_type!();

define_sql_function!(fn canon_owner_fee_stream_id(x:Text)->Text);
#[auto_type(no_type_alias)]
pub fn with_stream_id<'a>(value: &'a str) -> _ {
    canon_owner_fee_stream_id(stream_id).eq(value)
}

define_sql_function!(fn canon_owner_fee_room_number(x:Text)->Text);
#[auto_type(no_type_alias)]
pub fn with_room_number<'a>(value: &'a str) -> _ {
    canon_owner_fee_room_number(room_number).eq(value)
}

type ColumnDetailType = crate::schema::basic::sql_types::DetailType;
define_sql_function!(fn canon_owner_fee_detail_type(x:ColumnDetailType)->ColumnDetailType);
#[auto_type(no_type_alias)]
pub fn with_detail_type<'a>(value: &'a DetailType) ->_
{
   canon_owner_fee_detail_type(detail_type).eq(value)
}
type BoxedQuery<'a> = t_owner_fee_detail::BoxedQuery<'a, Pg, crate::SqlType<OwnerFeeDetailPo>>;
impl OwnerFeeDetailPo {
    fn all<'a>() ->BoxedQuery<'a>{
        table.select(OwnerFeeDetailPo::as_select()).into_boxed()
    }
    pub fn search_by_param<'a>(
         param_stream_id: Option<&'a str>,
         param_room_number: Option<&'a str>,
         param_detail_type: Option<&'a DetailType>,
         param_create_time_star: Option<&'a chrono::NaiveDateTime>,
         param_create_time_end: Option<&'a chrono::NaiveDateTime>,
         param_update_time_star: Option<&'a chrono::NaiveDateTime>,
         param_update_time_end: Option<&'a chrono::NaiveDateTime>,
    ) -> BoxedQuery<'a>{
        let mut statement = OwnerFeeDetailPo::all();
        if_filter!(statement = with_stream_id(param_stream_id));
        if_filter!(statement = with_room_number(param_room_number));
        if_filter!(statement = with_detail_type(param_detail_type));
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
}

pub fn create_new_owner_fee_detail_stream<'a>(
     param_stream_id: &'a str,
     param_room_number: &'a str,
     param_owner_name: Option<&'a str>,
     param_detail_type: &'a DetailType,
     param_amount: &'a BigDecimal,
     param_record_id:&'a str,
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
        record_id:param_record_id
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


#[derive(Deserialize, Serialize, DbEnum, Debug, Clone)]
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

