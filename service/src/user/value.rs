use serde::Deserialize;

///
/// 微信sns验证json返回
/// ```json
/// {"session_key":"+w42ySXIVwuKkOnsjGbIZw==","openid":"omqVV7Kdkw-ymI0iTpDwfgwyx_1Q"}
///
#[derive(Deserialize, Debug)]
pub struct WeChartSns {
    pub session_key: String,
    pub openid: String,
}

pub enum LoginType{
    Password(String, String),
    WeChartCode(String)
}
