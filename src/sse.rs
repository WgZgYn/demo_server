use crate::account::Username;
use crate::db::{SSEMessage, DB};
use actix_web::web::Bytes;
use actix_web::{web, HttpResponse};
// type SseStream = Pin<Box<dyn Stream<Item = Result<Bytes, actix_web::Error>>>>;

pub async fn sse_handler(data: web::Data<DB>) -> HttpResponse {
    let (tx, mut rx) = tokio::sync::mpsc::channel::<SSEMessage>(32);

    let mut conns = data.conn.write().expect("");
    conns
        .entry(Username("wzy".to_string()))
        .or_insert(Vec::new())
        .push(tx.clone());

    println!("new sse conn");
    let server_events = async_stream::stream! {
            while let Some(msg) = rx.recv().await {
                yield Ok::<_, actix_web::Error>(Bytes::from(format!("data: {}\n\n", msg.message())))
            }
    };

    HttpResponse::Ok()
        .content_type("text/event-stream")
        .streaming(server_events)
}
