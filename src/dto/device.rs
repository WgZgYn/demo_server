use crate::dto::account::Account;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceId(pub String);

#[derive(Debug, Serialize, Deserialize)]
pub struct Device {
    pub device_id: DeviceId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetDevice {
    pub account: Account,
}
