use crate::db::DB;
use actix_web::{web, HttpResponse};
use crate::dto::device::GetDevice;

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
