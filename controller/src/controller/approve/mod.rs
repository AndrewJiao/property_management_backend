use crate::dto::approve::{ApproveActionDto, ApproveCreateDto, ApproveResultDto, ApproveSearchDto};
use crate::dto::{ToInsertPO, ToUpdatePO};
use actix_web::web::scope;
use actix_web::{get, post, put, web, HttpRequest, HttpResponse};
use common::data_result::{PaginateSearch, WebResult};
use common::db_config::auto_trait::AutoOperation;
use common::db_config::db_get_connection;
use common::{result_success, validate};
use repository::approve::ApprovePo;
use repository::tool_table;
use repository::tool_table::CountType;
use repository::user::{RoleType, UserPo};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(scope("/approve")
        .service(get_data)
        .service(change_state)
        .service(add_data)
    );
}

///
///
///
#[get("/data")]
pub async fn get_data(param: web::Query<PaginateSearch>,req: HttpRequest) -> WebResult<HttpResponse> {
    let search_param: ApproveSearchDto = param.convert_param()?;
    validate!(search_param);
    let (user_po, _) = UserPo::current_user_info(&req)?;
    let p_account_id = match user_po.role_type {
        RoleType::User => Some(user_po.account_id),
        _ => None
    };

    let (result,total) = ApprovePo::by_search_param(
        (search_param.create_time_star.as_ref(), search_param.create_time_end.as_ref()),
        search_param.approve_state.as_ref(),
        search_param.approve_type.as_ref(),
        search_param.order_no.as_deref(),
        p_account_id,
        (param.current_page(), param.limit()),
    )?;

    let result = result.into_iter().map(|x| x.into()).collect::<Vec<ApproveResultDto>>();
    result_success!(result, param.produce_page_result(total))
}

///
/// 审批
///
#[put("/action/{id}")]
pub async fn change_state(id: web::Path<i64>, param: web::Json<ApproveActionDto>,http_request: HttpRequest) -> WebResult<HttpResponse> {
    let dto = param.into_inner();
    let update_po = dto
        .to_update_po(id.into_inner())
        .update_time();
    let result = service::approve::change_state(update_po, http_request).await?;
    result_success!(result)
}

///
/// 有点蠢
/// 用户为登录，只能发起创建用户的审批请求
/// 也就意味着这个接口必须开放不做认证
///
#[post("/data")]
pub async fn add_data(param: web::Json<ApproveCreateDto>, req: HttpRequest) -> WebResult<HttpResponse> {
    let (user_po, _) = UserPo::current_user_info(&req)?;
    let param = param.into_inner();
    validate!(param);
    let mut result = param
        .to_insert_po()
        .create_time();
    result.order_no = tool_table::current_date_count(CountType::ApproveOrderNumber)?;
    result.account_id = &user_po.account_id;
    result.save(&mut db_get_connection())?;
    result_success!(result)
}