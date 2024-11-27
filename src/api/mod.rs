use crate::api::my::{config_my, login, signup};
use crate::api::sse::sse_account;
use crate::api::test::config_test;
use crate::db::DataBase;
use crate::dto::http::request::UserInfoUpdate;
use crate::security::auth::{Auth, Claims};
use crate::utils;
use crate::utils::Response;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use log::error;

pub mod my;
mod sse;
pub mod test;

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
                    .route(web::delete().to(delete_account))
            )
            .service(
                web::resource("/userinfo")
                    .wrap(Auth)
                    .route(web::get().to(get_user_info))
                    .route(web::post().to(update_user_info)),
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

pub async fn get_user_info(req: HttpRequest, db: web::Data<DataBase>) -> HttpResponse {
    let e = req.extensions();
    let claims = match e.get::<Claims>() {
        Some(claims) => claims,
        None => return HttpResponse::Unauthorized().finish(),
    };
    match db.get_session().await {
        Ok(session) => match session.get_user_info(claims.id()).await {
            Ok(v) => HttpResponse::Ok().json(Response::success(v)),
            Err(e) => {
                error!("{}", e);
                HttpResponse::InternalServerError().finish()
            }
        },
        Err(e) => {
            error!("{}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn update_user_info(
    req: HttpRequest,
    data: web::Json<UserInfoUpdate>,
    db: web::Data<DataBase>,
) -> HttpResponse {
    let e = req.extensions();
    let claims = match e.get::<Claims>() {
        Some(claims) => claims,
        None => return HttpResponse::Unauthorized().finish(),
    };

    match db.get_session().await {
        Ok(session) => {
            match session
                .update_user_info(data.into_inner(), claims.id())
                .await
            {
                Ok(_) => HttpResponse::Ok().json(utils::Result::success()),
                Err(e) => {
                    error!("{}", e);
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
        Err(e) => {
            error!("{}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}


async fn delete_account(req: HttpRequest, db: web::Data<DataBase>) -> HttpResponse {
    let e = req.extensions();
    let claims = match e.get::<Claims>() {
        Some(claims) => claims,
        None => return HttpResponse::Unauthorized().finish(),
    };

    let mut session = match db.get_session().await {
        Ok(session) => session,
        Err(e) => { error!("{}", e); return HttpResponse::InternalServerError().finish(); }
    };

    match session.delete_account(claims.id()).await {
        Ok(_) => HttpResponse::Ok().json(utils::Result::success()),
        Err(e) => { error!("{}", e); HttpResponse::InternalServerError().finish() }
    }
}
