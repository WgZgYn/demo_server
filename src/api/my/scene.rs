use crate::db::DataBase;
use crate::dto::http::request::SceneAdd;
use crate::utils;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use log::error;
use crate::security::auth::Claims;

pub async fn add_scene(
    data: web::Json<SceneAdd>,
    req: HttpRequest,
    db: web::Data<DataBase>,
) -> HttpResponse {
    // TODO: judge whether the user can

    let mut session = match db.get_session().await {
        Ok(session) => session,
        Err(e) => {
            error!("{e}");
            return HttpResponse::InternalServerError().finish();
        }
    };
    match session.add_scene(data.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json(utils::Result::success()),
        Err(e) => {
            error!("{e}");
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn delete_scene(id: web::Path<i32>, db: web::Data<DataBase>) -> HttpResponse {
    let mut session = match db.get_session().await {
        Ok(session) => session,
        Err(e) => {
            error!("{e}");
            return HttpResponse::InternalServerError().finish();
        }
    };
    match session.delete_scene(id.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json(utils::Result::success()),
        Err(e) => {
            error!("{e}");
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn get_scene(req: HttpRequest, db: web::Data<DataBase>) -> HttpResponse {
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

    match session.get_scene(claims.id()).await {
        Ok(_) => HttpResponse::Ok().json(utils::Result::success()),
        Err(e) => { error!("{}", e); HttpResponse::InternalServerError().finish() }
    }
}