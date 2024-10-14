use crate::db::DB;
use crate::dto::task::{GetTask, PostTask, Task};
use actix_web::{web, HttpResponse};
use log::info;
use serde_json::json;
use crate::dto::sse_message::SSEMessage;

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
    info!("POST task");

    let msg = msg.into_inner();
    let task = msg.task;

    // 尝试写入事件，返回错误时返回 500 响应
    {
        let mut events = match data.event().write() {
            Ok(events) => events,
            Err(_) => {
                return HttpResponse::InternalServerError().json(json!({
                    "status": "error",
                    "message": "Failed to write event"
                }))
            }
        };

        let id = msg.account.username.clone();
        events.entry(id).or_default().push(task);
    }

    // 尝试读取连接，返回错误时返回 500 响应
    let mut conn = match data.conn.write() {
        Ok(conn) => conn,
        Err(_) => {
            return HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Failed to read connection"
            }))
        }
    };

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
