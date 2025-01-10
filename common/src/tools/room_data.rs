
//
// ///
// /// 用正则提取房号里面的单元，楼层，房号
// /// 例如：A302 提取出 A 3 A302
// ///
// pub fn extract_room_number(room_number: &str) -> (String, String, String) {
//     let re = regex::Regex::new(r"(\d+)-(\d+)-(\d+)").unwrap();
//     let cap = re.captures(room_number).unwrap();
//     let unit = cap.get(1).unwrap().as_str().to_string();
//     let floor = cap.get(2).unwrap().as_str().to_string();
//     let room = cap.get(3).unwrap().as_str().to_string();
//     (unit, floor, room)
// }

