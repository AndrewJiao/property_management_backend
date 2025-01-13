use crate::dto::owner_fee::{OwnerFeeDetailResultDto, OwnerFeeDetailSearchDto, OwnerFeeDetailUpdateDto, StreamAddDetailType};
use crate::dto::ToUpdatePO;
use actix_web::web::scope;
use actix_web::{get, post, put, web, HttpRequest, HttpResponse};
use bigdecimal::{BigDecimal, Zero};
use common::data_result::{AppResult, PaginateSearch, WebResult};
use common::db_config::db_get_connection;
use common::error::BaseError::AnyhowError;
use common::error::PARAM_NOT_SUPPORT;
use common::{result_success, validate};
use diesel::query_dsl::methods::OrderDsl;
use diesel::ExpressionMethods;
use log::debug;
use repository::component::page::Paginate;
use repository::owner_fee::OwnerFeeDetailPo;
use repository::schema::basic::t_owner_fee_detail::*;
use repository::user::UserPo;
use std::clone::Clone;
use std::collections::HashSet;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(scope("/owner_fee")
        .service(get_data)
        .service(put_data)
        .service(add_data)
        .service(get_data_with_auth)
    );
}

///
/// 查询明细
/// 通过记录节点计算每个明细的余额
///
#[get("/data")]
async fn get_data(param: web::Query<PaginateSearch>) -> WebResult<HttpResponse> {
    let search_param: OwnerFeeDetailSearchDto = param.convert_param()?;
    let (result_dto, total) = do_search(param.current_page(), param.limit(), search_param).await?;
    result_success!(result_dto, param.produce_page_result(total))
}

async fn do_search(current_page: i64, page_size: i64, search_param: OwnerFeeDetailSearchDto) -> AppResult<(Vec<OwnerFeeDetailResultDto>, i64)> {
    let statement = OwnerFeeDetailPo::search_by_param(
        search_param.stream_id.as_deref(),
        search_param.room_number.as_deref(),
        search_param.room_numbers.as_ref(),
        search_param.detail_type.as_ref(),
        search_param.create_time_star.as_ref(), search_param.create_time_end.as_ref(),
        search_param.update_time_star.as_ref(), search_param.update_time_end.as_ref(),
    );

    let (result, total) =
        OrderDsl::order(statement, create_time.desc())
            .paginate(current_page)
            .per_page(page_size)
            .load_and_count_pages::<OwnerFeeDetailPo>(&mut db_get_connection())?;

    let record_ids = result.iter().map(|e| e.record_id.as_str()).collect::<Vec<&str>>();
    let amount_map = &service::owner_fee::re_calculate_amount_balance(&record_ids).await?;
    debug!("amount_map = {:?}", amount_map);
    //查询关联的流水
    let stream_id_list = result.iter().map(|e| e.stream_id.as_str()).collect();
    let all_relative_stream_data = OwnerFeeDetailPo::by_relative_order_number(&stream_id_list)?;
    let all_hash_relative_stream_data_id: HashSet<String> = all_relative_stream_data.into_iter()
        .map(|e| (e.related_order_number))
        .collect();

    let result_dto = result.into_iter()
        .map(|e| {
            let v_amount_balance = amount_map.get(e.stream_id.as_str())
                .unwrap_or(&BigDecimal::zero())
                .clone();
            OwnerFeeDetailResultDto::new(e, v_amount_balance, &all_hash_relative_stream_data_id)
        })
        .collect::<Vec<OwnerFeeDetailResultDto>>();
    Ok((result_dto, total))
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

#[post("/data")]
async fn add_data(param: web::Json<serde_json::Value>) -> WebResult<HttpResponse>
where
{
    let dto: StreamAddDetailType = param.into_inner().into();
    match dto {
        StreamAddDetailType::ManagementFee(e) => {
            validate!(e);
            let result = service::owner_fee::add_data(&e.room_number, &e.version)?;
            result_success!(result)
        }
        StreamAddDetailType::ManagementFeeBatch(e) => {
            validate!(e);
            service::owner_fee::add_datas(&e.version)?;
            result_success!()
        }
        StreamAddDetailType::PreStoreFee(e) => {
            validate!(e);
            let result = service::owner_fee::manually_add_data(e.amount, e.room_number)?;
            result_success!(result)
        }
        StreamAddDetailType::SettlementFee(e) => {
            validate!(e);
            let result = service::owner_fee::manually_add_settle_data(e.stream_id)?;
            result_success!(result)
        }
        StreamAddDetailType::NoMatch => {
            Err(AnyhowError(PARAM_NOT_SUPPORT()))
        }
    }
}

#[get("/auth_data")]
async fn get_data_with_auth(param: web::Query<PaginateSearch>, req: HttpRequest) -> WebResult<HttpResponse> {
    let mut search_param: OwnerFeeDetailSearchDto = param.convert_param()?;
    validate!(param, search_param);
    let (_, relate_room) = UserPo::current_user_info(&req)?;
    //管理员由所有的room权限，所以可以在这里筛选
    if let (Some(mut p_room_param), Some(ref room_number_filter)) = (relate_room, &search_param.room_number) {
        p_room_param.retain(|item| room_number_filter.contains(&item.to_string()));
        search_param.room_numbers = Some(p_room_param);
    }

    let (result_dto, total) = do_search(param.current_page(), param.limit(), search_param).await?;
    result_success!(result_dto, param.produce_page_result(total))
}
