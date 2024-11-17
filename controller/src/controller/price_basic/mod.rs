use actix_web::web::scope;

use crate::dto::price_basic::PriceBasic;
use actix_web::{get, put, web, HttpResponse};
use common::data_result::{AppResult, PaginateSearch};
use common::db_config::db_get_connection;
use common::result_success;
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

    let result = t_price_basic::table
        .filter(t_price_basic::name.eq("水费分摊"))
        .select(PriceBasicPo::as_select())
        // .offset(param.off_set())
        // .limit(param.limit())
        .load(&mut db_get_connection())?;
    result_success!(result)
}


#[put("/data/{data_id}")]
async fn put_data(param: web::Path<i64>, info: web::Json<PriceBasic>) -> AppResult<HttpResponse> {
    let data_id = param.into_inner();
    info.to_update_po(data_id)
        .save_changes::<PriceBasicPo>(&mut db_get_connection())?;
    result_success!()
}