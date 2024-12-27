use actix_web::cookie::Cookie;
use crate::const_value::SETTINGS;
use crate::data_result::AppResult;
use crate::tools;
use hmac::{Hmac, Mac};
use jwt::{AlgorithmType, Header, SignWithKey, Token, VerifyWithKey};
use lazy_static::lazy_static;
use log::info;
use serde::{Deserialize, Serialize};
use sha2::{Sha384};
use crate::error::NO_AUTH;
use crate::tools::jwt::TokenOperation::Fail;

pub const JWT_TOKEN_KEY: &str = "AuthorizationToken";
pub fn create_wt_token_cookie(token_str: &str) -> Cookie {
    Cookie::build(JWT_TOKEN_KEY, token_str).secure(true).http_only(true).finish()
}

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

#[derive(PartialEq,Eq)]
pub(crate) enum TokenOperation {
    Success,
    Fail,
    SuccessAndRenew(String),
}
impl Default for TokenOperation{
    fn default() -> Self {
        Fail
    }
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
        let jwt_token_str = token_obj.sign_to_string()?;
        info!("jwt_token_str:{:?}", jwt_token_str);
        Ok(jwt_token_str)
    }


    ///
    /// 1.解析token是否有效
    /// 2.判断是否过期
    ///
    pub(crate) fn verify_token_str(token_str: &str) -> TokenOperation {
        let key = get_sec_key();

        //验证jwt
        let token = if let Ok(token) = VerifyWithKey::<Token<Header, AppJwtToken, _>>::verify_with_key(token_str, &key) {
            token
        }else{
            return Fail;
        };
        //验证超时
        let claims = token.claims();
        if let Err(_) = claims.is_out_off_time(){
            return Fail;
        }
        info!("claims verify success:{:?}", claims);
        if let Some(new_token) = claims.try_renew(){
            return TokenOperation::SuccessAndRenew(new_token.sign_to_string().unwrap_or_default());
        }
        TokenOperation::Success
    }




    pub fn is_out_off_time(&self) -> AppResult<()> {
        let now = tools::time::current_time_stamp();
        if now > self.exp {
            return Err(NO_AUTH());
        }
        Ok(())
    }

    pub fn try_renew(&self)->Option<Self> {
        let now = tools::time::current_time_stamp();
        let renew_time = SETTINGS.app_config.jwt_renew_time;
        if self.exp - now < renew_time {
            let new_token = self.clone().new(self.clone());
            Some(new_token)
        } else {
            None
        }

    }

    fn sign_to_string(&self) -> AppResult<String> {
        let header = Header {
            algorithm: AlgorithmType::Hs384,
            ..Default::default()
        };
        let key = get_sec_key();
        let jwt_token_str = Token::new(header, self).sign_with_key(&key)?.as_str().to_string();
        Ok(jwt_token_str)
    }
    fn new(self, old_token: Self) ->Self{
        Self{
            key_id: old_token.key_id,
            exp: tools::time::nexted_time_stamp(SETTINGS.app_config.jwt_expire_time),
            jti: tools::id::generate_uuid_v7(),
            account_id: old_token.account_id,
            account_info: old_token.account_info,
        }


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

pub trait JwtTokenInfoTrait {
    fn create_info(self) -> AccountInfo;
}





