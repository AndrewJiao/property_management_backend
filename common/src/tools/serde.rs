use serde::{Deserialize, Deserializer, Serializer};
use crate::const_value::SETTINGS;
use crate::error::PARAM_NOT_SUPPORT;

// 反序列化：空字符串和 null 都转换为 None，其他字符串为 Some(value)
pub fn empty_string_or_null_as_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    // 尝试反序列化为 Option<String>
    let option_str: Option<String> = Option::deserialize(deserializer)?;

    // 如果是空字符串或 null 则转为 None
    match option_str {
        Some(s) if s.is_empty() => Ok(None),
        None => Ok(None),
        Some(s) => Ok(Some(s)),
    }
}


pub fn empty_vec_or_null_as_none<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    // 尝试反序列化为 Option<String>
    let option_str: Option<Vec<String>> = Option::deserialize(deserializer)?;

    // 如果是空字符串或 null 则转为 None
    match option_str {
        Some(s) if s.is_empty() => Ok(None),
        None => Ok(None),
        Some(s) => Ok(Some(s)),
    }
}


// 序列化：None 转换为 null，空字符串保持为空字符串
pub fn none_as_null_or_empty_string<S>(value: &Option<String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        Some(s) if s.is_empty() => serializer.serialize_str(""),  // 空字符串序列化为 ""
        Some(s) => serializer.serialize_str(s),                    // 非空字符串按原值序列化
        None => serializer.serialize_none(),                       // None 序列化为 null
    }
}


pub fn json_verify<'de, D>(deserializer: D) -> Result<Option<serde_json::Value>, D::Error>
where
    D: Deserializer<'de>,
{
    let option_str: Option<serde_json::Value> = Option::deserialize(deserializer)?;

    match option_str {
        Some(value) => {
            match value.as_str() {
                None => { Ok(None) }
                Some(value_str) => {
                    //verify
                    let length = value_str.len();
                    if SETTINGS.app_config.json_length > length as u32 {
                        Ok(Some(value))
                    } else {
                        Err(serde::de::Error::custom(PARAM_NOT_SUPPORT().to_string()))
                    }
                }
            }
        }
        None => Ok(None),
    }
}

// #[derive(Serialize, Deserialize, Debug)]
// struct MyStruct {
//     #[serde(deserialize_with = "empty_string_or_null_as_none", serialize_with = "none_as_null_or_empty_string")]
//     my_field: Option<String>,
// }
//
// fn main() {
//     // 反序列化时处理 null 和空字符串
//     let json_data = r#"{"my_field": ""}"#;
//     let result: MyStruct = serde_json::from_str(json_data).unwrap();
//     println!("{:?}", result.my_field); // 输出: None
//
//     let json_data2 = r#"{"my_field": null}"#;
//     let result2: MyStruct = serde_json::from_str(json_data2).unwrap();
//     println!("{:?}", result2.my_field); // 输出: None
//
//     let json_data3 = r#"{"my_field": "Hello"}"#;
//     let result3: MyStruct = serde_json::from_str(json_data3).unwrap();
//     println!("{:?}", result3.my_field); // 输出: Some("Hello")
//
//     // 序列化时处理 None 和空字符串
//     let value1 = MyStruct {
//         my_field: None,
//     };
//     let json1 = serde_json::to_string(&value1).unwrap();
//     println!("{}", json1); // 输出: {"my_field":null}
//
//     let value2 = MyStruct {
//         my_field: Some("".to_string()),
//     };
//     let json2 = serde_json::to_string(&value2).unwrap();
//     println!("{}", json2); // 输出: {"my_field":""}
//
//     let value3 = MyStruct {
//         my_field: Some("Hello".to_string()),
//     };
//     let json3 = serde_json::to_string(&value3).unwrap();
//     println!("{}", json3); // 输出: {"my_field":"Hello"}
// }
