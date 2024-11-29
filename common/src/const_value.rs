use crate::conf::config_module::Settings;
use crate::db_config::{establish_connection, AppConn};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use diesel_logger::LoggingConnection;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref SETTINGS: Settings = Settings::new().expect("Failed to load settings");
}
// 不能直接拿来用
lazy_static!(
    pub static ref DB_CONNECTION :Pool<AppConn> = establish_connection();
);
