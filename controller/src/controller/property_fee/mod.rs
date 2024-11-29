use crate::dto::property_fee::{PropertyFeeDetailInitDto, PropertyFeeDetailSearchDto, PropertyFeeDetailUpdateDto};
use actix_web::web::scope;
use actix_web::{delete, get, post, put, web, HttpResponse};
use common::data_result::{AppResult, PaginateSearch};
use common::db_config::db_get_connection;
use common::{result_success, validate};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper, TextExpressionMethods};
use repository::component::page::Paginate;
use repository::property_fee::PropertyFeeDetailPo;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(scope("/property_fee")
        .service(get_data)
        .service(put_data)
        .service(init_data)
        .service(delete_data)
    );
}

use crate::controller::IfFilter;
use crate::dto::ToUpdatePO;
use repository::schema::basic::t_property_fee_detail::*;
use service::property_fee::do_edit_update;

#[get("/data")]
async fn get_data(param: web::Query<PaginateSearch>) -> AppResult<HttpResponse> {
    let search_param: PropertyFeeDetailSearchDto = param.convert_param()?;
    validate!(param,search_param);
    let mut statement = table.into_boxed();
    statement = statement
        .if_filter_tow_param(
            &search_param.create_time_star,
            &search_param.create_time_end,
            |sub_sql, (p1, p2)| sub_sql.filter(create_time.between(p1, p2)))
        .if_filter_tow_param(
            &search_param.update_time_star,
            &search_param.update_time_end,
            |sub_sql, (p1, p2)| sub_sql.filter(update_time.between(p1, p2)))
        .if_filter(&search_param.room_number, |sub_sql, p| sub_sql.filter(room_number.like(format!("%{}%", p))))
        .if_filter(&search_param.room_owner_name, |sub_statement, value| sub_statement.filter(room_owner_name.like(format!("%{}%", value))))
        .if_filter(&search_param.record_version, |sub_statement, value| sub_statement.filter(record_version.eq(value)))
        .filter(is_delete.eq(false));


    let (result, total) =
        QueryDsl::order(
            statement.select(PropertyFeeDetailPo::as_select()), update_time.desc())
            .paginate(param.current_page()).per_page(param.limit())
            .load_and_count_pages(&mut db_get_connection())?;
    result_success!(result, param.produce_page_result(total))
}

///
/// 修改水电数据
///
#[put("/data/{data_id}")]
async fn put_data(path_param: web::Path<i64>, body_param: web::Json<PropertyFeeDetailUpdateDto>) -> AppResult<HttpResponse> {
    validate!(body_param);
    let result = do_edit_update(body_param.to_update_po(path_param.into_inner()))?;
    result_success!(result)
}

///
/// 初始化
///
#[post("/data")]
async fn init_data(param: web::Json<PropertyFeeDetailInitDto>) -> AppResult<HttpResponse> {
    validate!(param);
    service::property_fee::init_data(param.month_version.as_deref())?;
    result_success!()
}

///
/// 删除
///
#[delete("/data/{data_id}")]
async fn delete_data(path_param: web::Path<i32>) -> AppResult<HttpResponse> {
    diesel::update(table)
        .filter(id.eq(path_param.into_inner()))
        .set((is_delete.eq(true), delete_at.eq(chrono::Local::now().naive_local())))
        .execute(&mut db_get_connection())?;
    result_success!()
}
