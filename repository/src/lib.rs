use diesel::dsl::{AsSelect, SqlTypeOf};
use diesel::pg::Pg;

pub mod attachment;
pub mod models;
pub mod schema;
pub mod price_basic;
pub mod owner_info;
pub mod component;
pub mod room_info;
pub mod property_fee;
pub mod owner_fee;
pub mod tool_table;
pub mod user;
pub mod approve;


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
    ($statement:ident = $column:ident.$method:ident($param:ident) ) => {
        if let Some(value) =  $param{
            $statement = $statement.filter($column.$method(value));
        }
    };
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


// #[macro_export]
// macro_rules! copy_statement {
//     ($statement:block)=>{
//         ({$statement}, {$statement});
//     }
// }
#[macro_export]
macro_rules! build_statement {
    ($a:ident,$b:ident,$($body:tt)*) => {{
        {
            $($body)*;
            $a  = statement
        }
        {
            $($body)*;
            $b = statement
        }
    }};
}


#[macro_export]
macro_rules! common_type {
    () => {

        #[allow(dead_code)]
        #[diesel::dsl::auto_type(no_type_alias)]
        pub fn with_create_time_between<'a>(begin: &'a chrono::NaiveDateTime, end: &'a chrono::NaiveDateTime) ->_
        {
            create_time.between(begin,end)
        }

        #[allow(dead_code)]
        #[diesel::dsl::auto_type(no_type_alias)]
        pub fn with_update_time_between<'a>(begin: &'a chrono::NaiveDateTime,end:&'a chrono::NaiveDateTime) ->_
        {
            update_time.between(begin, end)
        }

        #[allow(dead_code)]
        #[diesel::dsl::auto_type(no_type_alias)]
        pub fn with_data_enable<'a>()->_
        {
            is_delete.eq(false)
        }

        #[allow(dead_code)]
        #[diesel::dsl::auto_type(no_type_alias)]
        pub fn with_id_filter<'a>(param_id:i64)->_
        {
            id.eq(param_id)
        }

    }
}


// #[macro_export]
// macro_rules! with_conn_function {
//     ( pub fn $function_name_with_conn:ident | $function_name:ident( $($param:ident : $type_value:ty),*)->$return_value:ty
//         $inner:block
//     ) => {
//         pub fn $function_name_with_conn ($($param:$type_value),*,conn:&mut common::db_config::Conn) -> $return_value{
//             $inner
//         }
//
//         pub fn  $function_name($($param:$type_value),* ) -> $return_value{
//             Self::$function_name_with_conn($($param),*,&mut common::db_config::db_get_connection())
//         }
//     };
// }

