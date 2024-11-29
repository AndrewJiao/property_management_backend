use common::data_result::AppResult;
use common::db_config::db_get_connection;
use diesel::{QueryableByName, RunQueryDsl};
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum CountType {
    OWNER_FEE_SEQ_NUMBER
}

impl Display for CountType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CountType::OWNER_FEE_SEQ_NUMBER => write!(f, "OWNER_FEE_SEQ_NUMBER"),
        }
    }
}

#[derive(QueryableByName)]
struct ReturnValue {
    // pub id: i64,
    // pub code: String,
    #[diesel(sql_type = diesel::sql_types::Text)]
    // #[diesel(deserialize_as = LowercaseString)]
    pub value: String,
    // pub comment: String,
}


pub fn current_date_count(count_type: CountType) -> AppResult<String> {
    let sql = " update basic.t_tool_table set value = cast(value as numeric) + 1 where code = $1 returning value into current; ";
    let result = diesel::sql_query(sql)
        .bind::<diesel::sql_types::Text, _>(count_type.to_string())
        .get_result::<ReturnValue>(&mut db_get_connection())?;

    //生成00001，00002...00009，00100，00101...00999，01000，01001...09999，10000，10001...99999
    Ok(format!("{:05}", result.value))
}


