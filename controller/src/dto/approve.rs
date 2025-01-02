use crate::dto::{ToInsertPO, ToUpdatePO};
use common::const_value::SYSTEM;
use repository::approve::{ApproveInsertPo, ApprovePo, ApproveState, ApproveType, ApproveUpdatePo};
use serde::{Deserialize, Serialize};
use validator::Validate;
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApproveResultDto {
    pub id: i64,
    pub order_no: String,
    pub approve_state: ApproveState,
    pub approve_state_desc: String,
    pub approve_type: ApproveType,
    pub approve_type_desc: String,
    pub approve_data: serde_json::Value,
    pub comment: Option<String>,
    pub create_by: String,
    pub update_by: String,
    pub create_time: chrono::NaiveDateTime,
    pub update_time: chrono::NaiveDateTime,
}
impl From<ApprovePo> for ApproveResultDto{
    fn from(value: ApprovePo) -> Self {
        ApproveResultDto{
            id: value.id,
            order_no: value.order_no,
            approve_state: value.approve_state,
            approve_state_desc: value.approve_state.to_string(),
            approve_type: value.approve_type,
            approve_type_desc: value.approve_type.to_string(),
            approve_data: value.approve_data,
            comment: value.comment,
            create_by: value.create_by,
            update_by: value.update_by,
            create_time: value.create_time,
            update_time: value.update_time,
        }
    }
}


#[derive(Serialize, Deserialize, Validate, Default)]
#[serde(rename_all = "camelCase")]
pub struct ApproveSearchDto {
    #[validate(length(min = 1, max = 100))]
    pub order_no: Option<String>,
    pub approve_state: Option<ApproveState>,
    pub approve_type: Option<ApproveType>,
    pub create_time_star: Option<chrono::NaiveDateTime>,
    pub create_time_end: Option<chrono::NaiveDateTime>,
}

#[derive(Deserialize, Serialize,Validate)]
#[serde(rename_all = "camelCase")]
pub struct ApproveCreateDto {
    pub approve_type: ApproveType,
    pub approve_data: serde_json::Value,
    #[validate(length(max = 1000))]
    pub comment: Option<String>,
}
impl ToInsertPO for ApproveCreateDto {
    type PO<'a> = ApproveInsertPo<'a>;

    fn to_insert_po(&self) -> Self::PO<'_> {
        ApproveInsertPo {
            order_no: String::new(),
            approve_state: Default::default(),
            approve_type: self.approve_type,
            approve_data: &self.approve_data,
            comment: self.comment.as_deref(),
            create_by: SYSTEM,
            update_by: SYSTEM,
            create_time: None,
            update_time: None,
            is_delete: false,
        }
    }
}

#[derive(Deserialize,Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum ApproveAction {
    Approve,
    Reject,
}


#[derive(Deserialize,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApproveActionDto {
    pub approve_state: ApproveState,
}
impl ToUpdatePO for ApproveActionDto {
    type PO<'a> = ApproveUpdatePo<'a>;

    fn to_update_po(&self, id: i64) -> Self::PO<'_> {
        ApproveUpdatePo {
            id,
            approve_state: Some(self.approve_state),
            comment: None,
            update_by: Some(SYSTEM),
            update_time: None,
            is_delete: None,
        }
    }
}

