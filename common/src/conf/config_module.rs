use crate::conf::get_current_config_dir_path;
use bigdecimal::BigDecimal;
// use crate::error::BaseError;
use config::{Config, ConfigError};
use log::info;
use serde::Deserialize;
use crate::data_result::AppResult;

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
    pub record_max: i32,
    pub liquidated_damages_rate: BigDecimal
}

#[derive(Debug, Deserialize)]
pub struct ExcelConfig {
    pub basic_height: u32,
    pub basic_width: u32,
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

///
/// 阿里云oss配置
///
#[derive(Debug, Deserialize)]
pub struct AliyunOssConfig{
    pub sts_host: String,
    pub oss_host: String,
    pub access_key_id: String,
    pub access_key_secret: String,
    pub sts_role_arn: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    pub web_config: WebConfig,
    pub databases: DatabasesConfig,
    pub app_config: AppConfig,
    pub excel_config: ExcelConfig,
    pub aliyun_oss_config: AliyunOssConfig,
}
impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let config_dir = get_current_config_dir_path("config");
        println!("config: {}", config_dir);
        let s = Config::builder()
            .add_source(config::File::with_name(config_dir.as_str()))
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