pub mod oss_uploader;
use crate::const_value::SETTINGS;
use crate::http::AppHttpClient;
use crate::tools::time::now_utc_date_str;
use base64::engine::general_purpose;
use base64::{alphabet, engine, Engine};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::data_result::AppResult;

pub async fn get_temp_signature() -> AppResult<StsTempSignature> {
    let config = &SETTINGS.aliyun_oss_config;
    let mut query_param = build_common_query_param();
    build_signature_v2("GET", "/", &mut query_param);
    let host = &config.sts_host;
    let signature_obj = AppHttpClient::get(format!("https://{host}").as_str())
        .query(&query_param)
        .send().await?
        .json::<StsTempSignature>().await?;
    Ok(signature_obj)
}

const ALGORITHM: &str = "HMAC-SHA1";


#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct StsParam<'a> {
    role_arn: &'a str,
    role_session_name: &'a str,
}

fn build_common_query_param() -> Vec<(String, String)> {
    let config = &SETTINGS.aliyun_oss_config;
    vec![
        ("Action".to_string(), "AssumeRole".to_string()),
        ("Version".to_string(), "2015-04-01".to_string()),
        ("AccessKeyId".to_string(), config.access_key_id.to_string()),
        ("SignatureNonce".to_string(), Uuid::new_v4().to_string()),
        ("Timestamp".to_string(), now_utc_date_str("%Y-%m-%dT%H:%M:%SZ")),
        ("SignatureMethod".to_string(), ALGORITHM.to_string()),
        ("SignatureVersion".to_string(), "1.0".to_string()),
        ("RoleArn".to_string(), config.sts_role_arn.to_string()),
        ("RoleSessionName".to_string(), "temp-user".to_string()),
        ("Format".to_string(), "JSON".to_string()),
    ]
}
impl SpecialEncoder for Vec<(String, String)> {
    fn special_encode(&self) -> String {
        self.iter().map(|(k, v)| {
            format!("{}{}{}", k.as_str().special_encode(), "=", v.as_str().special_encode())
        }).collect::<Vec<String>>().join("&")
    }
}

trait SpecialEncoder {
    fn special_encode(&self) -> String;
}
impl SpecialEncoder for &str {
    fn special_encode(&self) -> String {
        special_replace(urlencoding::encode(self).to_string())
    }
}

fn special_replace(value: String) -> String {
    value.chars()
        .collect::<String>()
        .replace("+", "%20")
        .replace("*", "%2A")
        .replace("~", "%7E")
}


//使用 UTF-8 字符集按照RFC3986规则编码

const CUSTOM_ENGINE: engine::GeneralPurpose = engine::GeneralPurpose::new(&alphabet::STANDARD, general_purpose::PAD);
fn build_signature_v2(method: &str, uri: &str, query_param: &mut Vec<(String, String)>) {
    let config = &SETTINGS.aliyun_oss_config;
    query_param.sort();

    let query_str = query_param.special_encode().as_str().special_encode();
    let string_to_sign = format!("{method}&{uri}&{query_str}", method = method, uri = uri.special_encode());
    let key = format!("{}&", config.access_key_secret);
    let signature = CUSTOM_ENGINE.encode(&hmac_sha1::hmac_sha1(key.as_bytes(), string_to_sign.as_bytes()));
    query_param.insert(0, ("Signature".to_string(), signature));
}

///
///
/// eg.
/// ```json
/// {
///     "RequestId": "FA4A39D7-27F9-59CE-A576-285AB075C90F",
///     "AssumedRoleUser": {
///         "Arn": "acs:ram::1957570988142134:role/temp-role-backend/temp-user",
///         "AssumedRoleId": "300957547127996080:temp-user"
///     },
///     "Credentials": {
///         "SecurityToken": "CAISxAJ1q6Ft5B2yfSjIr5eMAfb2nY8Z8YSGTEWJqkM2Z/1tn7Xdrzz2IHhMdXRvBOwZt/4wlWFW7vYalqBvRppdAFHfYNEoHGK0CNX7MeT7oMWQweEuqv/MQBq+aXPS2MvVfJ+KLrf0ceusbFbpjzJ6xaCAGxypQ12iN+/i6/clFKN1ODO1dj1bHtxbCxJ/ocsBTxvrOO2qLwThjxi7biMqmHIl2D4gtPvnk53Ms0aE1Qam8IJP+dSteKrDRtJ3IZJyX+2y2OFLbafb2EZSkUMSrPYr0/EYpm6d4YDMXQEBuw //L+fR6ph1Kwt0I6IzEK1JqvTsS0nrSGw3Qu8dojY63oE9O0y3LOjISzvZAPY+YJ1w0zqvjpBxcGojYtiU1U1tKDlwHxhBYJEqUSQ7YkcPQRKBIcLJtS6MDE7NJKbtS29iM8Ydpz0agAF+H6i8BGQqk7+cwwgP6Hc4X/EDQfxJMnRD33y2hbGGLzNlZssiu4KiAhYQTzhrAW/nsUvZzM3UV7EUXLh4KEiojjYVr0jaLbFBWn5kuKU58JARjzLgRX91NV0CKWJZbCTx00VDG3B9wqv4ywsAw93KaS/00uTLOgTTKccy5KZS8iAA",
///         "AccessKeyId": "STS.NT9JLBpP8FFmNc8NCckrAppvM",
///         "AccessKeySecret": "AWfxteqqfXY6VH46bAzd7TXxzBarqn3yKibrewxMsN16",
///         "Expiration": "2024-12-20T03:35:52Z"
///     }
/// }
/// ```
///

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StsTempSignature {
    pub request_id: String,
    pub assumed_role_user: StsTempUser,
    pub credentials: Credentials,

}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Credentials {
    pub security_token: String,
    pub access_key_id: String,
    pub access_key_secret: String,
    pub expiration: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StsTempUser {
    pub arn: String,
    pub assumed_role_id: String,
}

