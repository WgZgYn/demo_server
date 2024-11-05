use crate::api::auth::Claims;
use crate::db::house::{add_house, query_house};
use crate::template::template::{claims_template, claims_with_data_template};
use crate::utils;
use crate::utils::Response;
use actix_web::{web, HttpRequest, HttpResponse};
use deadpool_postgres::{Object, Pool};
use log::error;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct AddHouse {
    house_name: String,
}
pub async fn add_house_api(
    body: web::Json<AddHouse>,
    pool: web::Data<Pool>,
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

async fn add(body: AddHouse, claims: Claims, client: Object) -> HttpResponse {
    match add_house(client, &body.house_name, claims.id()).await {
        Ok(_) => HttpResponse::Ok().json(utils::Result::success()),
        Err(e) => {
            error!("{e}");
            HttpResponse::InternalServerError().json(utils::Result::error("database error"))
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
struct QueryHouseResponse {
    house_name: String,
    house_id: i32,
}
impl QueryHouseResponse {
    fn new(house_name: String, house_id: i32) -> QueryHouseResponse {
        QueryHouseResponse {
            house_name,
            house_id,
        }
    }
}

pub async fn query_house_api(pool: web::Data<Pool>, req: HttpRequest) -> HttpResponse {
    claims_template(
        pool,
        req,
        Box::new(|claims, client| Box::pin(query(claims, client))),
    )
    .await
}

// TODO:
pub async fn update_house_api() -> HttpResponse {
    HttpResponse::Ok().finish()
}

// TODO:
pub async fn delete_house_api(pool: web::Data<Pool>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn query(claims: Claims, client: Object) -> HttpResponse {
    match query_house(client, claims.id()).await {
        Ok(rows) => {
            let mut data = Vec::new();
            for row in rows {
                data.push(QueryHouseResponse::new(
                    row.get("house_name"),
                    row.get("house_id"),
                ));
            }
            HttpResponse::Ok().json(Response::success(data))
        }
        Err(e) => {
            error!("{e}");
            HttpResponse::InternalServerError().json(utils::Result::error("database error"))
        }
    }
}

async fn delete() {}

async fn update() {}
