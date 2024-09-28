use actix_web::{web, HttpResponse};
use serde_json::json;
use std::sync::Mutex;

pub async fn ping(data: web::Data<Mutex<i32>>) -> HttpResponse {
    let mut v = data.lock().unwrap();
    *v += 1;
    println!("ping {}", *v);
    HttpResponse::Ok().json(json!({"status": "ok", "times": *v}))
}

// pub async fn test() -> HttpResponse {}
