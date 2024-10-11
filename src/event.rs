use crate::account::Account;
use crate::db::{SSEMessage, DB};
use crate::device::DeviceId;
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    action: String,
    device_id: DeviceId,
}

impl Task {
    pub fn new(action: String, device_id: DeviceId) -> Self {
        Task { action, device_id }
    }
}

#[derive(Serialize, Deserialize)]
pub struct GetTask {
    account: Account,
}

#[derive(Serialize, Deserialize)]
pub struct PostTask {
    pub account: Account,
    pub task: Task,
}

pub async fn get_task(data: web::Data<DB>, msg: web::Json<GetTask>) -> HttpResponse {
    let id = &msg.account.username;
    let mut tasks = match data.event().write() {
        Ok(tasks) => tasks,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    if let Some(tasks) = tasks.get_mut(id) {
        let v: Vec<Task> = tasks.drain(..).collect();
        HttpResponse::Ok().json(json!(
            {
                "tasks": v,
                "status": "ok",
                "action": "get_task"
            }
        ))
    } else {
        HttpResponse::NotFound().finish()
    }
}

pub async fn post_task(data: web::Data<DB>, msg: web::Json<PostTask>) -> HttpResponse {
    println!("POST task");

    let msg = msg.into_inner();
    let task = msg.task;

    // 尝试写入事件，返回错误时返回 500 响应
    {
        let mut events = match data.event().write() {
            Ok(events) => events,
            Err(_) => return HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Failed to write event"
            }))
        };


        let id = msg.account.username.clone();
        events.entry(id).or_default().push(task);
    }

    // 尝试读取连接，返回错误时返回 500 响应
    let mut conn = match data.conn.write() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": "Failed to read connection"
        })),
    };


    if let Some(senders) = conn.get_mut(&msg.account.username) {
        let n = senders.len();
        for i in 0..n {
            match senders[i].send(SSEMessage::new("update")).await {
                Ok(_) => {
                    println!("SSE message sent");
                }
                Err(_) => {
                    println!("one sender is disconnected");
                    senders.swap_remove(i); // Then remove the connection
                }
            }
        }
    }

    HttpResponse::Ok().json(json!({
        "status": "ok",
        "action": "post_task"
    }))
}
