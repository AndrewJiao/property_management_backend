use crate::dto::owner_fee::{OwnerFeeDetailSearchDto, OwnerFeeDetailUpdateDto};
use crate::dto::ToUpdatePO;
use actix_web::web::scope;
use actix_web::{get, post, put, web, HttpResponse};
use bigdecimal::BigDecimal;
use common::data_result::{PaginateSearch, WebResult};
use common::db_config::db_get_connection;
use common::{result_success, validate};
use diesel::query_dsl::methods::OrderDsl;
use diesel::ExpressionMethods;
use repository::component::page::Paginate;
use repository::owner_fee::{DetailType, OwnerFeeDetailPo};
use repository::schema::basic::t_owner_fee_detail::*;
use service::owner_fee::value_object::StreamAddVal;
use std::clone::Clone;
use serde::Deserialize;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(scope("/owner_fee")
        .service(get_data)
        .service(put_data)
        .service(add_data)
    );
}

#[get("/data")]
async fn get_data(param: web::Query<PaginateSearch>) -> WebResult<HttpResponse> {
    let search_param: OwnerFeeDetailSearchDto = param.convert_param()?;
    let statement = OwnerFeeDetailPo::search_by_param(
        search_param.stream_id.as_deref(),
        search_param.room_number.as_deref(),
        search_param.detail_type.as_ref(),
        search_param.create_time_star.as_ref(), search_param.create_time_end.as_ref(),
        search_param.update_time_star.as_ref(), search_param.update_time_end.as_ref(),
    );

    let (result, total) =
        OrderDsl::order(statement, create_time.desc())
            .paginate(param.current_page())
            .per_page(param.limit())
            .load_and_count_pages::<OwnerFeeDetailPo>(&mut db_get_connection())?;
    result_success!(result, param.produce_page_result(total))
}

///
/// 修改流水
///
#[put("/data/{data_id}")]
async fn put_data(path_param: web::Path<i64>, param: web::Json<OwnerFeeDetailUpdateDto>) -> WebResult<HttpResponse> {
    validate!(param);
    let insert_po = param.to_update_po(path_param.into_inner());
    let result = service::owner_fee::put_data(insert_po)?;
    result_success!(result)
}

#[derive(Deserialize)]
pub struct StreamAdd {
    amount: Option<BigDecimal>,
    detail_type: DetailType,
    room_number: String,
}

#[post("/data")]
async fn add_data(param: web::Json<StreamAdd>) -> WebResult<HttpResponse> {
    service::owner_fee::new_data(
        StreamAddVal {
            stream_type: param.detail_type.clone(),
            room_number: param.room_number.clone(),
            amount: param.amount.clone(),
        }
    )?;
    result_success!()
}
