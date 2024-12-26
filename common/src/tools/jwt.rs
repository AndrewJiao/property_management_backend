use crate::const_value::SETTINGS;
use crate::data_result::AppResult;
use crate::tools;
use hmac::{Hmac, Mac};
use jwt::{SignWithStore, VerifyWithStore};
use lazy_static::lazy_static;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::collections::BTreeMap;

pub const JWT_TOKEN_KEY: &str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9";

//定义一个jwt载荷对象
#[derive(Debug, Serialize, Deserialize)]
pub struct AppJwtToken<'a> {
    pub key_id:String,
    pub exp: i64,
    pub jti: String,
    pub account_id: &'a str,
    pub account_info: AccountInfo,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountInfo {
    pub account_id: String,
    pub name: String,
    pub account: String,
    pub role_type: String,
}
impl AppJwtToken<'_> {
    pub fn create_token_str<T>(param: T) -> AppResult<String>
    where
        T: JwtTokenInfoTrait,
    {
        let info = param.create_info();
        let token_obj = AppJwtToken {
            key_id: "".to_string(),
            exp: tools::time::nexted_time_stamp(SETTINGS.app_config.jwt_expire_time),
            jti: tools::id::generate_uuid_v7(),
            account_id: &info.account_id,
            account_info:info,
        };
        let range_key = thread_rng().gen_range(0..=2);
        let sec_key = STORE.keys().nth(range_key).expect("key not found");
        let jwt_token_str = (*sec_key, token_obj).sign_with_store(&STORE)?;
        println!("jwt_token_str:{jwt_token_str} ,token_key{sec_key}");
        Ok(jwt_token_str)
    }

    pub fn verify_token_str(token_str: &str) -> AppResult<AppJwtToken> {
        let token: AppJwtToken = token_str.verify_with_store(&STORE)?;
        Ok(token)
    }
}

type HmacSha256 = Hmac::<Sha256>;
fn create_store() -> BTreeMap<&'static str, Hmac<Sha256>> {
    let sec_vec = &SETTINGS.app_config.jwt_secret;
    let mut store = BTreeMap::new();
    store.insert("first_key", HmacSha256::new_from_slice(sec_vec[0].as_bytes()).expect("HMAC can take key of any size"));
    store.insert("second_key", HmacSha256::new_from_slice(sec_vec[1].as_bytes()).expect("HMAC can take key of any size"));
    store.insert("mul_key", HmacSha256::new_from_slice(sec_vec[2].as_bytes()).expect("HMAC can take key of any size"));
    store
}
lazy_static!(
    static ref STORE: BTreeMap<String, Hmac<Sha256>> = create_store();
);


pub trait JwtTokenInfoTrait {
    fn create_info(self) -> AccountInfo;
}





