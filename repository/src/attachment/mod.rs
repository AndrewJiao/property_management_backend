use crate::common_type;
use crate::schema::basic::t_attachment;
use crate::schema::basic::t_attachment::*;
use common::data_result::AppResult;
use common::db_config::{db_get_connection, Conn};
use diesel::pg::Pg;
use diesel::{AsChangeset, ExpressionMethods, Identifiable, Insertable, QueryDsl, Queryable, RunQueryDsl, Selectable, SelectableHelper};
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};
use management_macro::AutoOperation;

#[derive(Queryable, Selectable, Deserialize, Serialize, Debug)]
#[diesel(table_name = t_attachment)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(rename_all = "camelCase")]
pub struct AttachmentPo {
    pub id: i64,
    pub attachment_id: String,
    pub attachment_file_name: Option<String>,
    pub oss_file_name: Option<String>,
    pub comment: Option<String>,
    pub status: AttachmentStatus,
    pub create_by: String,
    pub update_by: String,
    pub create_time: chrono::NaiveDateTime,
    pub update_time: chrono::NaiveDateTime,
    pub is_delete: bool,
}

common_type!();

type BoxedQuery<'a> = t_attachment::BoxedQuery<'a, Pg, crate::SqlType<AttachmentPo>>;
impl AttachmentPo {
    pub fn all<'a>() -> BoxedQuery<'a> {
        table.select(AttachmentPo::as_select()).into_boxed()
    }

    pub fn by_id_list(p_attachment_id: &Vec<&str>) -> AppResult<Vec<AttachmentPo>> {
        let result = Self::all().filter(attachment_id.eq_any(p_attachment_id)).get_results(&mut db_get_connection())?;
        Ok(result)
    }
}

#[derive(Deserialize, Serialize, DbEnum,Debug)]
#[ExistingTypePath = "crate::schema::basic::sql_types::AttachmentState"]
#[serde(rename_all = "PascalCase")]
pub enum AttachmentStatus {
    Init,
    Done,
}


#[derive(Insertable, AutoOperation, Serialize)]
#[diesel(table_name = t_attachment)]
pub struct AttachmentInsertPo<'a> {
    pub attachment_id: &'a str,
    pub attachment_file_name: Option<&'a str>,
    pub oss_file_name: Option<&'a str>,
    pub comment: Option<&'a str>,
    pub status: AttachmentStatus,
    pub create_by: &'a str,
    pub update_by: &'a str,
    pub create_time: Option<chrono::NaiveDateTime>,
    pub update_time: Option<chrono::NaiveDateTime>,
    pub is_delete: bool,
}

impl AttachmentInsertPo<'_> {
    pub fn save(&self, conn: &mut Conn) -> AppResult<AttachmentPo> {
        let result = diesel::insert_into(table).values(self).get_result(conn)?;
        Ok(result)
    }

    pub fn uuid_v7() -> String {
        uuid_v7::gen_uuid_v7().to_string()
    }
}

#[derive(Identifiable, AutoOperation, Serialize, AsChangeset)]
#[diesel(table_name = t_attachment)]
#[serde(rename_all = "camelCase")]
pub struct AttachmentUpdatePo<'a> {
    pub id: i64,
    pub attachment_file_name: Option<&'a str>,
    pub oss_file_name: Option<&'a str>,
    pub comment: Option<&'a str>,
    pub status: Option<&'a AttachmentStatus>,
    pub create_by: Option<&'a str>,
    pub update_by: Option<&'a str>,
    pub create_time: Option<chrono::NaiveDateTime>,
    pub update_time: Option<chrono::NaiveDateTime>,
    pub is_delete: Option<bool>,
}

