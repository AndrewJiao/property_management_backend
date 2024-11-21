use crate::dto::property_fee::{PropertyFeeDetailSearchDto, PropertyFeeDetailUpdateDto};
use actix_web::web::scope;
use actix_web::{get, post, put, web, HttpResponse};
use common::data_result::{AppResult, PaginateSearch};
use common::db_config::db_get_connection;
use common::{result_success, validate};
use diesel::{ExpressionMethods, QueryDsl, SelectableHelper, TextExpressionMethods};
use repository::component::page::Paginate;
use repository::property_fee::PropertyFeeDetailPo;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(scope("/property_fee")
        .service(get_data)
        .service(put_data)
        .service(init_data)
    );
}

use crate::controller::IfFilter;
use repository::schema::basic::t_property_fee_detail::*;
use service::property_fee::do_edit_update;
use crate::dto::ToUpdatePO;

#[get("/data")]
async fn get_data(param: web::Query<PaginateSearch>, body_param: web::Json<PropertyFeeDetailSearchDto>) -> AppResult<HttpResponse> {
    validate!(param,body_param);
    let mut statement = table.into_boxed();
    statement = statement
        .if_filter_tow_param(
            &body_param.create_time_begin,
            &body_param.create_time_end,
            |sub_sql, (p1, p2)| sub_sql.filter(create_time.between(p1, p2)))
        .if_filter_tow_param(
            &body_param.update_time_begin,
            &body_param.update_time_end,
            |sub_sql, (p1, p2)| sub_sql.filter(update_time.between(p1, p2)))
        .if_filter(&body_param.room_number, |sub_sql, p| sub_sql.filter(room_number.like(format!("%{}%", p))))
        .if_filter(&body_param.room_owner_name, |sub_statement, value| sub_statement.filter(room_owner_name.like(format!("%{}%", value))))
        .if_filter(&body_param.record_version, |sub_statement, value| sub_statement.filter(record_version.eq(value)))
        .filter(is_delete.eq(false));


    let (result, total) =
        QueryDsl::order(
            statement.select(PropertyFeeDetailPo::as_select()), update_time.desc())
            .paginate(param.current_page()).per_page(param.limit())
            .load_and_count_pages(&mut db_get_connection())?;
    result_success!(result, param.produce_page_result(total as i32))
}

///
/// 修改水电数据j
///
#[put("/data/{data_id}")]
async fn put_data(path_param: web::Path<i32>, body_param: web::Json<PropertyFeeDetailUpdateDto>) -> AppResult<HttpResponse> {
    validate!(body_param);
    do_edit_update(body_param.to_update_po(path_param.into_inner()))?;

    result_success!()
}

///
/// 初始化
///
#[post("/data")]
async fn init_data() -> AppResult<HttpResponse> {
    service::property_fee::init_data()?;

    result_success!()
}
