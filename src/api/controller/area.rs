use crate::security::Claims;
use crate::utils;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use deadpool_postgres::Pool;
use log::error;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct AddArea {
    area_name: String,
    house_id: i32,
}

pub async fn add_area(
    body: web::Json<AddArea>,
    req: HttpRequest,
    pool: web::Data<Pool>,
) -> HttpResponse {
    let client = match pool.get().await {
        Ok(c) => c,
        Err(e) => {
            error!("Failed to get connection from pool: {:?}", e);
            return HttpResponse::InternalServerError()
                .json(utils::Result::error("database connection error"));
        }
    };

    let e = req.extensions();
    let claims = match e.get::<Claims>() {
        Some(claims) => claims,
        None => {
            return HttpResponse::Unauthorized()
                .json(utils::Result::error("add area should login account first"))
        }
    };

    let account_id: i32 = claims.id();

    match client
        .execute(
            "INSERT INTO area (area_name, house_id, created_vy) VALUES ($1, $2)",
            &[&body.area_name, &body.house_id, &account_id],
        )
        .await
    {
        Ok(_) => HttpResponse::InternalServerError()
            .json(utils::Result::error("database connection error")),
        Err(e) => {
            error!("Failed to insert area: {:?}", e);
            HttpResponse::InternalServerError()
                .json(utils::Result::error("database connection error"))
        }
    }
}
