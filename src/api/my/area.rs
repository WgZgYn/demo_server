use crate::api::auth::Claims;
use crate::template::template::claims_with_json_template;
use crate::db::area::add_area;
use crate::utils;
use actix_web::{web, HttpRequest, HttpResponse};
use deadpool_postgres::{Object, Pool};
use log::error;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct AddArea {
    area_name: String,
    house_id: i32,
}

pub async fn add_area_api(
    body: web::Json<AddArea>,
    req: HttpRequest,
    pool: web::Data<Pool>,
) -> HttpResponse {
    claims_with_json_template(
        body,
        pool,
        req,
        Box::new(|body, claims, client| Box::pin(add(body, claims, client))),
    )
    .await
}

async fn add(body: AddArea, claims: Claims, client: Object) -> HttpResponse {
    match add_area(client, &body.area_name, body.house_id, claims.id()).await {
        Ok(_) => HttpResponse::Ok().json(utils::Result::success()),
        Err(e) => {
            error!("Failed to insert area: {:?}", e);
            return HttpResponse::InternalServerError()
                .json(utils::Result::error("database connection error"));
        }
    }
}
