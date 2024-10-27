use crate::db::DB;
use crate::dto::sse_message::SSEMessage;
use crate::dto::task::{GetTask, PostTask, Task};
use actix_web::{web, HttpResponse};
use log::info;
use serde_json::json;

// Outdated

pub async fn get_task(data: web::Data<DB>, msg: web::Json<GetTask>) -> HttpResponse {
    let id = &msg.account.username;
    let mut tasks = data.event().write().await;

    if let Some(tasks) = tasks.0.get_mut(id) {
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
    info!("POST task");

    let msg = msg.into_inner();
    let task = msg.task;

    // 尝试写入事件，返回错误时返回 500 响应
    {
        let mut events = data.event().write().await;
        let id = msg.account.username.clone();
        events.0.entry(id).or_default().push(task);
    }

    // 尝试读取连接，返回错误时返回 500 响应
    let mut conn = data.conn.write().await;

    if let Some(senders) = conn.get_mut(&msg.account.username) {
        let n = senders.len();
        for i in 0..n {
            match senders[i].send(SSEMessage::new("update")).await {
                Ok(_) => {
                    info!("SSE message sent");
                }
                Err(_) => {
                    info!("one sender is disconnected");
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
