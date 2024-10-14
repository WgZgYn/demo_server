use demo_server::api::controller::{login, post_account};
use demo_server::api::service::sse::{sse_handler, sse_test};
use demo_server::api::test::{ping, test_auth, test_get_account, test_task};
use demo_server::db::DB;
use demo_server::device::get_device;
use demo_server::event::{get_task, post_task};
use demo_server::web_page::vue;

use actix_cors::Cors;
use actix_web::{guard, web, App, HttpServer};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::sync::Mutex;
use actix_web::middleware::Logger;
use actix_web_httpauth::middleware::HttpAuthentication;
use log::debug;
use demo_server::middleware;
use demo_server::repository::create_connection_pool;
use demo_server::security::{validator, Auth};


#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    debug!("starting server");

    // 配置证书信息
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;
    builder.set_private_key_file("private.key", SslFiletype::PEM)?;
    builder.set_certificate_chain_file("cert.pem")?;

    let pool = web::Data::new(create_connection_pool().await?);

    // 内存共享数据
    let counter = web::Data::new(Mutex::new(0));
    let db = web::Data::new(DB::default());

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .wrap(Logger::default())
            .wrap(middleware::Timer)
            .app_data(pool.clone()) // DEV only
            .app_data(counter.clone())
            .app_data(db.clone())
            .service(
                web::scope("/api")
                    .route("/ping", web::get().to(ping))
                    .route("/login", web::post().to(login))
                    .service(
                        web::scope("/sse")
                            .route("", web::get().to(sse_handler))
                            .route("/test", web::get().to(sse_test)),
                    )
                    .service(
                        web::resource("/account")
                            .wrap(Auth)
                            .route(web::post().to(post_account))
                            .route(web::get().to(test_get_account))
                    )
                    .service(
                        web::resource("/task")
                            .wrap(Auth)
                            .route(web::get().to(get_task))
                            .route(web::post().to(post_task))
                    )
                    .route("/test/auth", web::post().to(test_auth))
                    .service(
                        web::scope("/device")
                            .wrap(HttpAuthentication::bearer(validator))
                            .route("", web::get().to(get_device))
                            .route("/{id}/{ops}", web::get().to(test_task))
                    )
            )
            .service(vue()) // vue static dist
    })
        .bind("0.0.0.0:80")?
        .bind_openssl("0.0.0.0:443", builder)?
        .run()
        .await?;

    Ok(())
}
