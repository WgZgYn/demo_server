use crate::db::DataBase;
use crate::dto::http::request::{MemberAdd, MemberDelete};
use crate::security::auth::Claims;
use crate::utils::Response;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use log::error;

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