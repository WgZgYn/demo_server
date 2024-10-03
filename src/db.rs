use crate::account::{Account, Username};
use crate::device::Device;
use crate::event::Task;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::RwLock;
use tokio::sync::mpsc;

#[derive(Debug, Default)]
pub struct DB {
    pub users: RwLock<HashMap<Username, Account>>,
    pub devices: RwLock<HashMap<Username, Vec<Device>>>,
    tasks: Tasks,
    pub conn: RwLock<HashMap<Username, Vec<mpsc::Sender<SSEMessage>>>>,
}

impl DB {
    pub(crate) fn event(&self) -> &RwLock<HashMap<Username, Vec<Task>>> {
        &self.tasks.event
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SSEMessage {
    message: String,
}

impl SSEMessage {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
    pub fn message(&self) -> &str {
        &self.message
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Tasks {
    pub event: RwLock<HashMap<Username, Vec<Task>>>,
}
