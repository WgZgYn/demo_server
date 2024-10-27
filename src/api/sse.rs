use crate::api::auth::Claims;
use crate::db::DB;
use crate::dto::account::Username;
use crate::dto::SSEMessage;
use actix_web::web::Bytes;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use deadpool_postgres::Pool;
use log::info;
use std::time::Duration;
use tokio::time::sleep;

pub async fn sse_account(
    data: web::Data<Pool>,
    req: HttpRequest,
    db: web::Data<DB>,
) -> HttpResponse {
    let e = req.extensions();
    let claims = e.get::<Claims>().unwrap();
    let username = claims.sub();
    let (tx, mut rx) = tokio::sync::mpsc::channel::<SSEMessage>(32);

    let mut conns = db.conn.write().await;

    conns
        .entry(Username(username.to_string()))
        .or_insert(Vec::new())
        .push(tx.clone());

    info!("new sse conn");
    let server_events = async_stream::stream! {
        info!("one sender is disconnected");
        while let Some(msg) = rx.recv().await {
            yield Ok::<_, actix_web::Error>(Bytes::from(format!("data: {}\n\n", msg.message())))
        }
    };

    HttpResponse::Ok()
        .content_type("text/event-stream")
        .streaming(server_events)
}

pub async fn sse_handler(data: web::Data<DB>) -> HttpResponse {
    let (tx, mut rx) = tokio::sync::mpsc::channel::<SSEMessage>(32);

    let mut conns = data.conn.write().await;

    conns
        .entry(Username("wzy".to_string()))
        .or_insert(Vec::new())
        .push(tx.clone());

    info!("new sse conn");
    let server_events = async_stream::stream! {
        info!("one sender is disconnected");
        while let Some(msg) = rx.recv().await {
            yield Ok::<_, actix_web::Error>(Bytes::from(format!("data: {}\n\n", msg.message())))
        }
    };

    HttpResponse::Ok()
        .content_type("text/event-stream")
        .streaming(server_events)
}

pub async fn sse_test() -> HttpResponse {
    let event_stream = async_stream::stream! {
        for i in 1..=10 {
            let data = format!("data: {}\n\n", i);  // SSE格式要求以 "data: " 开头
            yield Ok::<_, actix_web::Error>(Bytes::from(data));
            sleep(Duration::from_secs(1)).await;  // 模拟延时
        }
    };

    HttpResponse::Ok()
        .insert_header(("Content-Type", "text/event-stream"))
        .insert_header(("Cache-Control", "no-cache"))
        .streaming(event_stream)
}
