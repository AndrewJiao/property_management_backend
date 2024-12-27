use crate::const_value::SETTINGS;
use crate::data_result::AppResult;
use crate::tools;
use hmac::{Hmac, Mac};
use jwt::{AlgorithmType, Header, SignWithKey,  Token, VerifyWithKey};
use lazy_static::lazy_static;
use log::info;
use serde::{Deserialize, Serialize};
use sha2::{Sha384};
use crate::error::NO_AUTH;

pub const JWT_TOKEN_KEY: &str = "AuthorizationToken";

//定义一个jwt载荷对象
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppJwtToken {
    pub key_id: String,
    pub exp: i64,
    pub jti: String,
    pub account_id: String,
    pub account_info: AccountInfo,
}
#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct AccountInfo {
    pub account_id: String,
    pub name: String,
    pub account: String,
    pub role_type: String,
}
impl AppJwtToken {
    pub fn create_token_str<T>(param: T) -> AppResult<String>
    where
        T: JwtTokenInfoTrait,
    {
        let info = param.create_info();
        let token_obj = AppJwtToken {
            key_id: "".to_string(),
            exp: tools::time::nexted_time_stamp(SETTINGS.app_config.jwt_expire_time),
            jti: tools::id::generate_uuid_v7(),
            account_id: info.account_id.clone(),
            account_info: info,
        };
        let header = Header {
            algorithm: AlgorithmType::Hs384,
            ..Default::default()
        };
        let key = get_sec_key();
        let jwt_token_str = Token::new(header, token_obj).sign_with_key(&key)?.as_str().to_string();
        info!("jwt_token_str:{:?}", jwt_token_str);
        Ok(jwt_token_str)
    }


    ///
    /// 1.解析token是否有效
    /// 2.判断是否过期
    ///
    pub fn verify_token_str(token_str: &str) -> AppResult<()> {
        let key = get_sec_key();
        let token: Token<Header, AppJwtToken, _> = token_str.verify_with_key(&key)?;
        let claims = token.claims();
        claims.is_out_off_time()?;
        info!("claims verify success:{:?}", claims);
        Ok(())
    }

    pub fn is_out_off_time(&self) -> AppResult<()> {
        let now = tools::time::current_time_stamp();
        if now > self.exp {
            return Err(NO_AUTH());
        }
        Ok(())
    }

}


type HmacSha384 = Hmac::<Sha384>;

lazy_static!(
    static ref SEC_KEY:HmacSha384 = get_sec_key();
);
fn get_sec_key() -> HmacSha384 {
    let sec_vec = &SETTINGS.app_config.jwt_secret;
    HmacSha384::new_from_slice(sec_vec.as_bytes()).expect("HMAC can take key of any size")
}


// fn create_store() -> BTreeMapStore {
//     let sec_vec = &SETTINGS.app_config.jwt_secret;
//     let mut store = BTreeMap::new();
//     store.insert("first_key", HmacSha256::new_from_slice(sec_vec[0].as_bytes()).expect("HMAC can take key of any size"));
//     store.insert("second_key", HmacSha256::new_from_slice(sec_vec[1].as_bytes()).expect("HMAC can take key of any size"));
//     store.insert("mul_key", HmacSha256::new_from_slice(sec_vec[2].as_bytes()).expect("HMAC can take key of any size"));
//     store
// }
// lazy_static!(
//     static ref STORE: BTreeMapStore = create_store();
// );


pub trait JwtTokenInfoTrait {
    fn create_info(self) -> AccountInfo;
}





