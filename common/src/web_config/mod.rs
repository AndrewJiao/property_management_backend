use crate::const_value::SETTINGS;
mod interceptors;
use crate::data_result::AppResult;
use crate::web_config::interceptors::jwt::JWTMiddleware;
use actix_web::dev::Server;
use actix_web::web::ServiceConfig;
use actix_web::{web, App, HttpServer};
use std::time::Duration;
use tracing_actix_web::TracingLogger;

pub trait DataTrait{}

pub fn build_service<F, T>(service_config: &'static [F; 7], data: web::Data<T>) -> AppResult<Server>
where
    F: FnOnce(&mut ServiceConfig) + Sync + Clone + Send,
    T: DataTrait + Send + Sync + Clone + 'static,
{
    let config = &SETTINGS.web_config;
    let server = HttpServer::new(
        move || {
            service_config.into_iter().fold(
                App::new().app_data(data.clone())
                    .wrap(create_cors())
                    .wrap(TracingLogger::default())
                    .wrap(JWTMiddleware)
                ,
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

fn create_cors() -> actix_cors::Cors {
    actix_cors::Cors::default()
        .allow_any_origin()
        .allow_any_method()
        .allow_any_header()
        .max_age(3600)
}
