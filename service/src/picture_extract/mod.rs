use crate::picture_extract::dto::ExtractSender;
use actix::{Actor, Context, Handler};
use common::const_value::SETTINGS;
use common::data_result::AppResult;
use log::info;
use std::fs;
use std::fs::File;
use std::process::{Command, Stdio};

pub mod dto;
pub mod command;



pub struct PictureExtractor;

impl Actor for PictureExtractor{
    type Context = Context<Self>;
}
impl Handler<ExtractSender> for PictureExtractor{
    type Result = AppResult<()>;

    fn handle(&mut self, msg: ExtractSender, _: &mut Self::Context) -> Self::Result {
        let dir = &SETTINGS.attachment_config.temp_output_dir;
        if !fs::exists(dir)? {
            info!("no temp dir so create new config_dir={dir}");
            fs::create_dir(dir)?
        }
        let file_name = msg.file_name;
        let path = format!("{dir}/{file_name}");

        temp_save_picture(msg.file, &path)?;
       let result =  analysis_picture(&path)?;
        info!("result={result}");
        Ok(())
    }
}

///
/// 临时存储上传图片
///
fn temp_save_picture(mut file: File, path: &str) -> AppResult<()> {

    let mut new_file = File::create_new(path)?;
    //讲文件写入指定目录
    std::io::copy(&mut file, &mut new_file)?;
    Ok(())
}

const PADDLE: &str = "paddleocr";
fn analysis_picture(path: &str) -> AppResult<String> {
    println!("process picture analysis");
    let output = Command::new(PADDLE)
        .arg("--image_dir")
        .arg(path)
        .arg("--use_angle_cls")
        .arg("true")
        .arg("--use_gpu")
        .arg("false")
        .stdout(Stdio::piped())      // 将标准输出重定向到一个管道
        .stderr(Stdio::piped())      // 将标准错误重定向到一个管道
        .output()?;
    println!("output={:?}", output.stdout);
    Ok(String::from_utf8(output.stdout)?)
}
