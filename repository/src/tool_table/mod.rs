use common::data_result::AppResult;
use common::db_config::{db_get_connection, Conn};
use common::tools::time::{before_month_local_date, now_local_date};
use diesel::{QueryableByName, RunQueryDsl};

#[derive(Debug)]
pub enum CountType {
    OwnerFeeSeqNumber,
    OwnerFeeRecordSeqNumber,
    ApproveOrderNumber,
}

impl Into<String> for &CountType {
    fn into(self) -> String {
        match self {
            CountType::OwnerFeeSeqNumber => "OwnerFeeSeqNumber".to_string(),
            CountType::OwnerFeeRecordSeqNumber => "OwnerFeeRecordSeqNumber".to_string(),
            CountType::ApproveOrderNumber => "ApproveOrderNumber".to_string(),
        }
    }
}

#[derive(QueryableByName)]
struct ReturnValue {
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub value: String,
}


pub fn current_date_count_with_conn(count_type: CountType, conn: &mut Conn) -> AppResult<String> {
    let sql = r#"
        update basic.t_tool_table
        set value =
                case
                    when current_date = basic.t_tool_table."current_date"
                        then
                        cast(value as numeric) + 1
                    else
                        0
                    end,
            "current_date" = current_date

        where code = $1
        returning value;
    "#;
    let result =
        diesel::sql_query(sql)
            .bind::<diesel::sql_types::Text, String>((&count_type).into())
            .get_result::<ReturnValue>(conn)?;
    //保留五位，不足补0

    let order_number = match count_type {
        CountType::OwnerFeeSeqNumber => {
            format!("{}{}{:0>5}",
                    "HSMZ",
                    now_local_date("%Y%m%d"),
                    result.value)
        }
        CountType::OwnerFeeRecordSeqNumber => {
            format!("R-HSMZ{}{:0>5}",
                    now_local_date("%Y%m%d"),
                    result.value)
        }
        CountType::ApproveOrderNumber => {
            format!("SP-HSMZ{}{:0>5}",
                    now_local_date("%Y%m%d"),
                    result.value)
        }
    };
    Ok(order_number)
}

pub fn get_special_prefix() -> String {
    format!("{}{}",
            "HSMZ",
            before_month_local_date("%Y%m%d", 3))
}

pub fn current_date_count(count_type: CountType) -> AppResult<String> {
    current_date_count_with_conn(count_type, &mut db_get_connection())
}


