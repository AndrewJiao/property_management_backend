use crate::dto::user::{UserCreateDto, UserSearchDto, UserUpdateDto};
use crate::dto::{ToInsertPO, ToUpdatePO};
use actix_web::web::scope;
use actix_web::{delete, get, post, put, web, HttpResponse};
use common::data_result::{PaginateSearch, WebResult};
use common::db_config::auto_trait::AutoOperation;
use common::db_config::db_get_connection;
use common::error::BaseError::AnyhowError;
use common::error::{DATA_NOT_EXIST, USER_ACCOUNT_EXIST};
use common::{result_success, validate};
use diesel::{ExpressionMethods, QueryDsl, SaveChangesDsl};
use repository::component::page::Paginate;
use repository::schema::basic::t_user::create_time;
use repository::user::UserPo;

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
    let statement = UserPo::search(search_param.account.as_deref(),
                                search_param.binding_room_number.as_deref(),
                                search_param.role,
                                search_param.name.as_deref());

    let (result, total) =
        statement.order_by(create_time.desc())
            .paginate(param.current_page())
            .per_page(param.limit())
            .load_and_count_pages(&mut db_get_connection())?;
    result_success!(result, param.produce_page_result(total))
}
#[post("data")]
pub async fn post_data(param: web::Json<UserCreateDto>) -> WebResult<HttpResponse> {
    validate!(param);
    //验证账户已存在
    if let Some(_) = UserPo::by_account(&param.account) {
        return Err(AnyhowError(USER_ACCOUNT_EXIST()));
    }
    valid_room_number(&param.binding_room_number)?;

    let result = service::user::create_account(param.to_insert_po())?;
    result_success!(result)
}

///
/// 更新用户信息
///
#[put("data/{id}")]
pub async fn put_data(path_param: web::Path<i64>, param: web::Json<UserUpdateDto>) -> WebResult<HttpResponse> {
    validate!(param);
    valid_room_number(&param.binding_room_number)?;

    let result = param.to_update_po(path_param.into_inner())
        .update_time()
        .save_changes::<UserPo>(&mut db_get_connection())?;
    result_success!(result)
}

fn valid_room_number(param: &Option<String>) -> WebResult<()> {
    if let Some(ref room_number) = param {
        if repository::owner_info::OwnerBasicInfoPo::by_room_number(room_number, &mut db_get_connection()).is_err() {
            return Err(AnyhowError(DATA_NOT_EXIST()));
        }
    }
    Ok(())
}

#[delete("data/{id}")]
pub async fn delete_data(path_param: web::Path<i64>) -> WebResult<HttpResponse> {
    repository::user::delete_by_id(path_param.into_inner())?;
    result_success!()
}
