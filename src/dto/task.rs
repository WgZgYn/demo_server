use serde::{Deserialize, Serialize};

use crate::dto::account::Account;
use crate::dto::device::DeviceId;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub action: String,
    pub device_id: DeviceId,
}

impl Task {
    pub fn new(action: String, device_id: DeviceId) -> Self {
        Task { action, device_id }
    }
}

#[derive(Serialize, Deserialize)]
pub struct GetTask {
    pub account: Account,
}

#[derive(Serialize, Deserialize)]
pub struct PostTask {
    pub account: Account,
    pub task: Task,
}