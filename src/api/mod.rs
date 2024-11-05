use crate::api::auth::{validator, Auth};
use crate::api::login::{login, login_token};
use crate::api::my::config_my;
use crate::api::signup::signup;
use crate::api::test::{config_test};
use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;
use crate::api::sse::sse_account;

pub mod auth;
pub mod login;
pub mod my;
pub mod signup;
pub mod test;
mod sse;

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
                    .route("", web::get().to(sse_account))
            )
            .service(
                web::resource("/task")
                    .wrap(Auth)
            )
            .service(
                web::scope("/device")
                    .wrap(HttpAuthentication::bearer(validator))
            )
            .configure(config_test),
    );
}
