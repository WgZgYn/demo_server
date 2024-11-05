use crate::api::auth::Claims;
use crate::db::area::{add_area, query_area};
use crate::template::template::claims_with_data_template;
use crate::utils;
use crate::utils::Response;
use actix_web::{web, HttpRequest, HttpResponse};
use deadpool_postgres::{Object, Pool};
use log::error;
use serde::{Deserialize, Serialize};

pub async fn add_area_api(
    body: web::Json<AddArea>,
    req: HttpRequest,
    pool: web::Data<Pool>,
) -> HttpResponse {
    claims_with_data_template(
        body,
        pool,
        req,
        Box::new(|body, claims, client| Box::pin(add(body.into_inner(), claims, client))),
    )
    .await
}

pub async fn query_area_api(
    pool: web::Data<Pool>,
    body: web::Json<QueryArea>,
    req: HttpRequest,
) -> HttpResponse {
    claims_with_data_template(
        body,
        pool,
        req,
        Box::new(|body, claims, client| Box::pin(query(body.into_inner(), claims, client))),
    )
    .await
}

// TODO:
pub async fn update_area_api(
    pool: web::Data<Pool>,
    body: web::Json<QueryArea>,
    req: HttpRequest,
) -> HttpResponse {
    HttpResponse::Ok().finish()
}

// TODO:
pub async fn delete_area_api(
    pool: web::Data<Pool>,
    body: web::Json<AreaId>,
    req: HttpRequest,
) -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn add(body: AddArea, claims: Claims, client: Object) -> HttpResponse {
    match add_area(client, &body.area_name, body.house_id, claims.id()).await {
        Ok(_) => HttpResponse::Ok().json(utils::Result::success()),
        Err(e) => {
            error!("Failed to insert area: {:?}", e);
            HttpResponse::InternalServerError()
                .json(utils::Result::error("database connection error"))
        }
    }
}

async fn query(body: QueryArea, _claims: Claims, client: Object) -> HttpResponse {
    match query_area(client, body.house_id).await {
        Ok(rows) => {
            let mut data = Vec::new();
            for row in rows {
                data.push(QueryAreaResponse::new(
                    row.get("area_name"),
                    row.get("area_id"),
                ));
            }
            HttpResponse::Ok().json(Response::success(data))
        }
        Err(e) => {
            error!("Failed to query area: {:?}", e);
            HttpResponse::InternalServerError()
                .json(utils::Result::error("database connection error"))
        }
    }
}

async fn delete() {}

async fn update() {}

#[derive(Deserialize, Serialize, Debug)]
pub struct AddArea {
    area_name: String,
    house_id: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct QueryArea {
    house_id: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct QueryAreaResponse {
    area_name: String,
    area_id: i32,
}
impl QueryAreaResponse {
    pub fn new(area_name: String, area_id: i32) -> Self {
        Self { area_name, area_id }
    }
}

#[derive(Deserialize, Serialize, Debug)]
struct UpdateArea {
    area_id: i32,
    area_name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AreaId(i32);
