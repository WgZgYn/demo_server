use demo_server::api::controller::{add_area, add_device, add_house, login, login_token, show_devices, signup};
use demo_server::api::service::sse::{sse_handler, sse_test};
use demo_server::api::test::{ping, get_auth_info, test_get_account, test_task};
use demo_server::db::DB;
use demo_server::device::get_device;
use demo_server::event::{get_task, post_task};
use demo_server::web_page::vue;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpMessage, HttpRequest, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use demo_server::middleware;
use demo_server::repository::create_connection_pool;
use demo_server::security::{validator, Auth, RecordIP};
use log::debug;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::sync::Mutex;

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
            .wrap(RecordIP::default())
            .app_data(pool.clone()) // DEV only
            .app_data(counter.clone())
            .app_data(db.clone())
            .service(
                web::scope("/api")
                    .service(
                        web::resource("/auth")
                            .wrap(Auth)
                            .route(web::get().to(login_token))
                    )
                    .route("/login", web::post().to(login))
                    .route("/signup", web::post().to(signup))
                    .service(
                        web::scope("/my")
                            .wrap(Auth)
                            .service(
                                web::resource("/area")
                                    .route(web::post().to(add_area)),
                            )
                            .service(
                                web::resource("/device")
                                    .route(web::get().to(show_devices))
                                    .route(web::post().to(add_device)),
                            )
                            .service(
                                web::resource("/house")
                                    .route(web::post().to(add_house)),
                            )
                    )
                    .service(
                        web::scope("/sse")
                            .route("", web::get().to(sse_handler))
                            .route("/test", web::get().to(sse_test)),
                    )
                    .service(
                        web::resource("/account")
                            .wrap(Auth)
                            .route(web::get().to(test_get_account)),
                    )
                    .service(
                        web::resource("/task")
                            .wrap(Auth)
                            .route(web::get().to(get_task))
                            .route(web::post().to(post_task)),
                    )
                    .service(
                        web::scope("/device")
                            .wrap(HttpAuthentication::bearer(validator))
                            .route("", web::get().to(get_device))
                            .route("/{id}/{ops}", web::get().to(test_task)),
                    )
                    .service(
                        web::scope("/test")
                            .service(
                                web::resource("/ping")
                                    .route(web::get().to(ping)),
                            )
                            .service(
                                web::scope("/auth")
                                    .wrap(Auth)
                                    .route("", web::get().to(get_auth_info))
                                    .route("/task/{id}/{ops}", web::post().to(test_task)),
                            )
                    )
                ,
            )
            .service(vue()) // vue static dist
    })
        .bind("0.0.0.0:80")?
        .bind_openssl("0.0.0.0:443", builder)?
        .run()
        .await?;

    Ok(())
}
