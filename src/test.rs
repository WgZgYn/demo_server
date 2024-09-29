use actix_web::{web, HttpResponse};
use serde_json::json;
use std::sync::Mutex;
use crate::account::Username;
use crate::db::DB;
use crate::device::DeviceId;
use crate::event::Task;

pub async fn ping(data: web::Data<Mutex<i32>>) -> HttpResponse {
    let mut v = data.lock().unwrap();
    *v += 1;
    println!("ping {}", *v);
    HttpResponse::Ok().json(json!({"status": "ok", "times": *v}))
}

// pub async fn test() -> HttpResponse {}
pub async fn test_task(data: web::Data<DB>, web::Path((id, ops)): web::Path<(String, String)>) -> HttpResponse {
    let mut g = data.event.write().expect("Failed to lock devices");
    g.entry(Username("wzy".to_string())).or_default().push(Task::new(ops, DeviceId(id)));
    HttpResponse::Ok().json(json!({"status": "ok"}))
}
