use actix_web::{get, web, HttpResponse};
use common::data_result::WebResult;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(hello);
}

#[get("/hello")]
async fn hello() -> WebResult<HttpResponse> {
    let response = HttpResponse::Ok().body("hello app");
    Ok(response)
}
