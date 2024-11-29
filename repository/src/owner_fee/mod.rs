use crate::schema::basic::t_owner_fee_detail;
use crate::tool_table::{current_date_count, CountType};
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use common::tools::time::now_local_date;
use diesel::backend::Backend;
use diesel::deserialize::FromSql;
use diesel::pg::Pg;
use diesel::serialize::{Output, ToSql};
use diesel::sql_types::Text;
use diesel::{AsChangeset, Expression, Identifiable, Insertable, Queryable, Selectable};
use diesel_derive_enum::DbEnum;
use management_macro::AutoOperation;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Deserialize, Debug)]
#[diesel(table_name = t_owner_fee_detail)]
pub struct OwnerFeeDetail {
    pub id: i64,
    pub stream_id: StreamId,
    pub room_number: String,
    pub owner_name: Option<String>,
    pub detail_type: DetailType,
    pub amount: BigDecimal,
    pub comment: Option<String>,
    pub create_by: String,
    pub update_by: String,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
    pub is_delete: bool,
}

#[derive(Serialize, Debug, Insertable, AutoOperation)]
#[diesel(table_name = t_owner_fee_detail)]
pub struct OwnerFeeDetailInsertPo<'a> {
    pub id: i64,
    pub stream_id: &'a StreamId,
    pub room_number: &'a str,
    pub owner_name: Option<&'a str>,
    pub detail_type: &'a DetailType,
    pub amount: &'a BigDecimal,
    pub comment: Option<&'a str>,
    pub create_by: &'a str,
    pub update_by: &'a str,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
}

#[derive(Serialize, Debug, Identifiable, AsChangeset, AutoOperation)]
#[diesel(table_name = t_owner_fee_detail)]
pub struct OwnerFeeDetailUpdatePo<'a> {
    pub id: i64,
    pub amount: Option<&'a BigDecimal>,
    pub comment: Option<&'a str>,
    pub update_by: Option<&'a str>,
    pub update_time: Option<NaiveDateTime>,
    pub is_delete: Option<&'a bool>,
}


#[derive(Deserialize, Serialize, DbEnum, Debug)]
#[ExistingTypePath = "crate::schema::basic::sql_types::DetailType"]
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
#[derive(Debug, Deserialize, Serialize)]
pub struct StreamId {
    content: String,
}
impl StreamId {}

impl Default for StreamId {
    fn default() -> Self {
        let content = format!("{}{}{}",
                              STREAM_ID_PREFIX,
                              now_local_date("%Y%m%d"),
                              current_date_count(CountType::OWNER_FEE_SEQ_NUMBER).unwrap_or("00001".to_string()));
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

impl Expression for StreamId {
    type SqlType = Text;
}

