use crate::api::auth::Claims;
use crate::db::user_info::{add_user_info, query_user_info};
use crate::template::template::{claims_template, claims_with_data_template};
use crate::utils;
use crate::utils::Response;
use actix_web::{web, HttpRequest, HttpResponse};
use deadpool_postgres::{Object, Pool};
use log::error;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct UserInfo {
    real_name: Option<String>,
    gender: Option<String>,
    location: Option<String>,
    age: Option<i32>,
    identity: Option<String>,
    email: Option<String>,
}

pub async fn add_user_info_api(
    pool: web::Data<Pool>,
    body: web::Json<UserInfo>,
    req: HttpRequest,
) -> HttpResponse {
    claims_with_data_template(
        body,
        pool,
        req,
        Box::new(|body, claims, client| Box::pin(add(body.into_inner(), claims, client))),
    )
    .await
}

pub async fn query_user_info_api(pool: web::Data<Pool>, req: HttpRequest) -> HttpResponse {
    claims_template(
        pool,
        req,
        Box::new(|claims, client| Box::pin(query(claims, client))),
    )
    .await
}

async fn add(body: UserInfo, claims: Claims, client: Object) -> HttpResponse {
    match add_user_info(
        client,
        claims.id(),
        body.real_name,
        body.gender,
        body.location,
        body.age,
        body.identity,
    )
    .await
    {
        Ok(_) => HttpResponse::Ok().json(utils::Result::success()),
        Err(e) => {
            error!("{e}");
            HttpResponse::InternalServerError().json(utils::Result::error("database error"))
        }
    }
}

async fn query(claims: Claims, client: Object) -> HttpResponse {
    match query_user_info(client, claims.id()).await {
        Ok(row) => HttpResponse::Ok().json(Response::success(UserInfo {
            real_name: row.get("real_name"),
            gender: row.get("gender"),
            location: row.get("location"),
            age: row.get("age"),
            identity: row.get("identity"),
            email: row.get("email"),
        })),
        Err(e) => {
            error!("{e}");
            HttpResponse::InternalServerError().json(utils::Result::error("query error"))
        }
    }
}
