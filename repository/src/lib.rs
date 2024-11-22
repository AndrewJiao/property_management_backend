pub mod models;
pub mod schema;
pub mod price_basic;
pub mod owner_info;
pub mod component;
pub mod room_info;
pub mod property_fee;



#[macro_export]
macro_rules! soft_delete_by_id {
    ($data_id:expr) => {
        diesel::update(table)
            .set(is_delete.eq(true))
            .filter(id.eq( $data_id ))
            .execute(&mut common::db_config::db_get_connection())?;
    }
}
