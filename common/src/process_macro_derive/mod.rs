#[macro_export]
macro_rules! result_success {
    ($data:expr, $paginate_result:expr) => {
      Ok(actix_web::HttpResponse::Ok().json(common::data_result::AppDataResult {
            data: $data,
            code: 200,
            message: "success".to_string(),
            time_stamp: chrono::Utc::now(),
            paginate_result: $paginate_result,
        }))
    };
    ($data:expr) => {
        result_success!($data, None)
    };
    () => {
        result_success!(())
    };
}
