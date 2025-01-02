use crate::dto::approve::{ApproveActionDto, ApproveCreateDto, ApproveResultDto, ApproveSearchDto};
use crate::dto::{ToInsertPO, ToUpdatePO};
use actix_web::web::scope;
use actix_web::{get, post, put, web, HttpResponse};
use common::data_result::{PaginateSearch, WebResult};
use common::db_config::auto_trait::AutoOperation;
use common::db_config::db_get_connection;
use common::{result_success, validate};
use repository::approve::ApprovePo;
use repository::tool_table;
use repository::tool_table::CountType;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(scope("/approve")
        .service(get_data)
        .service(change_state)
        .service(add_data)
    );
}

#[get("/data")]
pub async fn get_data(param: web::Query<PaginateSearch>) -> WebResult<HttpResponse> {
    let search_param: ApproveSearchDto = param.convert_param()?;
    validate!(search_param);
    let (result,total) = ApprovePo::by_search_param(
        (search_param.create_time_star.as_ref(), search_param.create_time_end.as_ref()),
        search_param.approve_state.as_ref(),
        search_param.approve_type.as_ref(),
        search_param.order_no.as_deref(),
        (param.current_page(), param.limit()),
    )?;

    let result = result.into_iter().map(|x| x.into()).collect::<Vec<ApproveResultDto>>();
    result_success!(result, param.produce_page_result(total))
}

#[put("/action/{id}")]
pub async fn change_state(id: web::Path<i64>, param: web::Json<ApproveActionDto>) -> WebResult<HttpResponse> {
    let dto = param.into_inner();
    let update_po = dto
        .to_update_po(id.into_inner())
        .update_time();
    let result = service::approve::change_state(update_po)?;

    result_success!(result)
}


#[post("/data")]
pub async fn add_data(param: web::Json<ApproveCreateDto>) -> WebResult<HttpResponse> {
    let param = param.into_inner();
    validate!(param);
    let mut result = param
        .to_insert_po()
        .create_time();
    result.order_no = tool_table::current_date_count(CountType::ApproveOrderNumber)?;
    result.save(&mut db_get_connection())?;
    result_success!(result)
}