use crate::dto::property_fee::{PropertyFeeDetailInitDto, PropertyFeeDetailResultDto, PropertyFeeDetailSearchDto, PropertyFeeDetailUpdateDto};
use actix_web::web::scope;
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use base64::engine::general_purpose;
use base64::Engine;
use common::data_result::{AppResult, OffsetSearch, Order, OrderType, PaginateSearch, WebResult};
use common::db_config::db_get_connection;
use common::error::BaseError::AnyhowError;
use common::error::{BUSINESS_ERROR_OWNER_FEE_DETAIL_EXIST};
use common::{result_success, validate};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper, TextExpressionMethods};
use log::debug;
use regex::Regex;
use repository::component::page::Paginate;
use repository::owner_fee::OwnerFeeDetailPo;
use repository::property_fee::PropertyFeeDetailPo;
use std::collections::HashMap;
use std::ops::Deref;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(scope("/property_fee")
        .service(get_data)
        .service(put_data)
        .service(init_data)
        .service(delete_data)
        .service(export_data)
        .service(get_data_card)
        .service(data_detail)
    );
}

use crate::controller::IfFilter;
use crate::dto::ToUpdatePO;
use repository::schema::basic::t_property_fee_detail::*;
use repository::user::UserPo;
use service::property_fee::do_edit_update;

#[get("/data")]
async fn get_data(param: web::Query<PaginateSearch>) -> WebResult<HttpResponse> {
    let search_param: PropertyFeeDetailSearchDto = param.convert_param()?;
    let search_order = param.convert_order()?;
    validate!(param, search_param);

    let (result, total) = do_search(param.current_page(), param.limit(),&search_order, &search_param)?;

    // 获取关联单号
    let owner_fee_param = result.iter()
        .flat_map(|e| {
            match (&e.room_number, &e.record_version) {
                (Some(p_room_number), Some(p_record_version)) => Some((p_room_number.as_ref(), p_record_version.as_ref())),
                _ => None
            }
        }).collect();
    let relative_owner_fee: HashMap<String, String> = OwnerFeeDetailPo::by_room_number_and_relative_order_numbers(&owner_fee_param, &mut db_get_connection())?
        .into_iter().map(|e| (format!("{}-{}", e.room_number, e.related_order_number), e.stream_id)).collect();

    let result_dto = PropertyFeeDetailResultDto::from_vec(result, &relative_owner_fee);

    result_success!(result_dto, param.produce_page_result(total))
}

fn do_search(current_page: i64, page_size: i64, order_types: &Option<Vec<Order>>, search_param: &PropertyFeeDetailSearchDto) -> AppResult<(Vec<PropertyFeeDetailPo>, i64)> {
    debug!("search_param: {:?} order: {:?}", search_param, order_types);
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
        .if_filter(&search_param.is_settle_down, |sub_statement, value| sub_statement.filter(is_settle_down.eq(value)))
        .filter(is_delete.eq(false));

    if let Some(order_types) = order_types{
        for order in order_types {
            statement = match order.field_name.as_str() {
                "roomNumber" => match order.order_type {
                    OrderType::Desc => statement.order(room_number.desc()),
                    OrderType::Asc => statement.order(room_number.asc()),
                },
                "createTime" => match order.order_type {
                    OrderType::Desc => statement.order(create_time.desc()),
                    OrderType::Asc => statement.order(create_time.asc()),
                },
                "updateTime" => match order.order_type {
                    OrderType::Desc => statement.order(update_time.desc()),
                    OrderType::Asc => statement.order(update_time.asc()),
                },
                _ => statement,
            };
        }
    }
    let (result, total) =
            statement.select(PropertyFeeDetailPo::as_select())
            .paginate(current_page).per_page(page_size)
            .load_and_count_pages(&mut db_get_connection())?;
    Ok((result,total))
}

///
/// 修改水电数据
///
#[put("/data/{data_id}")]
async fn put_data(path_param: web::Path<i64>, body_param: web::Json<PropertyFeeDetailUpdateDto>) -> WebResult<HttpResponse> {
    validate!(body_param);
    let result = do_edit_update(body_param.into_inner().to_update_po(path_param.into_inner()))?;
    result_success!(result)
}

///
/// 初始化
///
#[post("/data")]
async fn init_data(param: web::Json<PropertyFeeDetailInitDto>) -> WebResult<HttpResponse> {
    validate!(param);
    service::property_fee::init_data(param.month_version.as_deref())?;
    result_success!()
}

///
/// 删除
///
#[delete("/data/{data_id}")]
async fn delete_data(path_param: web::Path<i64>) -> WebResult<HttpResponse> {
    let conn = &mut db_get_connection();
    let p_id = path_param.into_inner();
    let po = PropertyFeeDetailPo::by_id(p_id).first::<PropertyFeeDetailPo>(conn)?;
    let exist_owner_fees = OwnerFeeDetailPo::by_room_number_and_relative_order_numbers(&vec![(po.room_number.unwrap().deref(), po.record_version.unwrap().deref())],conn)?
        .into_iter().next();
    debug!("exist_owner_fees: {:?}", exist_owner_fees);
    if exist_owner_fees.is_some() {
        return Err(AnyhowError(BUSINESS_ERROR_OWNER_FEE_DETAIL_EXIST()));
    }

    diesel::update(table)
        .filter(id.eq(p_id))
        .set((is_delete.eq(true), delete_at.eq(chrono::Local::now().naive_local())))
        .execute(conn)?;
    result_success!()
}

///
/// 导出
///
#[get("/export")]
async fn export_data(param: web::Query<PaginateSearch>) -> WebResult<HttpResponse> {
    let search_param: PropertyFeeDetailSearchDto = param.convert_param()?;
    let search_order = param.convert_order()?;
    validate!(param, search_param);

    let mut all_result = Vec::new();
    let mut current_page = 1;
    while let Ok((result, _)) = do_search(current_page, 2, &search_order, &search_param) {
        if result.is_empty() {
            break;
        }
        all_result.extend(result);
        current_page += 1;
    }
    let buffer = service::property_fee::excel::build_work_book(all_result)?;

    let mut file_name = String::from("海上明珠");
    let file_tail = ".xlsx";
    if let Some(p_record_version) = search_param.record_version{
        Regex::new(r"\w+-(?P<version>\S+)").unwrap().captures(&p_record_version).map(|cap| {
            let version = cap.name("version").unwrap().as_str();
            file_name.push_str(format!("-{version}").as_str());
        });
    }
    file_name.push_str(file_tail);

    debug!("write with file_name: {file_name}");
    let encode_file_name = general_purpose::STANDARD.encode(file_name);
    Ok(HttpResponse::Ok()
        .content_type("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet; charset=utf-8")
        .append_header(("Access-Control-Expose-Headers", "Content-Disposition"))
        .append_header(("Content-Disposition", format!("attachment; filename={encode_file_name}")))
        .body(buffer))
}

#[allow(dead_code)]
#[get("/data_card")]
async fn get_data_card(req: HttpRequest, param: web::Query<OffsetSearch>) -> WebResult<HttpResponse> {
    let param = param.into_inner();
    validate!(param);
    let (_, relate_room) = UserPo::current_user_info(&req)?;
    let room_param = relate_room.as_ref().map(|item| item.iter().map(|item| item.as_str()).collect::<Vec<&str>>());
    let data = PropertyFeeDetailPo::by_room_number_flow( room_param, param.offset, param.limit)?;
    result_success!(data)
}


///
/// 获取计算明细，详细列出计算过程
///
#[get("/data_detail")]
async fn data_detail(param: web::Query<PropertyFeeDetailSearchDto>) -> WebResult<HttpResponse> {
    let param = param.into_inner();
    validate!(param);
    if let(Some(p_room_number), Some(p_record_version)) = (&param.room_number, &param.record_version) {
        let data = PropertyFeeDetailPo::by_room_number_and_version(p_room_number, p_record_version, &mut db_get_connection())?;
        result_success!(data)
    } else {
        result_success!()
    }
}


