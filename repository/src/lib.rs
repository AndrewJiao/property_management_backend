use diesel::dsl::{AsSelect, SqlTypeOf};
use diesel::pg::Pg;

pub mod models;
pub mod schema;
pub mod price_basic;
pub mod owner_info;
pub mod component;
pub mod room_info;
pub mod property_fee;
pub mod owner_fee;
pub mod tool_table;


type SqlType<T> = SqlTypeOf<AsSelect<T, Pg>>;

#[macro_export]
macro_rules! soft_delete_by_id {
    ($data_id:expr) => {
    diesel::update(table)
        .set((is_delete.eq(true), delete_at.eq(Some(common::tools::time::now_local_date_time_naive()))))
        .filter(id.eq($data_id))
        .execute(&mut common::db_config::db_get_connection())?;
    }
}

#[macro_export]
macro_rules! if_filter {
    ($statement:ident = $method:ident($param:ident) ) => {
        if let Some(value) =  $param{
            $statement = $statement.filter($method(value));
        }
    };

    ($statement:ident = $method:ident($param1:ident,$param2:ident) ) => {
        if let (Some(value1),Some(value2)) =  ($param1,$param2){
            $statement = $statement.filter($method(value1,value2));
        }
    };
}
#[macro_export]
macro_rules! filter_data_enable {
    ($statement:ident) => {
      $statement = $statement.filter(is_delete.eq(false))
    };
}


#[macro_export]
macro_rules! common_type {
    () => {

        diesel::define_sql_function!(fn canon_create_time_type(x:diesel::sql_types::Timestamp)->diesel::sql_types::Timestamp);

        #[allow(dead_code)]
        #[diesel::dsl::auto_type(no_type_alias)]
        pub fn with_create_time_between<'a>(begin: &'a chrono::NaiveDateTime, end: &'a chrono::NaiveDateTime) ->_
        {
            canon_create_time_type(create_time).between(begin,end)
        }

        diesel::define_sql_function!(fn canon_update_time_type(x:diesel::sql_types::Timestamp)->diesel::sql_types::Timestamp);
        #[allow(dead_code)]
        #[diesel::dsl::auto_type(no_type_alias)]
        pub fn with_update_time_between<'a>(begin: &'a chrono::NaiveDateTime,end:&'a chrono::NaiveDateTime) ->_
        {
            canon_update_time_type(update_time).between(begin, end)
        }

        diesel::define_sql_function!(fn canon_data_enable(x:diesel::sql_types::Bool)->diesel::sql_types::Bool);
        #[allow(dead_code)]
        #[diesel::dsl::auto_type(no_type_alias)]
        pub fn with_data_enable<'a>()->_
        {
            canon_data_enable(is_delete).eq(false)
        }

        diesel::define_sql_function!(fn canon_id_filter(x:diesel::sql_types::BigInt)->diesel::sql_types::BigInt);
        #[allow(dead_code)]
        #[diesel::dsl::auto_type(no_type_alias)]
        pub fn with_id_filter<'a>(param_id:i64)->_
        {
            canon_id_filter(id).eq(param_id)
        }

    }
}
