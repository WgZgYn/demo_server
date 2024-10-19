use crate::dto::account::{Account, Username};
use crate::dto::device::Device;
use crate::dto::sse_message::SSEMessage;
use crate::dto::task::Task;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::RwLock;
use tokio::sync::mpsc;

#[derive(Serialize, Deserialize, Debug, Default)]
struct Tasks {
    pub event: RwLock<HashMap<Username, Vec<Task>>>,
}

#[derive(Debug, Default)]
pub struct DB {
    pub users: RwLock<HashMap<Username, Account>>,
    pub devices: RwLock<HashMap<Username, Vec<Device>>>,
    pub conn: RwLock<HashMap<Username, Vec<mpsc::Sender<SSEMessage>>>>,
    pub tasks: Tasks,
}

impl DB {
    pub(crate) fn event(&self) -> &RwLock<HashMap<Username, Vec<Task>>> {
        &self.tasks.event
    }
}
