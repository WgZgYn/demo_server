use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct AccountInfo {
    pub account_id: i32,
    pub username: String,
}

#[derive(Serialize, Deserialize)]
pub struct HouseInfo {
    pub house_id: i32,
    pub house_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct AreaInfo {
    pub area_id: i32,
    pub area_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct DeviceInfo {
    pub device_id: i32,
    pub device_name: String,
    pub efuse_mac: String,
    pub model_id: i32,
    pub model_name: String,
    pub device_type: DeviceType,
    pub service: Vec<Value>,
}

#[derive(Serialize, Deserialize)]
pub struct DeviceType {
    pub type_id: i32,
    pub type_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct MemberInfo {}

#[derive(Serialize, Deserialize, Default)]
pub struct UserInfo {
    pub age: Option<i32>,
    pub city: Option<String>,
    pub email: Option<String>,
    pub name: Option<String>,
    pub gender: Option<String>,
}
