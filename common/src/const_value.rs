use crate::conf::config_module::Settings;
use crate::db_config::establish_connection;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref SETTINGS: Settings = Settings::new().expect("Failed to load settings");
}
// 不能直接拿来用
lazy_static!(
    pub static ref DB_CONNECTION :Pool<ConnectionManager<PgConnection>> = establish_connection();
);
