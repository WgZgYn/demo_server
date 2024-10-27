use crate::utils;
use actix_web::{web, HttpResponse};
use tokio::sync::Mutex;

pub async fn ping(data: web::Data<Mutex<i32>>) -> HttpResponse {
    let mut v = data.lock().await;
    *v += 1;
    HttpResponse::Ok().json(utils::Response::success(*v))
}
