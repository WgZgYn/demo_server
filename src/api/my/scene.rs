use crate::db::DataBase;
use crate::dto::http::request::SceneAdd;
use crate::security::auth::get_id_from_http_request;
use crate::utils;
use crate::utils::Response;
use actix_web::{web, HttpRequest, HttpResponse};
use log::error;

pub async fn add_scene(
    data: web::Json<SceneAdd>,
    _req: HttpRequest,
    db: web::Data<DataBase>,
) -> HttpResponse {
    // TODO: judge whether the user can
    let session = match db.get_session().await {
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
    let id = match get_id_from_http_request(&req) {
        Some(id) => id,
        None => return HttpResponse::Unauthorized().finish(),
    };

    let session = match db.get_session().await {
        Ok(session) => session,
        Err(e) => {
            error!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    match session.get_scene(id).await {
        Ok(v) => HttpResponse::Ok().json(Response::success(v)),
        Err(e) => {
            error!("{}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
