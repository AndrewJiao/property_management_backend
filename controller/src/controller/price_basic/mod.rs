use actix_web::web::scope;

use crate::dto::price_basic::PriceBasicUpdateDto;
use crate::dto::ToUpdatePO;
use actix_web::{get, put, web, HttpResponse};
use common::data_result::{AppResult, PaginateSearch};
use common::db_config::auto_trait::AutoOperation;
use common::db_config::db_get_connection;
use common::{result_success, validate};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SaveChangesDsl, SelectableHelper};
use repository::price_basic::PriceBasicPo;
use repository::schema::basic::t_price_basic;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(scope("/price_basic")
        .service(get_data)
        .service(put_data)
    );
}


#[get("/data")]
async fn get_data(param: web::Query<PaginateSearch>) -> AppResult<HttpResponse> {
    validate!(param);
    let result = t_price_basic::table
        .filter(t_price_basic::is_delete.eq(false))
        .select(PriceBasicPo::as_select())
        .offset(param.off_set())
        .limit(param.limit())
        .load(&mut db_get_connection())?;

    let total: i64 = t_price_basic::table
        .filter(t_price_basic::is_delete.eq(false))
        .select(diesel::dsl::count_star())
        .first(&mut db_get_connection())?;

    result_success!(result, param.produce_page_result(total as i32))
}


#[put("/data/{data_id}")]
async fn put_data(param: web::Path<i64>, info: web::Json<PriceBasicUpdateDto>) -> AppResult<HttpResponse> {
    validate!(info);
    let data_id = param.into_inner();

    let _ = info
        .to_update_po(data_id as i32)
        .update_time()
        .save_changes::<PriceBasicPo>(&mut db_get_connection());
    result_success!()
}