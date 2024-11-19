use crate::dto::owner_info::{OwnerInfoSearchDto, OwnerInfoUpdateDto};
use actix_web::web::scope;
use actix_web::{get, put, web, HttpResponse};
use common::data_result::PaginateSearch;
use common::db_config::db_get_connection;
use common::error::AppResult;
use common::{result_success, validate};
use diesel::{QueryDsl, SelectableHelper, TextExpressionMethods};
use validator::ValidateRange;
use repository::component::page::Paginate;
use repository::owner_info::OwnerBasicInfoPo;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(scope("/owner_info")
        .service(get_info)
    );
}

use repository::schema::basic::t_owner_basic_info::*;

///
/// 获取用户基础信息
///
#[get("/info")]
async fn get_info(param: web::Query<PaginateSearch>, body_param: web::Json<OwnerInfoSearchDto>) -> AppResult<HttpResponse> {
    validate!(param, body_param);
    let mut statement = table.into_boxed();
    if let Some(ref e) = body_param.owner_name {
        statement = statement.filter(owner_name.like(e))
    }
    if let Some(ref e) = body_param.room_number {
        statement = statement.filter(room_number.like(e));
    }
    let (result, total) = statement
        .select(OwnerBasicInfoPo::as_select())
        .paginate(param.current_page()).per_page(param.limit())
        .load_and_count_pages(&mut db_get_connection())?;
    result_success!(result, param.produce_page_result(total as i32))
}

///
/// 修改用户
///
#[put("/info/{info_id}")]
async fn put_info(param: web::Path<i32>, body_param: web::Json<OwnerInfoUpdateDto>) -> AppResult<HttpResponse> {
    validate!(body_param);

    result_success!()
}





