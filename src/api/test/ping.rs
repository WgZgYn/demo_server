use actix_web::{web, HttpResponse};
use serde_json::json;
use std::sync::Mutex;
use std::time::UNIX_EPOCH;

pub async fn ping(data: web::Data<Mutex<i32>>) -> HttpResponse {
    match data.lock() {
        Ok(mut v) => {
            *v += 1;
            let time = std::time::SystemTime::now();
            let stamp = time
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs();
            HttpResponse::Ok().json(json!(
                {
                    "status": "ok",
                    "timestamp": stamp,
                    "times": *v
                }
            ))
        }
        Err(_) => HttpResponse::InternalServerError().json(json!({"status": "error"})),
    }
}
