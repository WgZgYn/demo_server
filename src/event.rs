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
    let mut tasks = data.event().write().expect("event not read");

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

    let id = msg.account.username.clone();
    let task = msg.into_inner().task;

    data.event()
        .write()
        .expect("Failed to write event")
        .entry(id.clone())
        .or_default()
        .push(task);

    let mut ss = data.conn.write().expect("Failed to write connection");

    let ss = ss.entry(id).or_default();

    // TODO: remove the disconnect sender
    for s in ss {
        println!("send update to sse");
        s.send(SSEMessage::new("update")).await.expect("can't send");
    }

    HttpResponse::Ok().json(json!(
        {
            "status": "ok",
            "action": "post_task"
        }
    ))
}
