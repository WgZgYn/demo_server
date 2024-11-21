use crate::db::DataBase;
use crate::dto::http::request::SceneAdd;
use crate::utils;
use actix_web::{web, HttpRequest, HttpResponse};
use log::error;

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

pub async fn delete_scene() -> HttpResponse {
    // TODO:
    HttpResponse::NotFound().finish()
}
