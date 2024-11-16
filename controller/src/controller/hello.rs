use actix_web::{get, web, HttpResponse};
use common::error::AppResult;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(hello);
}

#[get("/hello")]
async fn hello() -> AppResult<HttpResponse> {
    Ok(HttpResponse::Ok().body("hello app"))
}
