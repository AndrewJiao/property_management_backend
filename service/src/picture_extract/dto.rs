use actix::Message;
use common::data_result::AppResult;
use std::fs::File;

#[derive(Message)]
#[rtype(result = "AppResult<()>")]
pub struct ExtractSender {
    pub file: File,
    pub file_name: String
}

impl ExtractSender {
    pub fn new(file: File, file_name: String) -> Self {
        Self { file, file_name }
    }
}

