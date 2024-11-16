use crate::const_value::SETTINGS;
use crate::error::AppResult;
use actix_web::dev::Server;
use actix_web::web::ServiceConfig;
use actix_web::{web, App, HttpServer};
use std::time::Duration;


pub fn build_service<F>(service_config: &'static [F]) -> AppResult<Server>
where
    F: FnOnce(&mut ServiceConfig) + Sync + Clone + Send,
{
    let data = web::Data::new({});
    let config = &SETTINGS.web_config;
    let server = HttpServer::new(
        move || {
            service_config.into_iter().fold(
                App::new().app_data(data.clone()),
                |app, conf| {
                    app.configure(conf.clone())
                },
            )
        })
        .keep_alive(Duration::from_secs(config.keep_alive))
        .shutdown_timeout(config.shutting_down_timeout)
        .bind((config.host.as_str(), config.port))?
        .run();
    Ok(server)
}

pub enum ServiceType {
    Web,
    Rpc,
    Grpc,
}


