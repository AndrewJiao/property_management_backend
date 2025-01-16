use crate::user::value::{LoginType, WeChartSns};
use base64::Engine;
use common::const_value::SETTINGS;
use common::data_result::{AppError, AppResult};
use common::db_config::auto_trait::AutoOperation;
use common::db_config::{db_get_connection, Conn};
use common::error::{ACCOUNT_EXIST, ACCOUNT_NOT_SUPPORT_FAST_LOGIN, ROOM_HAS_BEEN_BIND, ROOM_IS_NOT_EXIST, USER_PASSWORD_ERROR, WE_CHART_SNS_ERROR};
use common::http::AppHttpClient;
use common::tools::jwt::AppJwtToken;
use common::tools::password::generate_password;
use diesel::{Connection, SaveChangesDsl};
use hmac::Mac;
use log::info;
use repository::owner_info::OwnerBasicInfoPo;
use repository::user::relate::UserRelateRoomPo;
use repository::user::{RoleType, UserInsertPo, UserPo, UserUpdatePo};
use sha2::Sha256;
use repository::user::fast_login::UserFastLoginPo;

pub mod value;

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
pub fn verify_password(account: &String, password: String) -> AppResult<UserPo> {
    let user_po = UserPo::by_account(account)?;
    let sec_password = password.as_str().parse_password().unwrap_or_default();
    if user_po.password ==  sec_password{
        Ok(user_po)
    } else {
        Err(USER_PASSWORD_ERROR())
    }
}

pub fn create_account(mut po: UserInsertPo, room_number: Option<Vec<String>>,conn:&mut Conn) -> Result {
    let uuid = uuid_v7::gen_uuid_v7().to_string();
    po.account_id = Some(uuid);
    if let Some(encode_password) = po.parse_password() {
        po.password = encode_password;
    } else {
        return Err(USER_PASSWORD_ERROR());
    }
    //事务

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


///
/// 修改密码
/// 修改前先校验密码正确性
///
pub fn change_password(account: String, old_password: String, new_passowrd: String) -> AppResult<UserPo> {
    verify_password(&account, old_password)?;
    let result = UserUpdatePo::change_password(&account, new_passowrd.as_str().parse_password().unwrap_or_default(), &mut db_get_connection())?;
    Ok(result)
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

pub fn valid_room_number(param: &Option<Vec<String>>) -> AppResult<()> {
    if let Some(ref room_number) = param {
        for room_number in room_number {
            if OwnerBasicInfoPo::by_room_number(room_number, &mut db_get_connection()).is_err() {
                return Err(ROOM_IS_NOT_EXIST());
            }
        }
    }
    Ok(())
}

///
/// 校验是否有绑定
///
pub fn valid_has_being_bind(param: &Option<Vec<String>>)->AppResult<()> {
    if let Some(ref room_number) = param {
        for room_number in room_number {
            if UserRelateRoomPo::by_room_number(room_number).is_ok() {
                return Err(ROOM_HAS_BEEN_BIND());
            }
        }
    }
    Ok(())
}


///
/// 验证密码
/// 设置jwtToken
///
pub async fn login(auth: LoginType) -> AppResult<(UserPo, String)> {
    let user_po = match auth {
        LoginType::Password(p_account, p_password) => {
            verify_password(&p_account, p_password)?
        }
        LoginType::WeChartCode(code,fast_login_flag) => {
            let we_chart_sns = we_chart_auth(&code).await?;
            let user_po = UserPo::by_relate_user_id(&we_chart_sns.session_key)?;
            let flag = !UserFastLoginPo::is_fast_login(&user_po.account_id);
            info!("fast_login_flag = {} flag = {}",fast_login_flag,flag);
            if fast_login_flag && flag {
                return Err(ACCOUNT_NOT_SUPPORT_FAST_LOGIN());
            }
            user_po
        }
    };
    //用户登录之后做个标记，以后可以尝试快速登录
    UserFastLoginPo::add_user_fast_login(&user_po.account_id, &mut db_get_connection())?;

    //设置jwtToken
    let token_str = AppJwtToken::create_token_str(user_po.clone())?;
    Ok((user_po,token_str))
}

///
/// we_chart授权
///
pub async fn we_chart_auth(code: &String) -> AppResult<WeChartSns> {
    let config = &SETTINGS.we_chart_config;
    let host = &config.host;
    let app_id = &config.app_id;
    let app_secret = &config.app_secret;
    let url = format!("https://{host}/sns/jscode2session");
    AppHttpClient::get(&url)
        .query(&[("appid", app_id), ("secret", app_secret), ("js_code", code), ("grant_type", &"authorization_code".to_string())])
        .send().await?
        .json().await
        .map_err(|_| WE_CHART_SNS_ERROR())
}

pub async fn register(nick_name: &String, code: &String) ->AppResult<UserPo>{
    let sns = we_chart_auth(code).await?;
    //校验用户是否已存在
    let po1 = UserPo::by_relate_user_id(&sns.session_key).ok();
    let po2 = UserPo::by_account(nick_name).ok();
    if po1.is_some() || po2.is_some() {
        return Err(ACCOUNT_EXIST());
    }

    let user_po = UserInsertPo {
        account_id: None,
        password: generate_password(12),
        account: nick_name,
        name: nick_name,
        role_type: RoleType::User,
        create_by: "system",
        update_by: "system",
        create_time: None,
        update_time: None,
        comment: None,
        is_delete: false,
        relate_user_id: Some(sns.session_key),
    };
    //创建一个不绑定房间的用户
    let (result, _) = create_account(user_po, None, &mut db_get_connection())?;
    Ok(result)
}