use actix_web::{web, HttpResponse};
use log::info;
use serde_json::json;
use crate::db::DB;
use crate::dto::account::{AccountAction, AccountQuery};

pub async fn post_account(data: web::Data<DB>, msg: web::Json<AccountQuery>) -> HttpResponse {
    info!("{:?}", msg.0);
    let id = msg.account.username.clone();
    match msg.action {
        AccountAction::Create => match data.users.write() {
            Ok(mut users) => {
                users.insert(id, msg.account.clone());
            }
            Err(err) => {
                info!("{:#?}", err);
                return HttpResponse::InternalServerError().finish();
            }
        },
        AccountAction::Update => {}
        AccountAction::Delete => {}
    }
    HttpResponse::Ok().json(json!(
    {
        "status": "ok",
        "action": msg.action,
    }))
}