mod area;
mod device;
mod house;
mod scene;
mod member;

use crate::api::my::area::root::id::*;
use crate::api::my::area::root::*;
use crate::api::my::device::root::id::service::execute_device_service;
use crate::api::my::device::root::id::status::get_device_status;
use crate::api::my::device::root::id::*;
use crate::api::my::device::root::*;
use crate::api::my::house::root::id::{delete_house, get_house_info, update_house_info};
use crate::api::my::house::root::{add_house, get_all_house_info};
use crate::api::my::member::{add_member, delete_member, get_member};
use crate::api::my::scene::{add_scene, delete_scene, get_scene};
use crate::api::sse::sse_account;
use crate::api::{get_user_info, update_user_info};
use crate::security::auth::Auth;
use crate::service::middleware::AccessAuth;
use actix_web::body::MessageBody;
use actix_web::dev::{Service, ServiceFactory};
use actix_web::web::ServiceConfig;
use actix_web::{web, FromRequest, Responder};

pub fn config_my(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/my")
            .wrap(Auth)
            .service(
                web::scope("/device")
                    .service(
                        web::resource("")
                            .route(web::get().to(get_all_devices))
                            .route(web::post().to(add_device)),
                    )
                    .service(
                        web::scope("/{id}")
                            .service(
                                web::resource("")
                                    .route(web::get().to(get_device_info))
                                    .route(web::patch().to(update_device_info))
                                    .route(web::delete().to(delete_device)),
                            )
                            .service(
                                web::resource("/service/{name}")
                                    .route(web::to(execute_device_service)),
                            )
                            .route("/status", web::get().to(get_device_status)),
                    ),
            )
            .service(
                web::scope("/house")
                    .service(
                        web::resource("")
                            .route(web::get().to(get_all_house_info))
                            .route(web::post().to(add_house)),
                    )
                    .service(
                        web::scope("/{id}")
                            .wrap(AccessAuth)
                            .service(
                                web::resource("")
                                    .route(web::get().to(get_house_info))
                                    .route(web::patch().to(update_house_info))
                                    .route(web::delete().to(delete_house))
                            )
                    ),
            )
            .service(
                web::scope("/area")
                    .service(
                        web::resource("")
                            .route(web::get().to(get_all_area_info))
                            .route(web::post().to(add_area)),
                    )
                    .service(
                        web::scope("/{id}").service(
                            web::resource("")
                                .route(web::get().to(get_area_info))
                                .route(web::patch().to(update_area_info))
                                .route(web::delete().to(delete_area))
                        ),
                    ),
            )
            .service(
                web::resource("/member")
                    .route(web::get().to(get_member))
                    .route(web::post().to(add_member))
                    .route(web::delete().to(delete_member)),
            )
            .service(
                web::resource("/info")
                    .route(web::get().to(get_user_info))
                    .route(web::patch().to(update_user_info))
                    .route(web::put().to(update_user_info)),
            )
            .service(
                web::resource("/scene")
                    .route(web::get().to(get_scene))
                    .route(web::post().to(add_scene))
                    .route(web::delete().to(delete_scene)),
            )
            .route("/sse", web::get().to(sse_account)),
    );
}