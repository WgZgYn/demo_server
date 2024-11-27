mod area;
mod device;
mod house;
mod scene;

use crate::api::my::area::root::id::*;
use crate::api::my::area::root::*;
use crate::api::my::device::root::id::service::execute_device_service;
use crate::api::my::device::root::id::status::get_device_status;
use crate::api::my::device::root::id::*;
use crate::api::my::device::root::*;
use crate::api::my::house::root::id::{delete_house, get_house_info, update_house_info};
use crate::api::my::house::root::{add_house, get_all_house_info};
use crate::api::my::scene::{add_scene, delete_scene, get_scene};
use crate::api::sse::sse_account;
use crate::db::{DataBase, Memory};
use crate::dto::http::request::{AreaAdd, HouseAdd, Login, MemberAdd, MemberDelete, SceneAdd, Signup, UserInfoUpdate};
use crate::dto::http::response::LoginSuccess;
use crate::dto::mqtt::HostMessage;
use crate::security::auth::{Auth, Claims};
use crate::security::hash;
use crate::security::hash::{gen_salt, password_hash};
use crate::service::send_host_message;
use crate::utils;
use crate::utils::Response;
use actix_web::http::Method;
use actix_web::web::ServiceConfig;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};
use log::{error, info};
use rumqttc::AsyncClient;
use crate::api::{get_user_info, update_user_info};

pub fn config_my(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/my")
            .wrap(Auth)
            .service(
                web::scope("/device")
                    .service(
                        web::resource("")
                            .route(web::get().to(get_all_devices))
                            .route(web::post().to(add_device)), // TODO: Use/Test this api
                    )
                    .service(
                        web::scope("/{id}")
                            .service(
                                web::resource("")
                                    .route(web::get().to(get_device_info))
                                    .route(web::patch().to(update_device_info)) // TODO: Use/Test this api
                                    .route(web::delete().to(delete_device)), // TODO: Use/Test this api
                            )
                            .service(
                                web::resource("/service/{name}")
                                    .route(web::to(execute_device_service)),
                            )
                            .route("/status", web::get().to(get_device_status)), // TODO: Use/Test this api
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
                        web::scope("/{id}").service(
                            web::resource("")
                                .route(web::get().to(get_house_info))
                                .route(web::patch().to(update_house_info))
                                .route(web::delete().to(delete_house))
                        ),
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
            // TODO:
            .service(
                web::resource("/scene")
                    .route(web::get().to(get_scene))
                    .route(web::post().to(add_scene))
                    .route(web::delete().to(delete_scene)),
            )
            .route("/sse", web::get().to(sse_account)),
    );
}

pub async fn login(data: web::Json<Login>, db: web::Data<DataBase>) -> HttpResponse {
    let Login { username, password } = data.into_inner();
    let session = match db.get_session().await {
        Ok(session) => session,
        Err(e) => {
            error!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    let (account_id, password_hash) = match session.get_account_id_password_hash(&username).await {
        Ok(v) => v,
        Err(e) => {
            info!("{}", e);
            return HttpResponse::Unauthorized().json(utils::Result::error("No such Account"));
        }
    };

    if hash::password_verify(&password_hash, password.as_ref()) {
        let token = match session
            .update_account_last_login(account_id, username)
            .await
        {
            Ok(v) => v,
            Err(e) => {
                error!("{}", e);
                return HttpResponse::InternalServerError().finish();
            }
        };
        HttpResponse::Ok().json(Response::success(LoginSuccess { token }))
    } else {
        HttpResponse::Unauthorized().json(utils::Result::error("Wrong password"))
    }
}

pub async fn signup(data: web::Json<Signup>, db: web::Data<DataBase>) -> HttpResponse {
    let Signup { username, password } = data.into_inner();
    let session = match db.get_session().await {
        Ok(session) => session,
        Err(e) => {
            error!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    let salt = gen_salt();
    let hash = password_hash(&password, &salt);

    if let Err(e) = session.add_account(&username, &hash, &salt).await {
        error!("{}", e);
        return HttpResponse::InternalServerError().json(utils::Result::error("No such Account"));
    }

    HttpResponse::Ok().finish()
}

pub async fn delete_member(data: web::Json<MemberDelete>, db: web::Data<DataBase>) -> HttpResponse {
    let session = match db.get_session().await {
        Ok(session) => session,
        Err(e) => {
            error!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };
    let MemberDelete { house_id, account_id } = data.into_inner();
    match session.delete_member(house_id, account_id).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            error!("{}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn add_member(data: web::Json<MemberAdd>, db: web::Data<DataBase>) -> HttpResponse {
    let session = match db.get_session().await {
        Ok(session) => session,
        Err(e) => {
            error!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };
    let MemberAdd { house_id, account_id } = data.into_inner();
    match session.add_member(house_id, account_id).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            error!("{}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn get_member(req: HttpRequest, db: web::Data<DataBase>) -> HttpResponse {
    let e = req.extensions();
    let claims = match e.get::<Claims>() {
        Some(claims) => claims,
        None => return HttpResponse::Unauthorized().finish(),
    };
    let session = match db.get_session().await {
        Ok(session) => session,
        Err(e) => {
            error!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    match session.get_member(claims.id()).await {
        Ok(member) => HttpResponse::Ok().json(Response::success(member)),
        Err(e) => {
            error!("{}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}