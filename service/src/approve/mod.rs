mod value;
use crate::approve::value::UserCreateValue;
use crate::user;
use common::data_result::{AppError, AppResult};
use common::db_config::auto_trait::AutoOperation;
use common::db_config::db_get_connection;
use common::error::APPROVE_STATE_ERROR;
use diesel::{Connection, SaveChangesDsl};
use log::info;
use repository::approve::{ApprovePo, ApproveState, ApproveUpdatePo};

pub fn change_state(approve_insert_po: ApproveUpdatePo) -> AppResult<ApprovePo> {
    let conn = &mut db_get_connection();
    let result = conn.transaction::<_, AppError, _>(|conn|{
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
                let value = serde_json::from_value::<UserCreateValue>(result.approve_data.clone())?;
                let room_number = value.binding_room_number.clone();
                //创建用户
                info!("create_user by : {:?}", value);
                let _ = user::create_account(value.to_insert_po(), room_number, conn)?;
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