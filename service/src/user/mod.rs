use base64::Engine;
use common::const_value::SETTINGS;
use common::data_result::{AppError, AppResult};
use common::db_config::auto_trait::AutoOperation;
use common::db_config::db_get_connection;
use common::error::{DATA_HAS_EXIST, DATA_NOT_EXIST, USER_PASSWORD_ERROR};
use diesel::{Connection, SaveChangesDsl};
use hmac::Mac;
use repository::owner_info::OwnerBasicInfoPo;
use repository::user::relate::UserRelateRoomPo;
use repository::user::{UserInsertPo, UserPo, UserUpdatePo};
use sha2::Sha256;
use common::tools::jwt::AppJwtToken;

///
/// 如果是加密就生成解密后的字符串，如果是解密就生成加密后的字符串
///
trait PasswordCoder {
    type Result;
    fn parse_password(&self) -> Option<String>;
}

type HmacSha256 = hmac::Hmac<Sha256>;

impl<T: Password> PasswordCoder for T {
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

impl Password for UserInsertPo<'_> {
    fn raw_password(&self) -> &str { &self.password }
}

type Result = AppResult<(UserPo, Option<Vec<String>>)>;

///
/// 根据长湖，查询数据并查询验证密码
///
pub fn verify_password(account: String, password: String) -> AppResult<UserPo> {
    let user_po = UserPo::by_account(&account)?;
    let sec_password = password.as_str().parse_password().unwrap_or_default();
    if user_po.password ==  sec_password{
        Ok(user_po)
    } else {
        Err(USER_PASSWORD_ERROR())
    }
}

pub fn create_account(mut po: UserInsertPo, room_number: Option<Vec<String>>) -> Result {
    let uuid = uuid_v7::gen_uuid_v7().to_string();
    po.account_id = Some(uuid);
    if let Some(encode_password) = po.parse_password() {
        po.password = encode_password;
    } else {
        return Err(USER_PASSWORD_ERROR());
    }
    //事务

    let conn = &mut db_get_connection();
    let user_po = conn.transaction::<_, AppError, _>(|conn| {
        valid_room_number(&room_number)?;
        valid_has_being_bind(&room_number)?;
        //先保存关联表
        if let Some(ref room_number) = room_number {
            let room_number_str = &room_number.iter().map(|e|e.as_str()).collect();
            UserRelateRoomPo::bind(&po.account_id.as_deref().unwrap(), room_number_str, conn)?;
        }
        let result = po.create_time().save(conn)?;
        Ok(result)
    })?;
    Ok((user_po, room_number))
}

pub fn put_data(update_po: UserUpdatePo, room_number: Option<Vec<String>>) -> Result {
    let conn = &mut db_get_connection();
    let result = conn.transaction::<_, AppError, _>(|conn| {
        valid_room_number(&room_number)?;
        valid_has_being_bind(&room_number)?;
        //先保存才能看到user的account_id
        let result = update_po
            .update_time()
            .save_changes::<UserPo>(&mut db_get_connection())?;
        if let Some(ref room_number) = room_number {
            UserRelateRoomPo::unbind(&vec![&result.account_id], conn)?;
            UserRelateRoomPo::bind(&result.account_id, &room_number.iter().map(|e|e.as_str()).collect(), &mut db_get_connection())?;
        }
        Ok(result)
    })?;
    Ok((result, room_number))
}

pub fn delete_data(id: i64) ->Result{
    let conn = &mut db_get_connection();
    let result = conn.transaction::<_, AppError, _>(|conn| {
        let result = repository::user::delete_by_id(id, conn)?;
        UserRelateRoomPo::unbind(&vec![&result.account_id], conn)?;
        Ok(result)
    })?;
    Ok((result, None))
}

fn valid_room_number(param: &Option<Vec<String>>) -> AppResult<()> {
    if let Some(ref room_number) = param {
        for room_number in room_number {
            if OwnerBasicInfoPo::by_room_number(room_number, &mut db_get_connection()).is_err() {
                return Err(DATA_NOT_EXIST());
            }
        }
    }
    Ok(())
}

///
/// 校验是否有绑定
///
fn valid_has_being_bind(param: &Option<Vec<String>>)->AppResult<()> {
    if let Some(ref room_number) = param {
        for room_number in room_number {
            if UserRelateRoomPo::by_room_number(room_number).is_ok() {
                return Err(DATA_HAS_EXIST());
            }
        }
    }
    Ok(())
}

///
/// 验证密码
/// 设置jwtToken
///
pub fn login(account: String, password: String) -> AppResult<(UserPo, String)> {
    let user_po = verify_password(account, password)?;
    //设置jwtToken
    let token_str = AppJwtToken::create_token_str(user_po.clone())?;
    Ok((user_po,token_str))
}
