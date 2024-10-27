use crate::api::auth::Claims;
use crate::template::template::claims_with_json_template;
use crate::db::house::add_house;
use crate::utils;
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
    claims_with_json_template(
        body,
        pool,
        req,
        Box::new(|body, claims, client| Box::pin(add(body, claims, client))),
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
