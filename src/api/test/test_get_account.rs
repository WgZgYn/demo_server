use actix_web::{web, HttpResponse};
use crate::db::DB;

pub async fn test_get_account(data: web::Data<DB>) -> HttpResponse {
    if let Ok(s) = data.users.read() {
        HttpResponse::Ok().json(&*s)
    } else {
        HttpResponse::InternalServerError().finish()
    }
}