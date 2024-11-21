use crate::data::sse::SseHandler;
use crate::security::auth::Claims;
use actix_web::web::Bytes;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use log::info;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::time::sleep;

pub async fn sse_account(sse: web::Data<RwLock<SseHandler>>, req: HttpRequest) -> HttpResponse {
    let e = req.extensions();
    let claims = e.get::<Claims>().unwrap();
    let account_id = claims.id();
    let (key, mut rx) = sse.write().await.new_session(account_id);
    info!("new sse conn");

    let server_events = async_stream::stream! {
        while let Some(msg) = rx.recv().await {
            yield Ok::<_, actix_web::Error>(Bytes::from(format!("data: {}\n\n", msg)))
        }
        info!("one session is to be closed");
        sse.write().await.close_session(account_id, key, rx);
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
