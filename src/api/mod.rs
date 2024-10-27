use crate::api::auth::{validator, Auth};
use crate::api::login::{login, login_token};
use crate::api::my::config_my;
use crate::api::service::sse::{sse_handler, sse_test};
use crate::api::signup::signup;
use crate::api::test::{get_auth_info, ping, test_get_account, test_task};
use crate::db::device::get_device;
use crate::db::event::{get_task, post_task};
use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;

pub mod auth;
pub mod login;
pub mod my;
pub mod service;
pub mod signup;
pub mod template;
pub mod test;

pub fn config_api(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::resource("/auth")
                    .wrap(Auth)
                    .route(web::get().to(login_token)),
            )
            .route("/login", web::post().to(login))
            .route("/signup", web::post().to(signup))
            .configure(config_my)
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
                    .service(web::resource("/ping").route(web::get().to(ping)))
                    .service(
                        web::scope("/auth")
                            .wrap(Auth)
                            .route("", web::get().to(get_auth_info))
                            .route("/task/{id}/{ops}", web::post().to(test_task)),
                    ),
            )
    );
}
