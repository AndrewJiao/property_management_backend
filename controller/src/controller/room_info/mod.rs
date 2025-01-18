use crate::dto::room_info::{RoomInfoDetailOffsetSearchDto, RoomInfoDetailSearchDto, RoomInfoDetailUpdateDto, RoomInfoManuallyInsertDto, RoomInfoSearchType};
use crate::dto::{ToInsertPO, ToUpdatePO};
use actix_web::web::scope;
use actix_web::{get, post, put, web,  HttpRequest, HttpResponse};
use common::data_result::{OffsetSearch, PaginateSearch, WebResult};
use common::db_config::auto_trait::AutoOperation;
use common::db_config::db_get_connection;
use common::error::{BaseError, BUSINESS_ERROR, DATA_HAS_EXIST, ROOM_IS_NOT_EXIST};
use common::{result_success, validate};
use diesel::query_dsl::methods::GroupByDsl;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SaveChangesDsl, SelectableHelper, Table, TextExpressionMethods};
use log::info;
use repository::component::page::Paginate;
use repository::property_fee::PropertyFeeDetailPo;
use repository::room_info::{ReCalculator, RoomInfoDetailPo};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(scope("/room_info")
        .service(get_data)
        .service(put_data)
        .service(get_find)
        .service(init_data)
        .service(post_data)
        .service(get_data_card)
    );
}

use crate::controller::IfFilter;
use repository::schema::basic::t_room_info_detail::dsl::t_room_info_detail;
use repository::schema::basic::t_room_info_detail::*;
use repository::user::UserPo;
use service::room_info::init_room_data;

///
/// 查询
///
#[get("/data")]
async fn get_data(param: web::Query<PaginateSearch>) -> WebResult<HttpResponse> {
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
async fn put_data(path_param: web::Path<i64>, body_param: web::Json<RoomInfoDetailUpdateDto>) -> WebResult<HttpResponse> {
    validate!(body_param);
    let data_id = path_param.into_inner();
    let save_data_before = table.find(data_id).first::<RoomInfoDetailPo>(&mut db_get_connection())?;
    let property_fee = PropertyFeeDetailPo::by_room_number_and_version(
        save_data_before.room_number.as_deref().ok_or(ROOM_IS_NOT_EXIST())?,
        save_data_before.month_version.as_deref().ok_or(ROOM_IS_NOT_EXIST())?, &mut db_get_connection()).ok();
    if property_fee.is_some() {
        return Err(BaseError::AnyhowError(BUSINESS_ERROR("费用已生成，不允许修改", 99999)));
    }
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
async fn get_find(param: web::Query<RoomInfoSearchType>) -> WebResult<HttpResponse> {
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
        RoomInfoSearchType::PreSearchBefore(ref p_room_number) => {
                if p_room_number.is_empty() {
                    return result_success!(Vec::<String>::new());
                }
                let result = table.select(t_room_info_detail::all_columns())
                    .filter(room_number.eq(p_room_number))
                    .filter(is_delete.eq(false))
                    .get_result::<RoomInfoDetailPo>(&mut db_get_connection())?;
                result_success!(result)
        }
    }
}


///
/// 初始化数据
///
#[post("/init")]
async fn init_data() -> WebResult<HttpResponse> {
    init_room_data()?;
    result_success!()
}
///
/// 手动新增
///
#[post("/data")]
async fn post_data(query_param: web::Json<RoomInfoManuallyInsertDto>) -> WebResult<HttpResponse> {
    let param = query_param.into_inner();
    validate!(param);
    if RoomInfoDetailPo::by_room_number_and_version(&param.room_number, &param.month_version)?.is_some(){
        return Err(BaseError::AnyhowError(DATA_HAS_EXIST()));
    }

    let _ = param.to_insert_po()
        .re_calculate()
        .update_time().save(&mut db_get_connection());
    result_success!()
}

///
/// 小程序不具备搜索能力，提供小程序获取表单接口
///
#[get("/data_card")]
async fn get_data_card(req: HttpRequest, param: web::Query<OffsetSearch>) -> WebResult<HttpResponse> {
    let param = param.into_inner();
    let search_param: RoomInfoDetailOffsetSearchDto = param.convert_param()?;
    validate!(param, search_param);

    let (_, relate_room) = UserPo::current_user_info(&req)?;
    let mut room_param = relate_room.as_ref().map(|item| item.iter().map(|item| item.as_str()).collect::<Vec<&str>>());
    //管理员由所有的room权限，所以可以在这里筛选
    if let (Some(ref mut room_param),Some(ref room_number_filter)) = (&mut room_param,search_param.room_number) {
        room_param.retain(|item| room_number_filter.contains(&item.to_string()));
    }

    let data = RoomInfoDetailPo::by_room_number_flow(
        room_param,
        Option::zip(search_param.create_time_star, search_param.create_time_end),
        search_param.only_not_completed,
        param.offset,
        param.limit)?;
    result_success!(data)
}




