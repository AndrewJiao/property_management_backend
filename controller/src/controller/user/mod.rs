use crate::controller::user::inner::ComputeUserResult;
use crate::dto::owner_info::OwnerInfoResultDto;
use crate::dto::user::{SearchType, UserChangePasswordDto, UserCreateDto, UserInfoDetailResult, UserLoginDto, UserResultDto, UserSearchDto, UserUpdateDto, WeChartUserLoginDto, WeChartUserRegisterDto};
use crate::dto::{ToInsertPO, ToUpdatePO};
use actix_web::web::scope;
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use common::data_result::{AppDataResult, PaginateSearch, WebResult};
use common::db_config::db_get_connection;
use common::error::BaseError::AnyhowError;
use common::error::USER_ACCOUNT_EXIST;
use common::tools::jwt::create_jwt_token_cookie;
use common::tools::time::now_utc_date_time_naive;
use common::{result_success, validate};
use repository::owner_info::OwnerBasicInfoPo;
use repository::user::relate::UserRelateRoomPo;
use repository::user::{RoleType, UserPo};
use repository::user::fast_login::UserFastLoginPo;
use service::user::value::LoginType;

mod inner;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(scope("/user_info")
        .service(put_data)
        .service(post_data)
        .service(get_data)
        .service(delete_data)
        .service(login)
        .service(logout)
        .service(get_by_account)
        .service(we_chart_login)
        .service(register)
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
    let result = service::user::create_account(param.to_insert_po(), param.binding_room_number.clone(),&mut db_get_connection())?;
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

#[put("data/change_password")]
pub async fn change_password(param: web::Json<UserChangePasswordDto>) -> WebResult<HttpResponse> {
    validate!(param);
    let UserChangePasswordDto { account, old_password, new_password } = param.into_inner();
    let result = service::user::change_password(account, old_password, new_password)?;
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
        SearchType::BindingRoomNumber(value) => {
            let result = UserRelateRoomPo::by_room_number_like(&value)?;
            result_success!(result)
        }
    }

}


///
/// 根据accountId获取用户信息
///
#[get("data_info")]
pub async fn get_by_account(req:HttpRequest) -> WebResult<HttpResponse> {
    let (user_po, room_numbers) = UserPo::current_user_info(&req)?;
    //管理员有所有房间信息，所以不需要再返回房间信息了避免数据过大
    let relate_owner_info = match user_po.role_type {
        RoleType::User => {
            OwnerBasicInfoPo::by_room_number_flow(room_numbers.as_ref(), 0, 100).ok()
                .map(|e| {
                    return e.into_iter()
                        .map(|item| item.into())
                        .collect::<Vec<OwnerInfoResultDto>>()
                })
        }
        _ => {
            // None
            OwnerBasicInfoPo::by_room_number_flow(room_numbers.as_ref(), 0, 100).ok()
                .map(|e| {
                    return e.into_iter()
                        .map(|item| item.into())
                        .collect::<Vec<OwnerInfoResultDto>>()
                })
        }
    };
    let result = UserInfoDetailResult {
        id: user_po.id,
        account_id: user_po.account_id,
        account: user_po.account,
        name: user_po.name,
        role_type: user_po.role_type,
        create_by: user_po.create_by,
        update_by: user_po.update_by,
        create_time: user_po.create_time,
        update_time: user_po.update_time,
        comment: None,
        relate_room_infos: relate_owner_info,
    };
    result_success!(result)
}



///
/// 登录
///
#[put("login")]
pub async fn login(param: web::Json<UserLoginDto>) -> WebResult<HttpResponse> {
    validate!(param);
    let UserLoginDto { account, password, code } = param.into_inner();
    let response;
    if let Some(code) = code {
        let (user_po, token_string) = service::user::login(LoginType::PasswordAndBindCode(account, password, code)).await?;
        response = write_auth_cookie(user_po, &token_string);
    } else {
        let (user_po, token_string) = service::user::login(LoginType::Password(account, password)).await?;
        response = write_auth_cookie(user_po, &token_string);
    }
    Ok(response)
}

///
/// 微信小程序登录
///
#[put("we_chart_login")]
pub async fn we_chart_login(param: web::Json<WeChartUserLoginDto>) -> WebResult<HttpResponse> {
    validate!(param);
    let WeChartUserLoginDto { code ,fast_login_flag} = param.into_inner();
    let (user_po, token_string) = service::user::login(LoginType::WeChartCode(code,fast_login_flag)).await?;
    let response = write_auth_cookie(user_po, &token_string);
    Ok(response)
}

///
/// 暂时只支持微信小程序注册
///

#[post("register")]
pub async fn register(param: web::Json<WeChartUserRegisterDto>) -> WebResult<HttpResponse> {
    validate!(param);
    let user_info = service::user::register(&param.nick_name, &param.code).await?;
    result_success!(user_info)
}


fn write_auth_cookie(user_po: UserPo, token_string: &String) -> HttpResponse {
    let response = HttpResponse::Ok()
        .cookie(create_jwt_token_cookie(&token_string))
        .json(
            AppDataResult {
                data: user_po,
                code: 200,
                message: "success".to_string(),
                time_stamp: now_utc_date_time_naive(),
                paginate_result: None,
            }
        );
    response
}

///
/// 登出
///
#[put("logout")]
pub async fn logout(http_request: HttpRequest) -> WebResult<HttpResponse> {
    let (user_po,_) = UserPo::current_user_info(&http_request)?;
    UserFastLoginPo::delete_user_fast_login(&user_po.account_id, &mut db_get_connection())?;

    let result = HttpResponse::Ok()
        .cookie(create_jwt_token_cookie(""))
        .json(
            AppDataResult {
                data: (),
                code: 200,
                message: "success".to_string(),
                time_stamp: now_utc_date_time_naive(),
                paginate_result: None,
            }
        );
    Ok(result)
}


