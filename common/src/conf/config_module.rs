use std::env;
// use crate::error::BaseError;
use config::{Config, ConfigError};
use log::info;
use serde::Deserialize;
use crate::error::AppResult;

///
/// web配置模块
///
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct WebConfig {
    pub host: String,
    pub port: u16,
    pub shutting_down_timeout: u64,
    pub keep_alive: u64,
}
///
/// 应用配置
///
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct AppConfig {
    pub json_length: u32,
    pub number_max: i32,
    pub number_min: i32,
}

///
/// 数据库配置
///
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct DatabasesConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
    pub database_url: String,
}


#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    pub web_config: WebConfig,
    pub databases: DatabasesConfig,
    pub app_config: AppConfig,
}
impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
        let s = Config::builder()
            .add_source(config::File::with_name(format!("{}/../config_dir/config", manifest_dir).as_str()))
            .build()?;
        s.try_deserialize()
    }
}

pub fn init_settings() -> AppResult<Settings> {
    //准备配置文件
    let settings = Settings::new();
    info!("{settings:?}");
    Ok(settings?)
}