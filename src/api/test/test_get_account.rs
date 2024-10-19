use crate::db::DB;
use actix_web::{web, HttpResponse};

pub async fn test_get_account(data: web::Data<DB>) -> HttpResponse {
    if let Ok(s) = data.users.read() {
        HttpResponse::Ok().json(&*s)
    } else {
        HttpResponse::InternalServerError().finish()
    }
}
