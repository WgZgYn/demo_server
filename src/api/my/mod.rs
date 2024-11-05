use crate::api::auth::Auth;
use crate::api::my::area::*;
use crate::api::my::device::*;
use crate::api::my::house::*;
use crate::api::my::user_info::*;
use actix_web::web;
use actix_web::web::ServiceConfig;

mod area;
mod device;
mod house;
mod user_info;

pub fn config_my(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/my")
            .wrap(Auth)
            .service(
                web::resource("/area")
                    .route(web::post().to(add_area_api))
                    .route(web::get().to(query_area_api))
                    .route(web::delete().to(delete_area_api))
                    .route(web::patch().to(update_area_api)),
            )
            .service(
                web::scope("/device")
                    .service(
                        web::resource("")
                            .route(web::get().to(query_devices_api))
                            .route(web::post().to(add_device_api))
                            .route(web::delete().to(delete_device_api))
                            .route(web::patch().to(update_device_api)),
                    )
                    .route("/{id}/{service}", web::get().to(device_service_api))
                    .route("/{id}", web::get().to(device_status_api)),
            )
            .service(
                web::resource("/house")
                    .route(web::post().to(add_house_api))
                    .route(web::get().to(query_house_api))
                    .route(web::delete().to(delete_house_api))
                    .route(web::patch().to(update_house_api)),
            )
            .service(
                web::resource("/info")
                    .route(web::post().to(add_user_info_api))
                    .route(web::get().to(query_user_info_api)),
            ),
    );
}
