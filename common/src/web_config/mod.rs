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

// fn create_openssl() -> SslAcceptorBuilder {
//     let config = &SETTINGS.open_ssl;
//     // 创建 SSL 接受器，用于 HTTPS 配置
//
//     let mut ssl_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
//     ssl_builder
//         .set_private_key_file(&config.private_key, SslFiletype::PEM)
//         .unwrap();
//     ssl_builder.set_certificate_chain_file(&config.certificate).unwrap();
//     ssl_builder
//     // 加载证书和私钥
//     let cert_file = File::open("cert.pem").unwrap();
//     let mut reader = BufReader::new(cert_file);
//     rustls::
//     let certs = rustls::internal::pemfile::certs(&mut reader).unwrap();
//
//     let key_file = File::open("key.pem").unwrap();
//     let mut reader = BufReader::new(key_file);
//     let mut keys = rustls::internal::pemfile::pkcs8_private_keys(&mut reader).unwrap();
//
//     // 使用 rustls 配置接受器
//     let rustls_config = rustls::ServerConfig::new(rustls::NoClientAuth::new());
//     let rustls_config = rustls_config
//         .with_single_cert(certs, keys.remove(0))
//         .expect("invalid key or certificate");
//     let rustls_config = Arc::new(rustls_config);
// }