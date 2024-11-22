use std::env;
use log::info;

pub fn init_log4j() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    println!("manifest_dir: {}", manifest_dir);
    log4rs::init_file(format!("{}/../config_dir/log4rs.yaml", manifest_dir), Default::default()).unwrap();
    info!("booting up");
}