use crate::dto::owner_info::{OwnerInfoInsertDto, OwnerInfoResultDto, OwnerInfoSearchDto, OwnerInfoSearchType, OwnerInfoUpdateDto};
use crate::dto::{ToInsertPO, ToUpdatePO};
#[cfg(feature = "picture_extract")]
use crate::AppData;
#[cfg(feature = "picture_extract")]
use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::web::scope;
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
#[cfg(feature = "picture_extract")]
use common::const_value::SETTINGS;
#[cfg(feature = "picture_extract")]
use common::data_result::AppResult;
use common::data_result::{OffsetSearch, PaginateSearch};
use common::data_result::WebResult;
use common::db_config::auto_trait::AutoOperation;
use common::db_config::db_get_connection;
#[cfg(feature = "picture_extract")]
use common::error::BUSINESS_ERROR;
use common::{result_success, validate};
use diesel::query_dsl::methods::OrderDsl;
use diesel::{ExpressionMethods, Insertable, QueryDsl, RunQueryDsl, SaveChangesDsl, SelectableHelper, TextExpressionMethods};
use log::info;
use repository::component::page::Paginate;
use repository::owner_info::OwnerBasicInfoPo;
use repository::schema::public::t_owner_basic_info::*;
use repository::soft_delete_by_id;
use repository::user::UserPo;
#[cfg(feature = "picture_extract")]
use service::picture_extract::dto::ExtractSender;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(scope("/owner_info")
        .service(get_info)
        .service(put_info)
        .service(add_info)
        .service(get_find)
        .service(delete_info)
        // .service(upload_picture)
    );
}

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
    if let Some(e) = search_param.room_type {
        statement = statement.filter(room_type.eq_any(e));
    }

    if let Some(true) = search_param.focus_vehicle {
        statement = statement.filter(other_basic.is_not_null())
    }

    let (result, total) =
        OrderDsl::order(statement
                            .filter(is_delete.eq(false))
                            .select(OwnerBasicInfoPo::as_select()),
                        room_number.desc())
            .paginate(param.current_page()).per_page(param.limit())
            .load_and_count_pages(&mut db_get_connection())?;
    let result = result.into_iter().map(OwnerInfoResultDto::from).collect::<Vec<OwnerInfoResultDto>>();
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

#[derive(Debug,MultipartForm)]
#[cfg(feature = "picture_extract")]
struct UploadForm{
    #[multipart(limit = "10MB")]
    file:TempFile,
}

#[post("/picture")]
#[cfg(feature = "picture_extract")]
async fn upload_picture(MultipartForm(form): MultipartForm<UploadForm>, data: web::Data<AppData>) -> WebResult<HttpResponse> {
    let file_name_opt = form.file.file_name;
    let suffix = file_name_opt.verify_extract_prefix()?;
    let file = form.file.file;

    let temp_file_name  = uuid_v7::gen_uuid_v7();

    let _ = data.actors.picture_extractor.send(
        ExtractSender::new(
            file.into_file(),
            format!("{temp_file_name}.{suffix}"
            ),
        )).await?;
    result_success!()
}


#[cfg(feature = "picture_extract")]
trait ExtractSuffix{
    fn verify_extract_prefix(&self) -> AppResult<String>;
}



#[cfg(feature = "picture_extract")]
impl ExtractSuffix for Option<String> {
    fn verify_extract_prefix(&self) -> AppResult<String> {
        let suffix_vec = &SETTINGS.attachment_config.picture_suffix;
        let regex = regex::Regex::new(r"^\w+.(?<suffix>[a-zA-Z0-9]+)$")?;


        let extracted_suffix =  match self {
            Some(file_name) => {
                info!("file_name = {:?}",file_name);
                regex.captures(file_name)
                    .map(|e| e.name("suffix").map(|e| e.as_str().to_string()))
                    .flatten().ok_or(BUSINESS_ERROR("文件格式不支持", 230001))
            }
            None => Err(BUSINESS_ERROR("文件格式不支持", 230001))
        };
        extracted_suffix.and_then
            (|suffix| if suffix_vec.contains(&suffix) { Ok(suffix) } else { Err(BUSINESS_ERROR("文件格式不支持", 230001)) })

    }
}


#[get("/data_card")]
async fn get_data_card(req: HttpRequest, param: web::Query<OffsetSearch>) -> WebResult<HttpResponse> {
    let param = param.into_inner();
    validate!(param);
    let (_, relate_room) = UserPo::current_user_info(&req)?;
    let data = OwnerBasicInfoPo::by_room_number_flow(relate_room.as_ref(), param.offset, param.limit)?;
    result_success!(data)
}

