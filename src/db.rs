use crate::account::{Account, Username};
use crate::event::Task;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::RwLock;
use crate::device::Device;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DB {
    pub users: RwLock<HashMap<Username, Account>>,
    pub event: RwLock<HashMap<Username, Vec<Task>>>,
    pub devices: RwLock<HashMap<Username, Vec<Device>>>
}
