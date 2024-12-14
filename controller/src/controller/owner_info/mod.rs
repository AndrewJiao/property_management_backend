use crate::dto::owner_info::{OwnerInfoInsertDto, OwnerInfoSearchDto, OwnerInfoSearchType, OwnerInfoUpdateDto};
use actix_web::web::scope;
use actix_web::{delete, get, post, put, web, HttpResponse};
use common::data_result::{ WebResult};
use common::data_result::PaginateSearch;
use common::db_config::auto_trait::AutoOperation;
use common::db_config::db_get_connection;
use common::{result_success, validate};
use diesel::query_dsl::methods::OrderDsl;
use diesel::{ExpressionMethods, Insertable, QueryDsl, RunQueryDsl, SaveChangesDsl, SelectableHelper, TextExpressionMethods};
use log::info;
use repository::component::page::Paginate;
use repository::owner_info::OwnerBasicInfoPo;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(scope("/owner_info")
        .service(get_info)
        .service(put_info)
        .service(add_info)
        .service(get_find)
        .service(delete_info)
    );
}
use crate::dto::{ToInsertPO, ToUpdatePO};
use repository::schema::basic::t_owner_basic_info::*;
use repository::soft_delete_by_id;

///
/// 获取用户基础信息
///
#[get("/info")]
async fn get_info(param: web::Query<PaginateSearch>) -> WebResult<HttpResponse> {
    let search_param: OwnerInfoSearchDto = param.convert_param()?;
    validate!(param, &search_param);

    let mut statement = table.into_boxed();
    if let Some(e) = search_param.owner_name.as_deref() {
        statement = statement.filter(owner_name.like(format!("%{}%", e)))
    }
    if let Some(e) = search_param.room_number.as_deref() {
        statement = statement.filter(room_number.like(format!("%{}%", e)));
    }
    let (result, total) =
        OrderDsl::order(statement
                            .filter(is_delete.eq(false))
                            .select(OwnerBasicInfoPo::as_select()),
                        create_time.desc())
            .paginate(param.current_page()).per_page(param.limit())
            .load_and_count_pages(&mut db_get_connection())?;
    result_success!(result, param.produce_page_result(total))
}

///
/// 修改用户
///
#[put("/info/{info_id}")]
async fn put_info(path: web::Path<i64>, body_param: web::Json<OwnerInfoUpdateDto>) -> WebResult<HttpResponse> {
    let info_id = path.into_inner();
    validate!(body_param);
    info!("param = {:?}", body_param);
    let result: OwnerBasicInfoPo = body_param
        .to_update_po(info_id)
        .update_time()
        .save_changes(&mut db_get_connection())?;
    result_success!(result)
}
///
/// 新增用户
///

#[post("/info")]
async fn add_info(body_param: web::Json<OwnerInfoInsertDto>) -> WebResult<HttpResponse> {
    validate!(body_param);
    body_param
        .to_insert_po()
        .update_time()
        .insert_into(table)
        .execute(&mut db_get_connection())?;
    result_success!()
}

#[delete("/info/{info_id}")]
async fn delete_info(path: web::Path<i32>) -> WebResult<HttpResponse> {
    soft_delete_by_id!(path.into_inner());

    result_success!()
}

#[get("/find")]
async fn get_find(param: web::Query<OwnerInfoSearchType>) -> WebResult<HttpResponse> {
    match param.into_inner() {
        OwnerInfoSearchType::RoomNumber(ref value) => {
            if value.is_empty() {
                return result_success!(Vec::<String>::new());
            }

            let result = QueryDsl::group_by(
                table.select(room_number)
                    .filter(room_number.is_not_null())
                    .filter(room_number.ne(""))
                    .filter(room_number.like(format!("%{}%", value))), room_number)
                    .filter(is_delete.eq(false))
                .get_results::<String>(&mut db_get_connection())?;
            result_success!(result)
        },
        OwnerInfoSearchType::OwnerName(ref value)=>{
                if value.is_empty() {
                    return result_success!(Vec::<String>::new());
                }
            let result = QueryDsl::group_by(
                table.select(owner_name)
                    .filter(owner_name.is_not_null())
                    .filter(owner_name.ne(""))
                    .filter(owner_name.like(format!("%{}%", value))), owner_name)
                    .filter(is_delete.eq(false))
                .get_results::<Option<String>>(&mut db_get_connection())?
                .into_iter().flat_map(|e|e).collect::<Vec<String>>();
                result_success!(result)
            }

    }
}

