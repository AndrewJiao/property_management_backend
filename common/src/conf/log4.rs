use crate::conf::get_current_config_dir_path;
use log::info;

pub fn init_log4j() {
    let config_dir_path = get_current_config_dir_path("log4rs.yaml");
    println!("config_dir_path: {}", config_dir_path);
    log4rs::init_file(config_dir_path, Default::default()).unwrap();
    info!("booting up");
}