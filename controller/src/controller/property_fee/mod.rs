use actix_web::{get, web, HttpResponse};
use actix_web::web::scope;
use common::data_result::{AppResult, PaginateSearch};
use common::{result_success, validate};
use crate::controller::price_basic::put_data;
use crate::dto::property_fee::PropertyFeeDetailSearchDto;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(scope("/property_fee")
        .service(get_data)
    );
}

#[get("/data")]
async fn get_data(param: web::Query<PaginateSearch>, body_param: web::Json<PropertyFeeDetailSearchDto>) -> AppResult<HttpResponse> {
    validate!(param,body_param);


    result_success!()
}
