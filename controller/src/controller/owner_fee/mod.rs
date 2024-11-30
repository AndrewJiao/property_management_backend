use crate::dto::owner_fee::OwnerFeeDetailSearchDto;
use actix_web::web::scope;
use actix_web::{get, web, HttpResponse};
use common::data_result::PaginateSearch;
use common::db_config::db_get_connection;
use common::error::AppResult;
use common::result_success;
use diesel::query_dsl::methods::OrderDsl;
use diesel::{ExpressionMethods};
use repository::component::page::Paginate;
use repository::owner_fee::OwnerFeeDetailPo;
use repository::schema::basic::t_owner_fee_detail::*;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(scope("/owner_fee")
        .service(get_data)
    );
}

#[get("/data")]
async fn get_data(param: web::Query<PaginateSearch>) -> AppResult<HttpResponse> {
    let search_param: OwnerFeeDetailSearchDto = param.convert_param()?;
    let statement = OwnerFeeDetailPo::search_by_param(
        search_param.stream_id.as_deref(),
        search_param.room_number.as_deref(),
        search_param.detail_type.as_ref(),
        search_param.create_time_star.as_ref(), search_param.create_time_end.as_ref(),
        search_param.update_time_star.as_ref(), search_param.update_time_end.as_ref(),
    );

    let (result, total) =
        OrderDsl::order(statement, create_time.desc())
            .paginate(param.current_page())
            .per_page(param.limit())
            .load_and_count_pages::<OwnerFeeDetailPo>(&mut db_get_connection())?;
    result_success!(result, param.produce_page_result(total))
}
