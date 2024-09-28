use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use crate::account::Account;
use crate::db::DB;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceId(String);

#[derive(Debug, Serialize, Deserialize)]
pub struct Device {
    device_id: DeviceId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetDevice {
    account: Account
}

pub async fn get_device(data: web::Data<DB>, msg: web::Json<GetDevice>) -> HttpResponse {
    let id = &msg.account.username;
    let d = data.devices.read().expect("read error");
    if let Some(devices) = d.get(id) {
        HttpResponse::Ok().json(devices)
    } else {
        HttpResponse::NotFound().finish()
    }
}