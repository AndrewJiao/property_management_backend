use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct OtherPartInfo {
    car_number: Option<i32>,
    motor_cycle_number: Option<i32>,
    car_number_electron: Option<i32>
}
