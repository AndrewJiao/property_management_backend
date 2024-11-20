use common::conf::log4::init_log4j;
use common::const_value::DB_CONNECTION;
use common::error::AppResult;
use common::web_config::build_service;

#[path = "../controller/mod.rs"]
mod controller;
#[path = "../dto/mod.rs"]
mod dto;

#[actix_web::main]
async fn main() -> AppResult<()> {
    before_init();

    let configs = &[
        controller::price_basic::config,
        controller::owner_info::config,
        controller::hello::config,
        controller::room_info::config,
        controller::property_fee::config
    ];
    Ok(build_service(
        configs,
    )?.await?)
}


fn before_init() {
    init_log4j();

    // 初始化db
    let _ = DB_CONNECTION;
}


