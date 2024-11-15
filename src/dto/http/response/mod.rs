use crate::dto::entity::simple::{AccountInfo, AreaInfo, DeviceInfo, HouseInfo};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct AccountDevices {
    pub account_info: AccountInfo,
    pub houses_devices: Vec<HouseDevices>,
}
#[derive(Serialize, Deserialize)]
pub struct HouseDevices {
    pub house_info: HouseInfo,
    pub areas_devices: Vec<AreaDevices>,
}
#[derive(Serialize, Deserialize)]
pub struct AreaDevices {
    pub area_info: AreaInfo,
    pub devices: Vec<DeviceInfo>,
}

#[derive(Serialize, Deserialize)]
pub struct LoginSuccess {
    pub token: String,
}
