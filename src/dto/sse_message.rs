use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SSEMessage {
    pub message: String,
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