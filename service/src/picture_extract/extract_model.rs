use common::data_result::AppResult;
use diesel::BoolExpressionMethods;
use futures::{FutureExt, TryFutureExt};
use lazy_static::lazy_static;
use log::info;
use regex::{Match, Regex};
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

///
///
/// \[\d+/\d+/\d+ \d+:\d+:\d+\] ppocr INFO: \[\[\[(\d+.\d), (\d+.\d)\], \[(\d+.\d), (\d+.\d)\], \[(\d+.\d), (\d+.\d)\], \[(\d+.\d), (\d+.\d)\]\], \('(\S+)', (\S+)\)\]
///
const CONTENT_VALUE: &str = "content_value";
const PRECISION: &str = "precision";
const LEFT_TOP_HOR: &str = "left_top_hor";
const LEFT_TOP_VER: &str = "left_top_ver";
const RIGHT_TOP_HOR: &str = "right_top_hor";
const RIGHT_TOP_VER: &str = "right_top_ver";
const RIGHT_BOTTOM_HOR: &str = "right_bottom_hor";
const RIGHT_BOTTOM_VER: &str = "right_bottom_ver";
const LEFT_BOTTOM_HOR: &str = "left_bottom_hor";
const LEFT_BOTTOM_VER: &str = "left_bottom_ver";
const PATTERN: &str = r"\[\d+/\d+/\d+ \d+:\d+:\d+\] ppocr INFO: \[\[\[(?<left_top_hor>\d+.\d), (?<left_top_ver>\d+.\d)\], \[(?<right_top_hor>\d+.\d), (?<right_top_ver>\d+.\d)\], \[(?<right_bottom_hor>\d+.\d), (?<right_bottom_ver>\d+.\d)\], \[(?<left_bottom_hor>\d+.\d), (?<left_bottom_ver>\d+.\d)\]\], \('(?<content_value>\S+)', (?<mmatcher>\S+)\)\]";
lazy_static!(
    static ref  pattern: String = format!(r"\[\d+/\d+/\d+ \d+:\d+:\d+\] ppocr INFO: \[\[\[(?{LEFT_TOP_HOR}\d+.\d), (?{LEFT_TOP_VER}\d+.\d)\], \[(?{RIGHT_TOP_HOR}\d+.\d), (?{RIGHT_TOP_VER}\d+.\d)\], \[(?{RIGHT_BOTTOM_HOR}\d+.\d), (?{RIGHT_BOTTOM_VER}\d+.\d)\], \[(?{LEFT_BOTTOM_HOR}\d+.\d), (?{LEFT_BOTTOM_VER}\d+.\d)\]\], \('(?{CONTENT_VALUE}\S+)', (?{PRECISION}\S+)\)\]");
);
type Orientation = (f64,f64);


pub struct ExtractContent {
    values: Vec<ExtractValue>,
}
#[derive(Debug)]
pub struct ExtractValue{
    left_top: Orientation,
    right_top: Orientation,
    right_bottom: Orientation,
    left_bottom: Orientation,
    content_value:String,
    precision: f64,

    // top:Option<Orientation>,
    // bottom:Option<Orientation>,
    // left:Option<Orientation>,
    // right:Option<Orientation>,
}
trait ExtractValueTrait{
    fn extract_value(&self) -> AppResult<ExtractContent>;
}

impl ExtractValueTrait for String {
    fn extract_value(&self) -> AppResult<ExtractContent> {
        // let h_offset = SETTINGS.picture_config.owner_table_horizontal_offset;
        // let v_offset = SETTINGS.picture_config.owner_table_vertical_offset;
        // let mut horizontal_set = HashMap::new();
        // let mut vertical_set = HashMap::new();
        let mut values = Vec::new();

        let regex: Regex = Regex::new(pattern.as_str())?;
        let split = self.split("\r\n");
        for line in split{
            info!("each read line = {}",line);
            if let Some(caps) = regex.captures(line) {
                let left_top_v = caps.name(LEFT_TOP_HOR).tof64();
                let left_top_h = caps.name(LEFT_TOP_VER).tof64();
                let right_top_h = caps.name(RIGHT_TOP_HOR).tof64();
                let right_top_v = caps.name(RIGHT_BOTTOM_VER).tof64();
                // horizontal_set.insert(left_top_h, left_top_v);
                // vertical_set.insert(right_top_h, right_top_v);
                let value = ExtractValue{
                    left_top: (left_top_h, left_top_v),
                    right_top: (right_top_h, right_top_v),
                    right_bottom: (caps.name(RIGHT_BOTTOM_HOR).tof64(), caps.name(RIGHT_BOTTOM_VER).tof64()),
                    left_bottom: (caps.name(LEFT_BOTTOM_HOR).tof64(), caps.name(LEFT_BOTTOM_VER).tof64()),
                    content_value: caps.name(CONTENT_VALUE).map(|e| e.as_str().to_string()).unwrap_or_default(),
                    precision: caps.name(PRECISION).tof64(),
                };
                values.push(value);
            }
        }
        Ok(ExtractContent{values})
    }
}

trait  MaterToF64{
    fn tof64(&self) -> f64;
}
impl MaterToF64 for Option<Match<'_>> {
    fn tof64(&self) -> f64 {
        self.map(|m| m.as_str().parse().ok()).flatten().unwrap_or_default()
    }
}
