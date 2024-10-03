use crate::account::Account;
use crate::db::DB;
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceId(pub String);

#[derive(Debug, Serialize, Deserialize)]
pub struct Device {
    device_id: DeviceId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetDevice {
    account: Account,
}

pub async fn get_device(data: web::Data<DB>, msg: web::Json<GetDevice>) -> HttpResponse {
    let id = &msg.account.username;
    match data.devices.read() {
        Ok(d) => match d.get(&id) {
            Some(d) => HttpResponse::Ok().json(d),
            None => HttpResponse::NotFound().body("Device not found for account"),
        },
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
