use actix_web::{web, HttpResponse};
use serde_json::{json, Value};
use std::sync::Mutex;

pub async fn ping(data: web::Data<Mutex<i32>>) -> HttpResponse {
    let mut v = data.lock().unwrap();
    *v += 1;
    println!("ping {}", *v);
    HttpResponse::Ok().json(json!({"status": "ok", "times": *v}))
}

// pub async fn test() -> HttpResponse {}
pub async fn test_task(path: web::Path<(String, String)>) -> HttpResponse  {
    let (id, ops) = path.into_inner();

    let json: Value = reqwest::Client::new()
        .post("http://localhost:8080/task")
        .header("Content-Type", "application/json")
        .body(json!(
            {
                "account": {
                    "username": "wzy",
                    "password_hash": "123456"
                },
                "task": {
                    "action": ops,
                    "device_id": id
                }
            }
        ).to_string())
        .send().await.expect("failed to send request")
        .json().await.expect("failed to read response");

    HttpResponse::Ok().json(json)
}
