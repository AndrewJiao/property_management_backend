use std::env::current_dir;
use std::path::PathBuf;

pub mod config_module;
pub mod log4;


pub(crate) fn get_current_config_dir() -> PathBuf {
    let current_dir = current_dir().expect("current_dir not set");
    current_dir.join("config_dir")
}
pub(crate) fn get_current_config_dir_path(config: &str) -> String {
    let config_dir = get_current_config_dir();
    format!("{}/{}", config_dir.to_str().unwrap(), config)
}