use crate::error::BaseError;
use config::{Config, ConfigError};
use log::info;
use serde::Deserialize;

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
        //看当前文件路径
        let file_path = std::env::current_dir().unwrap();
        println!("{:?}", file_path);

        //看当前项目路径
        let project_path = std::env::current_exe().unwrap();
        println!("{:?}", project_path);

        let s = Config::builder()
            .add_source(config::File::with_name("common/config"))
            .build()?;
        s.try_deserialize()
    }
}

pub fn init_settings() -> Result<Settings, BaseError> {
    //准备配置文件
    let settings = Settings::new();
    info!("{settings:?}");
    Ok(settings?)
}