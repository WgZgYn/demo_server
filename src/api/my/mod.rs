use crate::api::auth::Auth;
use crate::api::my::area::add_area_api;
use crate::api::my::device::{add_device_api, show_devices_api};
use crate::api::my::house::add_house_api;
use actix_web::web;
use actix_web::web::ServiceConfig;

pub mod area;
pub mod device;
pub mod house;

pub fn config_my(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .wrap(Auth)
            .service(web::resource("/area").route(web::post().to(add_area_api)))
            .service(
                web::resource("/device")
                    .route(web::get().to(show_devices_api))
                    .route(web::post().to(add_device_api)),
            )
            .service(web::resource("/house").route(web::post().to(add_house_api)))
    );
}
