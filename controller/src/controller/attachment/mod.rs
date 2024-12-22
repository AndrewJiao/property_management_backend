use actix_web::web::scope;
use actix_web::{get, web, HttpResponse};
use common::const_value::SETTINGS;
use common::data_result::WebResult;
use common::http::oss;
use common::http::oss::*;
use common::result_success;
use serde::{Deserialize, Serialize};
use service::attachment;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(scope("/attachment")
        .service(init)
    );
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UploadTokenDto {
    // yourRegion填写Bucket所在地域。以华东1（杭州）为例，yourRegion填写为oss-cn-hangzhou。
    region: String,
    // 从STS服务获取的临时访问密钥（AccessKey ID和AccessKey Secret）。
    access_key_id: String,
    access_key_secret: String,
    // 从STS服务获取的安全令牌（SecurityToken）。
    sts_token: String,
    // 填写Bucket名称，例如examplebucket。
    bucket: String,
    oss_host: String,
}
impl From<StsTempSignature> for UploadTokenDto{
    fn from(value: StsTempSignature) -> Self {
        let config = &SETTINGS.aliyun_oss_config;
        let region = config.region.clone();
        let bucket = config.bucket.clone();
        let oss_host = config.oss_host.clone();
        UploadTokenDto {
            region,
            oss_host,
            access_key_id: value.credentials.access_key_id,
            access_key_secret: value.credentials.access_key_secret,
            sts_token: value.credentials.security_token,
            bucket,
        }
    }
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AttachmentInitDto{
    file_name: String,
}

#[get("/init")]
async fn init(param: web::Query<AttachmentInitDto>) -> WebResult<HttpResponse> {
    let token = get_temp_signature().await?;
    attachment::init_attachment_token(&param.file_name);
    let result = oss::oss_uploader::get_post_signature_for_oss_upload(token)?;
    result_success!(result)
}


