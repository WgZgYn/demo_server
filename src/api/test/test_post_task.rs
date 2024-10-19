use actix_web::{web, HttpResponse};
use serde_json::{json, Value};

pub async fn test_task(path: web::Path<(String, String)>) -> HttpResponse {
    let (id, ops) = path.into_inner();
    if let Ok(json) = send(id, ops).await {
        HttpResponse::Ok().json(json)
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

async fn send(id: String, ops: String) -> reqwest::Result<Value> {
    reqwest::Client::new()
        .post("http://localhost:80/task")
        .header("Content-Type", "application/json")
        .body(
            json!(
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
            )
            .to_string(),
        )
        .send()
        .await?
        .json()
        .await
}
