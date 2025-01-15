mod value;

use crate::approve::value::{BindingRoomValue, ChangeRoomInfo, UserCreateValue, WeChartUserCreateValue};
use crate::user;
use crate::user::we_chart_auth;
use actix_web::HttpRequest;
use common::data_result::{AppError, AppResult};
use common::db_config::auto_trait::AutoOperation;
use common::db_config::db_get_connection;
use common::error::{APPROVE_STATE_ERROR, NO_AUTH, USER_ACCOUNT_EXIST};
use diesel::{Connection, SaveChangesDsl};
use log::{error, info};
use repository::approve::{ApprovePo, ApproveState, ApproveType, ApproveUpdatePo};
use repository::owner_info::UpdateOwnerBasicInfoPo;
use repository::user::relate::UserRelateRoomPo;
use repository::user::UserPo;

pub async fn change_state(approve_insert_po: ApproveUpdatePo<'_>, http_request: HttpRequest) -> AppResult<ApprovePo> {
    let conn = &mut db_get_connection();
    let current_user = UserPo::current_user_info(&http_request)?;
    let result = conn.transaction::<_, AppError, _>(|conn| {
        let state = approve_insert_po.approve_state.unwrap_or_default();

        let result = approve_insert_po
            .update_time()
            .save_changes::<ApprovePo>(conn)?;

        match state {
            ApproveState::Pending => {
                return Err(APPROVE_STATE_ERROR())
            }
            ApproveState::Approved => {
                //由于只有一种审批类型，所以这里不需要判断,直接生成对应的数据
                match result.approve_type {
                    //普通创建用户
                    ApproveType::CreateUser => {
                        let value = serde_json::from_value::<UserCreateValue>(result.approve_data.clone())?;
                        let room_number = value.binding_room_number.clone();
                        //创建用户
                        info!("create_user by : {:?}", value);
                        //验证账户已存在
                        let insert_po = value.to_insert_po();
                        if UserPo::by_account(&insert_po.account).is_ok() {
                            return Err(USER_ACCOUNT_EXIST());
                        }
                        let _ = user::create_account(insert_po, room_number, conn)?;
                    }
                    //微信小程序创建用户
                    ApproveType::WeChartCreateUser => {
                        let value = serde_json::from_value::<WeChartUserCreateValue>(result.approve_data.clone())?;

                        let sns_future = we_chart_auth(&value.code);
                        let sns = futures::executor::block_on(sns_future)?;
                        let room_number = value.binding_room_number.clone();
                        //验证账户已存在
                        let insert_po = value.to_insert_po(sns.session_key);
                        if UserPo::by_account(&insert_po.account).is_ok() {
                            return Err(USER_ACCOUNT_EXIST());
                        }
                        //创建用户
                        info!("create_user by : {:?}", value);
                        let _ = user::create_account(insert_po, room_number, conn)?;
                    },
                    ApproveType::BindingRooms => {
                        let (user_po, _) = current_user;
                        let value = serde_json::from_value::<BindingRoomValue>(result.approve_data.clone())?;
                        //校验room数据是否存在,校验是否已经被绑定了
                        let room_number = Some(value.binding_room_number.clone());
                        user::valid_room_number(&room_number)?;
                        user::valid_has_being_bind(&room_number)?;
                        //合并relate_room和value中的room
                        UserRelateRoomPo::bind(&user_po.account_id, &value.get_room_ref(), conn)?;
                    }
                    ApproveType::ChangeRoomInfo => {
                        let (user_po, relate_room) = current_user;
                        let value = serde_json::from_value::<ChangeRoomInfo>(result.approve_data.clone())?;
                        //判断是否有这个房间的权限
                        if relate_room.map(|e| e.contains(&value.room_number)).is_none() {
                            error!("no auth to change room info for user : {:?} in room number {:?}", &user_po.account, &value.room_number);
                            return Err(NO_AUTH());
                        }
                        UpdateOwnerBasicInfoPo::update_other_part(&value.room_number, value.other_part_info, conn)?;
                    }
                }
            }
            ApproveState::Rejected => {
                //这里不需要做任何操作
            }
        }
        Ok(result)
    })?;
    info!("approve success : {:?}", result);
    Ok(result)
}