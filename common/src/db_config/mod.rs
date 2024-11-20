pub mod auto_trait;
pub mod type_convertor;

use crate::const_value::{DB_CONNECTION, SETTINGS};
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


///
/// 考虑多线程环境下获取
///
pub fn db_get_connection<'a>() -> PooledConnection<ConnectionManager<PgConnection>> {
    DB_CONNECTION.get().expect("Failed to get db connection")
}
