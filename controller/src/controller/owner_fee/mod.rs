use crate::dto::owner_fee::OwnerFeeDetailSearchDto;
use actix_web::web::scope;
use actix_web::{get, web, HttpResponse};
use common::data_result::PaginateSearch;
use common::error::AppResult;
use diesel::QueryDsl;
use common::result_success;
use repository::schema::basic::t_owner_fee_detail::*;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(scope("/owner_fee")
        .service(get_data)
    );
}

#[get("/data")]
async fn get_data(param: web::Query<PaginateSearch>) -> AppResult<HttpResponse> {
    let search_param: OwnerFeeDetailSearchDto = param.convert_param()?;
    let statement = table.into_boxed();






    result_success!()
}
