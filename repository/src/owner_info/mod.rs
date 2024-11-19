use crate::schema::basic::t_owner_basic_info;
use chrono::NaiveDateTime;
use diesel::backend::Backend;
use diesel::deserialize::FromSql;
use diesel::pg::Pg;
use diesel::serialize::{Output, ToSql};
use diesel::{AppearsOnTable, AsChangeset, Identifiable, Queryable, Selectable};
use diesel::Expression;
use management_macro::AutoOperation;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Debug;
use std::io::Write;

#[derive(Selectable, Queryable, Deserialize, Serialize)]
#[diesel(table_name = t_owner_basic_info)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct OwnerBasicInfoPo {
    id: i32,
    room_number: String,
    owner_name: Option<String>,
    room_square: Option<String>,
    create_by: Option<String>,
    update_by: Option<String>,
    create_time: Option<NaiveDateTime>,
    update_time: Option<NaiveDateTime>,
    is_delete: bool,
    comment: Option<String>,
    other_basic: Option<AppJson>,
}


#[derive(Identifiable, AsChangeset, Deserialize, Serialize, AutoOperation)]
#[diesel(table_name = t_owner_basic_info)]
pub struct UpdateOwnerBasicInfoPo<'a> {
    pub id: i32,
    pub room_number: &'a str,
    pub owner_name: Option<&'a str>,
    pub is_delete: Option<bool>,
    pub comment: Option<&'a str>,
    pub other_basic: Option<AppJson>,
}


#[derive(Deserialize, Serialize, Debug)]
pub struct AppJson(pub serde_json::Value);

impl AppJson {
    pub fn value(&self) -> &Value {
        &self.0
    }

    pub fn take_value(self) -> Value {
        self.0
    }
}

impl FromSql<diesel::sql_types::Json, Pg> for AppJson {
    fn from_sql(bytes: <Pg as Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        let str = std::str::from_utf8(bytes.as_bytes())?;
        match serde_json::from_str(str) {
            Ok(serde_json) => Ok(AppJson(serde_json)),
            Err(e) => Err(e.into())
        }
    }
}

impl ToSql<diesel::sql_types::Json, Pg> for AppJson {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
        let json_str = serde_json::to_string(&self.0)?;
        out.write_all(json_str.as_bytes())?;
        Ok(diesel::serialize::IsNull::No)
    }
}


impl From<serde_json::Value> for AppJson {
    fn from(value: Value) -> Self {
        AppJson(value)
    }
}


impl Expression for AppJson { type SqlType = diesel::sql_types::Nullable<diesel::sql_types::Json>; }
impl AppearsOnTable<t_owner_basic_info::table> for AppJson {}
