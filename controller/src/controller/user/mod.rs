use crate::controller::user::inner::ComputeUserResult;
use crate::dto::user::{SearchType, UserCreateDto, UserLoginDto, UserResultDto, UserSearchDto, UserUpdateDto};
use crate::dto::{ToInsertPO, ToUpdatePO};
use actix_web::web::scope;
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use actix_web::cookie::{Cookie, CookieBuilder};
use common::data_result::{PaginateSearch, WebResult};
use common::error::BaseError::AnyhowError;
use common::error::USER_ACCOUNT_EXIST;
use common::{result_success, validate};
use common::tools::jwt::JWT_TOKEN_KEY;
use repository::user::UserPo;
mod inner;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(scope("/user_info")
                    .service(put_data)
                    .service(post_data)
                    .service(get_data)
                    .service(delete_data)
    );
}

#[get("data")]
pub async fn get_data(param: web::Query<PaginateSearch>) -> WebResult<HttpResponse> {
    let param = param.into_inner();
    let search_param = param.convert_param::<UserSearchDto>()?;
    validate!(search_param,param);
    let (result, total) = UserPo::search(
        search_param.account.as_deref(),
        search_param.role_type.as_ref(),
        search_param.name.as_deref(),
        search_param.binding_room_number.as_ref(),
        search_param.create_time_star.as_ref(),
        search_param.create_time_end.as_ref(),
        search_param.update_time_star.as_ref(),
        search_param.update_time_end.as_ref(),
        param.current_page(),
        param.limit(),
    )?;
    //分组
    let result = result
        .into_iter().map(|e| e.compute_user_result())
        .collect::<Vec<UserResultDto>>();
    result_success!(result, param.produce_page_result(total))

}
#[post("data")]
pub async fn post_data(param: web::Json<UserCreateDto>) -> WebResult<HttpResponse> {
    validate!(param);
    //验证账户已存在
    if UserPo::by_account(&param.account).is_ok() {
        return Err(AnyhowError(USER_ACCOUNT_EXIST()));
    }
    let result = service::user::create_account(param.to_insert_po(), param.binding_room_number.clone())?;
    result_success!(result)
}

///
/// 更新用户信息
///
#[put("data/{id}")]
pub async fn put_data(path_param: web::Path<i64>, param: web::Json<UserUpdateDto>) -> WebResult<HttpResponse> {
    validate!(param);

    let result = service::user::put_data(param.to_update_po(path_param.into_inner()), param.binding_room_number.clone())?
        .compute_user_result();
    result_success!(result)
}


#[delete("data/{id}")]
pub async fn delete_data(path_param: web::Path<i64>) -> WebResult<HttpResponse> {
    let result = service::user::delete_data(path_param.into_inner())?
        .compute_user_result();
    result_success!(result)
}

///
/// 查询房间绑定列表
///
#[get("data/binding_room")]
pub async fn get_binding_room(param: web::Query<SearchType>) -> WebResult<HttpResponse> {

    match param.into_inner() {
        SearchType::Account(value) => {
            let result = UserPo::find_by_account(&value)?;
            result_success!(result)
        }
        SearchType::Name(value) => {
            let result = UserPo::find_by_name(&value)?;
            result_success!(result)
        }
        SearchType::BindingRoomNumber(_) => {
            todo!();
            result_success!()
        }
    }

}


///
/// 登录
///
#[put("login")]
pub async fn login(param: web::Json<UserLoginDto>) -> WebResult<HttpResponse> {
    validate!(param);
    let UserLoginDto { account, password } = param.into();
    let token_string = service::user::login(account, password)?;
    HttpResponse::Ok().cookie(Cookie::build(JWT_TOKEN_KEY, token_string)
        .secure(true).http_only(true).finish());
    result_success!()
}

///
/// 登出
///
#[get("logout")]
pub async fn logout() -> WebResult<HttpResponse> {
    HttpResponse::Ok()
        .cookie(Cookie::build(JWT_TOKEN_KEY, "").secure(true).http_only(true).finish());
    result_success!()
}


