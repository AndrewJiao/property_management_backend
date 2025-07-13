use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct InnerSetting {
    pub inner_setting_value: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Setting {
    pub test_setting: InnerSetting,
}


#[actix_web::main]
async fn main() {
    let result = std::env::var("TEST_SETTING").unwrap();

    config::Config::builder()
        .set_override("test_setting.inner_setting_value", result).unwrap()
        .build()
        .unwrap()
        .try_deserialize::<Setting>()
        .map(|setting| {
            println!("Inner setting value: {}", setting.test_setting.inner_setting_value);
        })
        .unwrap_or_else(|err| {
            eprintln!("Failed to deserialize settings: {}", err);
        });



}
