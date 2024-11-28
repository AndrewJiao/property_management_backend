use crate::dto::room_info::{RoomInfoDetailSearchDto, RoomInfoDetailUpdateDto, RoomInfoSearchType};
use crate::dto::ToUpdatePO;
use actix_web::web::scope;
use actix_web::{get, post, put, web, HttpResponse};
use common::data_result::{AppResult, PaginateSearch};
use common::db_config::auto_trait::AutoOperation;
use common::db_config::db_get_connection;
use common::{result_success, validate};
use diesel::query_dsl::methods::GroupByDsl;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SaveChangesDsl, SelectableHelper, TextExpressionMethods};
use log::info;
use repository::component::page::Paginate;
use repository::room_info::RoomInfoDetailPo;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(scope("/room_info")
        .service(get_data)
        .service(put_data)
        .service(get_find)
        .service(init_data)
    );
}

use crate::controller::IfFilter;
use repository::schema::basic::t_room_info_detail::*;
use service::room_info::init_room_data;

///
/// 查询
///
#[get("/data")]
async fn get_data(param: web::Query<PaginateSearch>) -> AppResult<HttpResponse> {
    let search_param: RoomInfoDetailSearchDto = param.convert_param()?;
    validate!(&param, &search_param);
    print!("search_param:{:?}", &search_param);

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
        .if_filter(&search_param.room_number, |sub_sql, p| sub_sql.filter(room_number.like(format!("{}%", p))))
        .if_filter(&search_param.month_version, |sub_sql, p| sub_sql.filter(month_version.eq(p)))
        .filter(is_delete.eq(false));
    info!("sql:{}", diesel::debug_query::<diesel::pg::Pg, _>(&statement));
    let (result, total) = QueryDsl::order(
        statement.select(RoomInfoDetailPo::as_select()),
        create_time.desc())
        .paginate(param.current_page()).per_page(param.limit())
        .load_and_count_pages(&mut db_get_connection())?;

    result_success!(result, param.produce_page_result(total))
}


///
/// 编辑
///
#[put("/data/{data_id}")]
async fn put_data(path_param: web::Path<i64>, body_param: web::Json<RoomInfoDetailUpdateDto>) -> AppResult<HttpResponse> {
    validate!(body_param);
    let data_id = path_param.into_inner();
    let save_data_before = table.find(data_id).first::<RoomInfoDetailPo>(&mut db_get_connection())?;
    let result: RoomInfoDetailPo = body_param.to_update_po(data_id)
        .full_filed(&save_data_before)
        .re_calculate()
        .update_time()
        .save_changes(&mut db_get_connection())?;
    result_success!(result)
}

///
/// 获取版本
///
#[get("/find")]
async fn get_find(param: web::Query<RoomInfoSearchType>) -> AppResult<HttpResponse> {
    match param.into_inner() {
        RoomInfoSearchType::MonthVersion(ref value) => {
            if value.is_empty() {
                return result_success!(Vec::<String>::new());
            }
            let result = GroupByDsl::group_by(
                table.select(month_version)
                    .filter(month_version.is_not_null())
                    .filter(month_version.ne(""))
                    .filter(month_version.like(format!("%{}%", value))), month_version)
                .get_results::<Option<String>>(&mut db_get_connection())?;

            result_success!(result)
        }
    }
}


///
/// 初始化数据
///
#[post("/init")]
async fn init_data() -> AppResult<HttpResponse> {
    init_room_data()?;
    result_success!()
}

