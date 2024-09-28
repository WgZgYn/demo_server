use crate::account::Account;
use crate::db::DB;
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    action: String,
    device_id: usize,
}

#[derive(Serialize, Deserialize)]
pub struct GetTask {
    account: Account,
}

#[derive(Serialize, Deserialize)]
pub struct PostTask {
    account: Account,
    task: Task,
}

pub async fn get_task(data: web::Data<DB>, msg: web::Json<GetTask>) -> HttpResponse {
    let id = &msg.account.user_id;
    let tasks = data
        .event
        .read()
        .expect("event not read");

    let tasks = tasks
        .get(id);

    if let Some(tasks) = tasks {
        HttpResponse::Ok().json(json!(
            {
                "tasks": tasks,
                "status": "ok",
                "action": "get_task"
            }
        ))
    } else {
        HttpResponse::Ok().json(json!({ "status": "error" }))
    }
}

pub async fn post_task(data: web::Data<DB>, msg: web::Json<PostTask>) -> HttpResponse {
    let id = msg.account.user_id;
    let task = msg.into_inner().task;
    data.event
        .write()
        .expect("Failed to write event")
        .entry(id)
        .or_default()
        .push(task);

    HttpResponse::Ok().json(json!(
        {
            "status": "ok",
            "action": "post_task"
        }
    ))
}
