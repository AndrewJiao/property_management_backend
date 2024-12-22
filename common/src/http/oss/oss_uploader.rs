use std::collections::HashMap;
use crate::const_value::SETTINGS;
use crate::http::oss::{StsTempSignature, CUSTOM_ENGINE};
use crate::tools::time::now_utc_date_str;
use base64::Engine;
use hmac::Mac;
use serde_json::{Map, Value};
use crate::data_result::AppResult;

/// 通过指定有效的时长（秒）生成过期时间。
/// @param seconds 有效时长（秒）。
/// @return ISO8601 时间字符串，如："2014-12-01T12:00:00.000Z"。
pub fn generate_expiration(seconds: u64) -> String {
    // 获取当前时间戳（以秒为单位）
    let now = chrono::Utc::now().timestamp();
    // 计算过期时间的时间戳
    let expiration_time = now + seconds as i64;
    // 将时间戳转换为DateTime对象，并格式化为ISO8601格式
    let expiration_datetime = chrono::NaiveDateTime::from_timestamp(expiration_time, 0);
    // 定义时区
    let expiration_utc = chrono::DateTime::<chrono::Utc>::from_utc(expiration_datetime, chrono::Utc);
    // 定义日期时间格式，例如2023-12-03T13:00:00.000Z
    let formatted_date = expiration_utc.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();
    // 输出结果
    formatted_date
}

pub fn get_post_signature_for_oss_upload(sts_data: StsTempSignature) -> AppResult<HashMap<String, String>> {
    let config = &SETTINGS.aliyun_oss_config;
    let access_key_id = sts_data.credentials.access_key_id;
    let access_key_secret = sts_data.credentials.access_key_secret;
    let security_token = sts_data.credentials.security_token;
    let region = &config.region;
    //获取x-oss-credential里的date，当前日期，格式为yyyyMMdd
    let date = now_utc_date_str("%Y%m%d");
    let x_oss_date = now_utc_date_str("%Y%m%dT%H%M%SZ");
    //创建policy
    let x_oss_credential = format!("{access_key_id}/{date}/{region}/oss/aliyun_v4_request");
    let upload_dir = "dir";

    let mut policy = Map::new();
    policy.insert("expiration".to_string(), Value::String(generate_expiration(3600)));
    let mut condition: Vec<Value> = Vec::new();
    condition.push_condition_str("bucket", security_token.as_str());
    condition.push_condition_str("x-oss-security-token", security_token.as_str());
    condition.push_condition_str("x-oss-signature-version", "OSS4-HMAC-SHA256");
    condition.push_condition_str("x-oss-credential", x_oss_credential.as_str());
    condition.push_condition_str("x-oss-date", x_oss_date.as_str());
    condition.push_condition_vec(vec![
        Value::String("content-length-range".to_string()),
        Value::Number(1.into()),
        Value::Number(10240000.into()),
    ]);
    condition.push_condition_vec(vec![
        Value::String("eq".to_string()),
        Value::String("$success_action_status".to_string()),
        Value::String("200".to_string()),
    ]);
    condition.push_condition_vec(vec![
        Value::String("starts-with".to_string()),
        Value::String("$key".to_string()),
        Value::String(upload_dir.to_string()),
    ]);
    policy.insert("conditions".to_string(), Value::Array(condition));
    let json_policy = serde_json::to_string(&policy).unwrap();
    // 步骤2：构造待签名字符串（StringToSign）。
    let string_to_sign = CUSTOM_ENGINE.encode(json_policy.as_bytes());
    // 步骤3：计算SigningKey。
    let date_key = date.hmac_sha256(&format!("aliyun_v4{access_key_secret}").as_bytes().to_vec());
    let date_region_key = region.hmac_sha256(&date_key);
    let date_region_service_key = "oss".to_string().hmac_sha256(&date_region_key);
    let signing_key = "aliyun_v4_request".to_string().hmac_sha256(&date_region_service_key);

    // 步骤4：计算Signature
    let result = string_to_sign.hmac_sha256(&signing_key);
    let digest = md5::compute(result);
    println!("{:x}", digest);
    let signature = format!("{:x}", digest);

    let mut response = std::collections::HashMap::new();
    // 将数据添加到 map 中
    response.insert("version".to_string(), "OSS4-HMAC-SHA256".to_string());
    // 这里是易错点，不能直接传policy，需要做一下Base64编码
    response.insert("policy".to_string(), string_to_sign);
    response.insert("credential".to_string(), x_oss_credential);
    response.insert("ossdate".to_string(), x_oss_date);
    response.insert("signature".to_string(), signature);
    response.insert("token".to_string(), security_token);
    response.insert("dir".to_string(), upload_dir.to_string());
    response.insert("host".to_string(), config.oss_host.clone());
    Ok(response)
}
trait Encoder {
    fn hmac_sha256(&self, key: &Vec<u8>) -> Vec<u8>;
}
type HmacSha256 = hmac::Hmac<sha2::Sha256>;
impl Encoder for String {
    fn hmac_sha256(&self, key: &Vec<u8>) -> Vec<u8> {
        // 初始化HMAC密钥规格，指定算法为HMAC-SHA256并使用提供的密钥。
        let mut mac = HmacSha256::new_from_slice(key)
            .expect("HMAC can take key of any size");
        mac.update(self.as_bytes());
        mac.finalize().into_bytes().to_vec()
    }
}

trait PushCondition {
    fn push_condition_str(&mut self, key: &str, value: &str);
    fn push_condition_vec(&mut self, vec: Vec<Value>);
}
impl PushCondition for Vec<Value> {
    fn push_condition_str(&mut self, key: &str, value: &str) {
        let mut map = Map::new();
        map.insert(key.to_string(), Value::String(value.to_string()));
        self.push(Value::Object(map));
    }

    fn push_condition_vec(&mut self, vec: Vec<Value>) {
        self.push(Value::Array(vec));
    }
}