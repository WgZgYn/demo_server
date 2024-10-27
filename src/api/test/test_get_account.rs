use crate::db::DB;
use actix_web::{web, HttpResponse};

pub async fn test_get_account(data: web::Data<DB>) -> HttpResponse {
    let s = data.users.read().await;
    HttpResponse::Ok().json(&*s)
}
