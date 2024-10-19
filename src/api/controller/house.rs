use crate::security::Claims;
use crate::utils;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use deadpool_postgres::Pool;
use log::error;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct AddHouse {
    house_name: String,
}
pub async fn add_house(
    body: web::Json<AddHouse>,
    db: web::Data<Pool>,
    req: HttpRequest,
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
            return HttpResponse::InternalServerError()
                .json(utils::Result::error("extractor error"))
        }
    };

    let account_id: i32 = claims.id();

    match client
        .execute(
            "INSERT INTO house\
     (house_name, created_by)\
      VALUES($1, $2)",
            &[&body.house_name, &account_id],
        )
        .await
    {
        Ok(_) => HttpResponse::Ok().json(utils::Result::success()),
        Err(e) => {
            error!("{e}");
            return HttpResponse::InternalServerError()
                .json(utils::Result::error("database error"));
        }
    }
}
