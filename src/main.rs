mod account;
mod db;
mod event;
mod ping;

use crate::db::DB;
use crate::ping::ping;
use actix_web::{web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::borrow::Cow;
use std::sync::Mutex;
use crate::account::{get_account, post_account};
use crate::event::{get_task, post_task};

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
    let counter = web::Data::new(Mutex::new(0));
    let db = web::Data::new(DB::default());

    HttpServer::new(move || {
        let counter = counter.clone();
        App::new()
            .app_data(counter.clone())
            .app_data(db.clone())

            .route("/ping", web::get().to(ping))
            .route("/task", web::get().to(get_task))
            .route("/task", web::post().to(post_task))

            .route("/account", web::post().to(post_account))
            .route("/account", web::get().to(get_account))

            .route("/message", web::get().to(get_message))
            .route("/message", web::post().to(post_message))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
