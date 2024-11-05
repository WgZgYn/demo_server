use crate::api::auth::Claims;
use crate::utils;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use deadpool_postgres::{Object, Pool};
use log::error;
use std::future::Future;
use std::pin::Pin;

type AsyncFn<T> =
    dyn FnOnce(T, Claims, Object) -> Pin<Box<dyn Future<Output = HttpResponse> + Send>> + Send;

pub async fn claims_with_data_template<T>(
    body: T,
    db: web::Data<Pool>,
    req: HttpRequest,
    f: Box<AsyncFn<T>>,
) -> HttpResponse {
    let client = match db.get().await {
        Ok(conn) => conn,
        Err(e) => {
            error!("{e}");
            return HttpResponse::InternalServerError()
                .json(utils::Result::error("database error"));
        }
    };

    let e = req.extensions();
    let claims = match e.get::<Claims>() {
        Some(claims) => claims,
        None => {
            return HttpResponse::InternalServerError().json(utils::Result::error("claims error"))
        }
    };

    f(body, claims.clone(), client).await
}

type AsyncFn2 =
    dyn FnOnce(Claims, Object) -> Pin<Box<dyn Future<Output = HttpResponse> + Send>> + Send;
pub async fn claims_template(
    db: web::Data<Pool>,
    req: HttpRequest,
    f: Box<AsyncFn2>,
) -> HttpResponse {
    let client = match db.get().await {
        Ok(conn) => conn,
        Err(e) => {
            error!("{e}");
            return HttpResponse::InternalServerError()
                .json(utils::Result::error("database error"));
        }
    };

    let e = req.extensions();
    let claims = match e.get::<Claims>() {
        Some(claims) => claims,
        None => {
            return HttpResponse::InternalServerError().json(utils::Result::error("claims error"))
        }
    };

    f(claims.clone(), client).await
}
