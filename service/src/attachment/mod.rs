use common::const_value::SYSTEM;
use common::db_config::auto_trait::AutoOperation;
use common::db_config::db_get_connection;
use repository::attachment::{AttachmentInsertPo, AttachmentStatus};

pub fn init_attachment_token(file_name: &str) {
    let attachment_id = AttachmentInsertPo::uuid_v7();
    let oss_file_name = format!("{OWNER_INFO}/{attachment_id}");
    let _ = AttachmentInsertPo {
        attachment_id: &attachment_id,
        attachment_file_name: Some(file_name),
        oss_file_name: Some(oss_file_name.as_str()),
        comment: None,
        status: AttachmentStatus::Init,
        create_by: SYSTEM,
        update_by: SYSTEM,
        create_time: None,
        update_time: None,
        is_delete: false,
    }.create_time().save(&mut db_get_connection());
}

const OWNER_INFO: &str = "picture_extract";
