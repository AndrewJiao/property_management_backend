#[macro_export]
macro_rules! result_success {
    ($data:expr, $paginate_result:expr) => {
      Ok(actix_web::HttpResponse::Ok().json(common::data_result::AppDataResult {
            data: $data,
            code: 200,
            message: "success".to_string(),
            time_stamp: common::tools::time::now_local_date_time_naive(),
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

#[macro_export]
macro_rules! validate {
    ($($data:expr),* $(,)?) => {
        {
            use validator::Validate;
            $(
                if let Err(e) = $data.validate() {
                    return Err(common::error::BaseError::AnyhowError(common::error::PARAM_NOT_SUPPORT()));
                }
            )*
        }
    };
}
