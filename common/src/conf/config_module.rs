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
    pub liquidated_damages_rate: BigDecimal,
    pub password_sec_key: String,
    pub jwt_expire_time: i64,
    pub jwt_renew_time: i64,
    pub jwt_secret: String,
    pub jwt_handler_ignore_path: Vec<String>,
}


#[derive(Debug, Deserialize)]
pub struct OpenSSl{
    pub private_key: String,
    pub certificate: String,
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
pub struct AliyunOssConfig {
    pub sts_host: String,
    pub oss_host: String,
    pub access_key_id: String,
    pub access_key_secret: String,
    pub sts_role_arn: String,
    pub region: String,
    pub bucket: String,
}
#[derive(Debug, Deserialize)]
pub struct AttachmentConfig {
    pub picture_suffix: Vec<String>,
    pub temp_output_dir: String,
}

#[derive(Debug, Deserialize)]
pub struct PictureConfig {
    pub owner_table_horizontal_offset: u32,
    pub owner_table_vertical_offset: u32,
}

#[derive(Debug, Deserialize)]
pub struct WeChartConfig{
    pub app_id: String,
    pub app_secret: String,
    pub host: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    pub web_config: WebConfig,
    pub databases: DatabasesConfig,
    pub app_config: AppConfig,
    pub excel_config: ExcelConfig,
    pub aliyun_oss_config: AliyunOssConfig,
    pub attachment_config: AttachmentConfig,
    pub picture_config: PictureConfig,
    pub open_ssl: OpenSSl,
    pub we_chart_config: WeChartConfig,
}
impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let config_path = match std::env::var("env").unwrap_or("dev".to_string()).as_str() {
            "prod" => {"config_prod"}
            _ => {"config"}
        };
        let config_dir = get_current_config_dir_path(config_path);
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