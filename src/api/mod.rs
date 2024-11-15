use crate::api::auth::{validator, Auth};
use crate::api::login::{login, login_token};
use crate::api::my::{config_my, config_my_v2};
use crate::api::signup::signup;
use crate::api::sse::sse_account;
use crate::api::test::config_test;
use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;
use crate::api::v2::{get_user_info, update_user_info};

pub mod auth;
pub mod login;
pub mod my;
pub mod signup;
mod sse;
pub mod test;
mod v2;

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
            .service(web::scope("/sse").route("", web::get().to(sse_account)))
            .service(web::resource("/task").wrap(Auth))
            .service(web::scope("/device").wrap(HttpAuthentication::bearer(validator)))
            .configure(config_test),
    );
}

pub fn config_api_v2(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::resource("/auth")
                    .wrap(Auth)
                    .route(web::get().to(login_token)),
            )
            .route("/login", web::post().to(v2::login))
            .route("/signup", web::post().to(v2::signup))
            .service(
                web::resource("/userinfo")
                    .route(web::get().to(get_user_info))
                    .route(web::post().to(update_user_info))
            )
            .service(
                web::scope("/sse")
                    .wrap(Auth)
                    .route("", web::get().to(sse_account))
            )
            .configure(config_my_v2),
    );
}
