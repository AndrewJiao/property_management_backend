use common::const_value::SETTINGS;
use common::http::oss::test::{build_auth_head, build_common_heads, StsParam};
use common::http::AppHttpClient;
#[actix_web::main]
async fn main() {
    do_test().await;
}

async fn do_test() {
    let config = &SETTINGS.aliyun_oss_config;
    let param:Option<StsParam> = None;
    let mut common_headers = build_common_heads(&param);
    let mut query = vec![("RoleArn".to_string(), config.sts_role_arn.clone()),("RoleSessionName".to_string(), "test".to_string())];
    let mut common_signature_headers = common_headers.iter().map(|(a, b)| (a.to_string(), b.clone())).collect::<Vec<(String, String)>>();
    let authorization = build_auth_head("GET", "/", &mut query, &mut common_signature_headers, &param);
    common_headers.push(("Authorization", authorization));
    let host = &config.sts_host;
    let result = AppHttpClient::get(format!("https://{host}").as_str())
        .headers(&common_headers)
        .query(&query)
        .send().await.unwrap()
        .text().await.unwrap();
    println!("result = {}", result);
}
