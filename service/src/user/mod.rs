use base64::Engine;
use common::const_value::SETTINGS;
use common::data_result::AppResult;
use common::db_config::auto_trait::AutoOperation;
use common::error::USER_PASSWORD_ERROR;
use hmac::Mac;
use repository::user::{UserInsertPo, UserPo};
use sha2::Sha256;

pub fn create_account(mut po: UserInsertPo) -> AppResult<UserPo> {
    let uuid = uuid_v7::gen_uuid_v7().to_string();
    po.account_id = Some(uuid);
    if let Some(encode_password ) = po.parse_password(){
        po.password = encode_password;
    }else{
        return Err(USER_PASSWORD_ERROR());
    }
    po.create_time().save()
}



///
/// 如果是加密就生成解密后的字符串，如果是解密就生成加密后的字符串
///
trait PasswordCoder {
    type Result;
    fn parse_password(&self) -> Option<String>;
}

type HmacSha256 = hmac::Hmac<Sha256>;

impl<T: Password> PasswordCoder for T{
    type Result = String;
    fn parse_password(&self) -> Option<String> {
        let password = self.raw_password();
        let sec_key = &SETTINGS.app_config.password_sec_key;

        HmacSha256::new_from_slice(&sec_key.as_bytes())
            .map(|mut mac| {
                mac.update(password.as_bytes());
                let bytes = mac.finalize().into_bytes();
                base64::engine::general_purpose::STANDARD.encode(bytes)
            }).ok()
    }
}


pub trait Password {
    fn raw_password(&self) -> &str;
}

impl Password for &str {
    fn raw_password(&self) -> &str { &self }
}

impl Password for UserInsertPo<'_>{
    fn raw_password(&self) -> &str { &self.password }
}