mod account;
mod db;
mod device;
mod event;
mod sse;
mod test;
mod web_page;

use crate::account::{get_account, post_account};
use crate::db::DB;
use crate::device::get_device;
use crate::event::{get_task, post_task};
use crate::sse::sse_handler;
use crate::test::{ping, test_task};
use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::borrow::Cow;
use std::sync::Mutex;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use crate::web_page::vue;

#[derive(Debug, Deserialize, Serialize)]
struct Message {
    username: Cow<'static, str>,
    password_hash: Cow<'static, str>,
    content: Cow<'static, str>,
}

async fn get_message() -> HttpResponse {
    HttpResponse::Ok().body("ok")
}

async fn post_message(msg: web::Json<Message>) -> HttpResponse {
    HttpResponse::Ok().json(json!(
        {
            "status": "success",
            "content": "echo ".to_string() + &msg.0.content
        }
    ))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 配置证书信息
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;
    builder.set_private_key_file("private.key", SslFiletype::PEM)?;
    builder.set_certificate_chain_file("cert.pem")?;

    // 内存共享数据
    let counter = web::Data::new(Mutex::new(0));
    let db = web::Data::new(DB::default());

    HttpServer::new(move || {
        let counter = counter.clone();
        App::new()
            .wrap(Cors::permissive()) // DEV only
            .service(vue()) // Put VUE package to /static
            .app_data(counter.clone())
            .app_data(db.clone())
            .route("/ping", web::get().to(ping))
            .route("/task", web::get().to(get_task))
            .route("/task", web::post().to(post_task))
            .route("/account", web::post().to(post_account))
            .route("/account", web::get().to(get_account))
            .route("/device", web::get().to(get_device))
            .route("/device/{id}/{ops}", web::get().to(test_task))
            .route("/message", web::get().to(get_message))
            .route("/message", web::post().to(post_message))
            .route("/sse", web::get().to(sse_handler))
    })
        .bind("0.0.0.0:80")?
        .bind_openssl("0.0.0.0:443", builder)?
        .run()
        .await
}
