use crate::utils::timestamp::get_timestamp;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Result {
    code: i32,
    message: String,
    timestamp: u64,
}

impl Result {
    pub fn new(code: i32, message: String) -> Result {
        Self { code, message, timestamp: get_timestamp() }
    }

    pub fn success() -> Result {
        Self::new(200, "success".to_string())
    }

    pub fn error(msg: String) -> Result {
        Self::new(500, msg)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response<T> {
    code: i32,
    message: String,
    timestamp: u64,
    data: Option<T>
}

impl<T> Response<T> {
    pub fn new(code: i32, message: String, data: Option<T>) -> Response<T> {
        Self { code, message, timestamp: get_timestamp(), data }
    }

    pub fn success(data: T) -> Response<T> {
        Self::new(200, "success".to_string(), Some(data))
    }

    pub fn error(msg: String) -> Result {
        Result::new(500, msg)
    }
}