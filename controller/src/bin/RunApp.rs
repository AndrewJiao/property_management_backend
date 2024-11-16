use common::conf::log4::init_log4j;
use common::const_value::DB_CONNECTION;
use common::error::AppResult;
use common::web_config::build_service;

#[path = "../controller/mod.rs"]
mod controller;

#[actix_web::main]
async fn main() -> AppResult<()> {
    Ok(build_service(
        &[
            controller::hello::config,
        ],
    )?.await?)
}

fn before_init() {
    init_log4j();

    // 初始化db
    let _ = DB_CONNECTION;
}


