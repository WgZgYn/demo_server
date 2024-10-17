use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use deadpool_postgres::{Manager, Object, Pool};
use log::error;
use serde::{Deserialize, Serialize};
use crate::security::Claims;
use crate::utils;
pub async fn with_claims<T, F>(body: web::Json<T>, db: web::Data<Pool>, req: HttpRequest, sql: &str, f: F) -> HttpResponse
where
    T: Serialize,
    F: FnOnce(&T, &Claims, Object) -> HttpResponse,
{
    let client = match db.get().await {
        Ok(conn) => conn,
        Err(e) => {
            error!("{e}");
            return HttpResponse::InternalServerError().json(utils::Result::error("database error"));
        }
    };

    let e = req.extensions();
    let claims = match e.get::<Claims>() {
        Some(claims) => claims,
        None => return HttpResponse::InternalServerError().json(utils::Result::error("claims error"))
    };

    f(&*body, claims, client)
}