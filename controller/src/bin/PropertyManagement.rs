#[cfg(feature = "picture_extract")]
use actix::{Actor, Addr};
use actix_web::web;
use common::conf::log4::init_log4j;
use common::const_value::DB_CONNECTION;
use common::data_result::AppResult;
use common::web_config::{build_service, DataTrait};
#[cfg(feature = "picture_extract")]
use service::picture_extract::PictureExtractor;

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
        controller::property_fee::config,
        controller::owner_fee::config,
        // controller::attachment::config,
    ];
    let data = build_web_data();
    Ok(build_service(configs, data)?.await?)
}


fn build_web_data() -> web::Data<AppData> {
    web::Data::new(AppData::new())
}

#[derive(Clone)]
pub struct AppData {
    #[cfg(feature = "picture_extract")]
    actors: Actors,
}
impl DataTrait for AppData {}

#[derive(Clone)]
#[cfg(feature = "picture_extract")]
pub struct Actors {
    picture_extractor: Addr<PictureExtractor>,
}

impl AppData {
    fn new() -> Self {
        AppData {
            #[cfg(feature = "picture_extract")]
            actors: Actors { picture_extractor: PictureExtractor.start() },
        }
    }
}



fn before_init() {
    init_log4j();
    // 初始化db
    let _ = DB_CONNECTION;
}
