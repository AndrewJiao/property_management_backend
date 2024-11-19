pub mod auto_trait;
pub mod type_convertor;

use crate::const_value::SETTINGS;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::PgConnection;

//region 封装连接池
pub fn establish_connection() -> Pool<ConnectionManager<PgConnection>> {
    let database_url = &SETTINGS.databases.database_url;
    let manage = ConnectionManager::<PgConnection>::new(database_url);

    Pool::builder()
        .build(manage)
        .expect("Failed to create pool.")
}

pub fn establish_connection_str(database_url: &str) -> Pool<ConnectionManager<PgConnection>> {
    let manage = ConnectionManager::<PgConnection>::new(database_url);

    Pool::builder()
        .build(manage)
        .expect("Failed to create pool.")
}

pub fn db_get_connection() -> PooledConnection<ConnectionManager<PgConnection>> {
    let connection = &mut establish_connection();
    connection.get().expect("connection get error")
}
