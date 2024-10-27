use crate::api::auth::Claims;
use actix_web::{HttpMessage, HttpRequest, HttpResponse};
use serde_json::json;

pub async fn get_auth_info(req: HttpRequest) -> HttpResponse {
    let e = req.extensions();
    let claims = e.get::<Claims>().unwrap();

    HttpResponse::Ok().json(json!(
        {
            "id": claims.sub(),
            "role": claims.role(),
            "status": "ok"
        }
    ))
}
