use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, Hash, PartialEq)]
pub struct Username(pub String);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    pub username: Username,
    pub password_hash: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum AccountAction {
    Create,
    Update,
    Delete,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AccountQuery {
    pub account: Account,
    pub action: AccountAction,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SignupAccount {
    username: String,
    password: String,
}

impl SignupAccount {
    pub fn username(&self) -> &str {
        &self.username
    }
    pub fn password(&self) -> &str {
        &self.password
    }
}
