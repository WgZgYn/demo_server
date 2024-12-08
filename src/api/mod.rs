use crate::api::account::{delete_account, get_user_info, login, signup, update_account, update_user_info};
use crate::api::my::config_my;
use crate::api::sse::sse_account;
use crate::api::test::config_test;
use crate::security::auth::Auth;
use crate::utils;
use actix_web::{web, HttpResponse};

pub mod my;
mod sse;
pub mod test;
mod account;

pub async fn login_token() -> HttpResponse {
    HttpResponse::Ok().json(utils::Result::success())
}

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
            .service(
                web::resource("/account")
                    .wrap(Auth)
                    .route(web::patch().to(update_account))
                    .route(web::delete().to(delete_account))
            )
            .service(
                web::resource("/userinfo")
                    .wrap(Auth)
                    .route(web::get().to(get_user_info))
                    .route(web::post().to(update_user_info))
                    .route(web::patch().to(update_user_info)),
            )
            .service(
                web::scope("/sse")
                    .wrap(Auth)
                    .route("", web::get().to(sse_account)),
            )
            .configure(config_my)
            .configure(config_test),
    );
}