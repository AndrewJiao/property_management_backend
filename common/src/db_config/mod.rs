pub mod auto_trait;
pub mod type_convertor;

use crate::const_value::{DB_CONNECTION, SETTINGS};
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::PgConnection;
use log::info;

//region 封装连接池
pub fn establish_connection() -> Pool<AppConn> {
    let database_url = if SETTINGS.databases.database_url.is_empty() {
        let driven = "postgres://";
        let host = &SETTINGS.databases.host;
        let port = SETTINGS.databases.port;
        let user = &SETTINGS.databases.user;
        let password = &SETTINGS.databases.password;
        let database = &SETTINGS.databases.database;
        format!("{driven}{user}:{password}@{host}:{port}/{database}")
    } else {
        SETTINGS.databases.database_url.clone()
    };
    info!("Database URL: {database_url}");

    let manage = ConnectionManager::<PgConnection>::new(database_url);
    // let conn = LoggingConnection::new(manage);
    Pool::builder()
        .build(manage)
        .expect("Failed to create pool.")
}

pub type AppConn = ConnectionManager<PgConnection>;
pub type Conn = PooledConnection<AppConn>;
///
/// 考虑多线程环境下获取
///
pub fn db_get_connection<'a>() -> PooledConnection<AppConn> {
    DB_CONNECTION.get().expect("Failed to get db connection")
}
