use crate::utils::timestamp::get_timestamp;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Result<'a> {
    code: i32,
    message: &'a str,
    timestamp: u64,
}

impl<'a> Result<'a> {
    pub fn new(code: i32, message: &'a str) -> Result<'a> {
        Self { code, message, timestamp: get_timestamp() }
    }

    pub fn success() -> Result<'a> {
        Self::new(200, "success")
    }

    pub fn error(msg: &'a str) -> Result<'a> {
        Self::new(500, msg)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response<'a, T> {
    code: i32,
    message: &'a str,
    timestamp: u64,
    data: Option<T>
}

impl<'a, T> Response<'a, T> {
    pub fn new(code: i32, message: &'a str, data: Option<T>) -> Response<'a, T> {
        Self { code, message, timestamp: get_timestamp(), data }
    }

    pub fn success(data: T) -> Response<'a, T> {
        Self::new(200, "success", Some(data))
    }

    pub fn error(msg: &'a str) -> Result<'a> {
        Result::new(500, msg)
    }
}