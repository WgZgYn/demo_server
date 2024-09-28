use crate::account::{Account, UserId};
use crate::event::Task;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::RwLock;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DB {
    pub users: RwLock<HashMap<UserId, Account>>,
    pub event: RwLock<HashMap<UserId, Vec<Task>>>,
}
