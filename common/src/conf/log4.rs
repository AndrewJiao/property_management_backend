use log::info;

pub fn init_log4j() {
    log4rs::init_file("common/log4rs.yaml", Default::default()).unwrap();
    info!("booting up");
}